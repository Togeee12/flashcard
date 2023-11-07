use super::{generate_err_response, api_models::{self, ResponseError}, wrapped, api_models::ConvertUserFromOptional};
use crate::{utils, db, auth, models};

use diesel::result::DatabaseErrorKind;
use actix_web::{web, HttpRequest, HttpResponse};

macro_rules! impl_try_from {
    ($struct_name:ident { $($field:ident : $field_type:ty,)* }) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field: $field_type),*
        }

        impl api_models::ConvertUserFromOptional for $struct_name {
            fn try_from_optional(optional: &api_models::UserRequestContent) -> Result<Self, &'static str> {
                $(
                    let $field = optional.$field.clone().ok_or(concat!(stringify!($field), " is required"))?;
                )*

                Ok($struct_name {
                    $($field),*
                })
            }
        }
    };
}

#[allow(clippy::needless_lifetimes)] // False positive
pub async fn users_handler<'a>(
    app_data: web::Data<models::AppData<'a>>,
    req: HttpRequest,
    payload: String,
) -> HttpResponse {
    let execute = move || -> Result<HttpResponse, ResponseError> {
        // Validate encoding
        wrapped::is_utf8(&payload)?;
        // Parse JSON
        let request_data = wrapped::parse_json::<api_models::UsersRequest>(&payload)?;
        // Get db connection
        let mut conn: db::Conn = wrapped::get_db_conn(&app_data.pool)?;


        let content = match request_data.content {
            Some(data) => data,
            None => api_models::UserRequestContent::empty(),
        };

        match request_data.request_type {
            api_models::UsersRequestType::GetMyProfile => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;
                
                // Check if user exists
                let user_exists = match db::check_if_user_exists(&mut conn, &user_id) {
                    Ok(value) => value,
                    _ => return Err(ResponseError::InternalError)
                };

                if ! user_exists {
                    return Err(ResponseError::CouldntAuthenticate);
                }

                // Fetch user data
                let user_data = match db::get_user(&mut conn, &user_id) {
                    Ok(value) => value,
                    _ => return Err(ResponseError::InternalError)
                };

                // Respond with user data
                let response_user = api_models::UserData::from(user_data, true);
                let mut response_struct = api_models::Response::new();
                response_struct.set_user(response_user);
                return Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()));
            }


            
            api_models::UsersRequestType::GetUser => {
                // Try by unique_id
                impl_try_from!( ParsedUserDataById { unique_id: String, });
                if let Ok(pasrsed_user_data) = ParsedUserDataById::try_from_optional(&content) {
                    if let Ok(user) = db::get_user(&mut conn, &pasrsed_user_data.unique_id) {
                        let response_user = api_models::UserData::from(user, false);
                        let mut response_struct = api_models::Response::new();
                        response_struct.set_user(response_user);
                        return Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()));
                    }
                }

                // Try by username
                impl_try_from!( ParsedUserDataByUsername { username: String, });
                if let Ok(pasrsed_user_data) = ParsedUserDataByUsername::try_from_optional(&content) {
                    if let Ok(user) = db::get_user_by_username(&mut conn, &pasrsed_user_data.username) {
                        let response_user = api_models::UserData::from(user, false);
                        let mut response_struct = api_models::Response::new();
                        response_struct.set_user(response_user);
                        return Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()));
                    }
                }

                // Not found
                let response_struct = api_models::Response::empty_ok();
                return Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()));
            }



            api_models::UsersRequestType::CreateUser => {
                impl_try_from!( ParsedUserData {
                    email: String,
                    username: String,
                    password: String,
                    country: String,
                });

                // Validate input data
                let user_data = if let Ok(pasrsed_user_data) = ParsedUserData::try_from_optional(&content) {
                    if ! utils::is_valid_email(&pasrsed_user_data.email) ||
                        ! utils::is_valid_password(&pasrsed_user_data.password) ||
                        ! utils::is_valid_username(&pasrsed_user_data.username) ||
                        ! utils::is_valid_country_code(&pasrsed_user_data.country) {
                        return Err(ResponseError::InvalidData);
                    }
                    pasrsed_user_data
                } else {
                    return Err(ResponseError::InvalidData);
                };

                // Get db connection
                let mut conn = match app_data.pool.get() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(ResponseError::InternalError);
                    }
                };

                // Send db request for new user id
                let new_user_id = if let Ok(new_id) = db::generate_user_id(&mut conn) {
                    new_id
                } else {
                    return Err(ResponseError::InternalError);
                };

                // Create user data
                let user = models::User {
                    unique_id: new_user_id,
                    username: user_data.username,
                    email: user_data.email,
                    password_hash: auth::hash_password(&app_data.argon2, &user_data.password),
                    date_of_registration: utils::get_unix_timestamp() as i64,
                    country: user_data.country
                };

                match db::add_user(&mut conn, user) {
                    Ok(_) => {
                        let response_struct = api_models::Response::empty_ok();
                        return Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()));
                    }
                    Err(diesel::result::Error::DatabaseError(kind, _)) => {
                        if let DatabaseErrorKind::UniqueViolation = kind {
                            return Err(ResponseError::EmailOrUSernameUsed);
                        }
                    }
                    Err(_) => {
                        return Err(ResponseError::InternalError);
                    }
                }
            }
            


            api_models::UsersRequestType::UpdateUser => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                // Filter out unwanted fields
                match (&content.unique_id, &content.date_of_registration) {
                    (Some(_), _) | (_, Some(_)) => return Err(ResponseError::InvalidData),
                    _ => {}
                }

                let mut user_data = match db::get_user(&mut conn, &user_id) {
                    Ok(value) => value,
                    _ => {
                        return Err(ResponseError::InternalError);
                    }
                };

                // Validate email
                if let Some(email) = content.email {
                    if ! utils::is_valid_email(&email) {
                        return Err(ResponseError::InvalidData);
                    }
                    user_data.email = email;
                }

                // Validate username
                if let Some(username) = content.username {
                    if ! utils::is_valid_username(&username) {
                        return Err(ResponseError::InvalidData);
                    }
                    user_data.username = username;
                }

                // Validate password
                if let Some(password) = content.password {
                    if ! utils::is_valid_password(&password) {
                        return Err(ResponseError::InvalidData);
                    }
                    user_data.password_hash = auth::hash_password(&app_data.argon2, &password);
                }

                // Validate country
                if let Some(country) = content.country {
                    if ! utils::is_valid_country_code(&country) {
                        return Err(ResponseError::InvalidData);
                    }
                    user_data.country = country;
                }

                match db::update_user(&mut conn, user_data) {
                    Ok(_) => {}
                    Err(diesel::result::Error::DatabaseError(kind, _)) => {
                        if let DatabaseErrorKind::UniqueViolation = kind {
                            return Err(ResponseError::EmailOrUSernameUsed);
                        }
                    }
                    Err(_) => {
                        return Err(ResponseError::InternalError);
                    }
                }

                return Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()));
            }



            api_models::UsersRequestType::DeleteUser => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                impl_try_from!( ParsedUserData {
                    password: String,
                });
                // Validate input data
                let password = if let Ok(pasrsed_user_data) = ParsedUserData::try_from_optional(&content) {
                    if ! utils::is_valid_password(&pasrsed_user_data.password) {
                        return Err(ResponseError::InvalidData);
                    }
                    pasrsed_user_data.password
                } else {
                    return Err(ResponseError::InvalidData);
                };

                let user_data = match db::get_user(&mut conn, &user_id) {
                    Ok(value) => value,
                    _ => {
                        return Err(ResponseError::InternalError);
                    }
                };

                if ! auth::verify_password(&app_data.argon2, &password, &user_data.password_hash) {
                    return Err(ResponseError::CouldntAuthenticate);
                }

                match db::delete_user(&mut conn, &user_id) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(ResponseError::InternalError);
                    }
                }

                return Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()));
            }
        }

        Err(ResponseError::InvalidData)
    };

    match execute() {
        Ok(resp) => resp,
        Err(code) => generate_err_response(code)
    }
}