use super::{generate_err_response, api_models::{self, ResponseError}, wrapped};
use crate::{utils, db, auth, models};

use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;

pub async fn auth_handler<'a>(
    app_data: web::Data<models::AppData<'a>>,
    req: HttpRequest,
    payload: String,
) -> HttpResponse {
    let execute = move || -> Result<HttpResponse, ResponseError> {
        wrapped::is_ascii(&payload)?;
        let request_data = wrapped::parse_json::<api_models::AuthRequest>(&payload)?;

        match request_data.request_type {
            api_models::AuthRequestType::Authenticate => {
                let content = match request_data.content {
                    Some(value) => value,
                    None => return Err(ResponseError::InvalidData),
                };

                // Validate input data
                if ! utils::is_valid_email(&content.email) || 
                    ! utils::is_valid_password(&content.password) {
                    return Err(ResponseError::InvalidData);
                }

                // Get db connection
                let mut conn = wrapped::get_db_conn(&app_data.pool)?;

                // Querry db for user
                let user = match db::get_user_by_email(&mut conn, &content.email) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(ResponseError::InvalidEmailOrPw);
                    }
                };

                // Compare passwords
                if ! auth::verify_password(&app_data.argon2, &content.password, &user.password_hash) {
                    return Err(ResponseError::InvalidEmailOrPw);
                }

                // Generate jwt
                let app_data = app_data.get_ref();
                let claims = auth::Claims { 
                    sub: user.unique_id.to_owned(), 
                    exp: utils::get_unix_timestamp() + app_data.jwt_duration
                };
                let jwt = auth::encode_jwt(&app_data.jwt_secret, &claims)
                    .expect("Error creating jwt");
                
                // Set cookie
                let cookie = Cookie::build("jwt_v1", jwt)
                    .domain(app_data.domain.to_owned())
                    .path("/")
                    .secure(false)
                    .http_only(true) 
                    .finish();

                Ok(HttpResponse::Ok().cookie(cookie).body(api_models::Response::empty_ok().set_unique_id(&user.unique_id).to_string()))
            }


            api_models::AuthRequestType::Check => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                Ok(HttpResponse::Ok().body(api_models::Response::empty_ok().set_unique_id(&user_id).to_string()))
            }


            api_models::AuthRequestType::LogOut => {
                let cookie = Cookie::build("jwt_v1", "")
                    .domain(app_data.domain.to_owned())
                    .path("/")
                    .secure(false)
                    .http_only(true) 
                    .finish();

                Ok(HttpResponse::Ok().cookie(cookie).body(api_models::Response::empty_ok().to_string()))
            }
        }

        
    };

    match execute() {
        Ok(resp) => resp,
        Err(code) => generate_err_response(code)
    }
}