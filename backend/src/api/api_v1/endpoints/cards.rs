use super::{generate_err_response, api_models::{self, ResponseError}, wrapped, api_models::ConvertCardsFromOptional};
use crate::{utils, db, models};

use actix_web::{web, HttpRequest, HttpResponse};

macro_rules! impl_try_from {
    ($struct_name:ident { $($field:ident : $field_type:ty,)* }) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field: $field_type),*
        }

        impl api_models::ConvertCardsFromOptional for $struct_name {
            fn try_from_optional(optional: &api_models::CardsRequestContent) -> Result<Self, &'static str> {
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
pub async fn cards_handler<'a>(
    app_data: web::Data<models::AppData<'a>>,
    req: HttpRequest,
    payload: String,
) -> HttpResponse {
    let execute = move || -> Result<HttpResponse, ResponseError> {
        // Validate encoding
        wrapped::is_utf8(&payload)?;
        // Parse JSON
        let request_data = wrapped::parse_json::<api_models::CardsRequest>(&payload)?;
        // Get db connection
        let mut conn: db::Conn = wrapped::get_db_conn(&app_data.pool)?;

        
        let content = match request_data.content {
            Some(data) => data,
            None => api_models::CardsRequestContent::empty(),
        };

        match request_data.request_type {
            api_models::CardsRequestType::GetStacksByOwnerId => {
                impl_try_from!( ParsedStacksData {
                    unique_id: String,
                });

                let unique_id = match ParsedStacksData::try_from_optional(&content) {
                    Ok(value) => value.unique_id,
                    _ => return Err(ResponseError::InvalidData),
                };

                let authorized = {
                    if let Ok(user_id) = wrapped::authenticate(&req, &app_data.jwt_secret) {
                        user_id == unique_id
                    } else { false }
                };

                let mut stacks = match db::get_stacks_by_owner(&mut conn, &unique_id) {
                    Ok(value) => value,
                    Err(_) => return Err(ResponseError::InternalError)
                };

                if ! authorized {
                    stacks.retain(|elem| elem.visibility);
                }

                let mut response_struct = api_models::Response::new();
                response_struct.set_stacks(db_stacks_to_resp_stacks(stacks));
                Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()))
            }


            api_models::CardsRequestType::GetStackById => {
                impl_try_from!( ParsedStacksData {
                    unique_id: String,
                });

                let unique_id = match ParsedStacksData::try_from_optional(&content) {
                    Ok(value) => value.unique_id,
                    _ => return Err(ResponseError::InvalidData),
                };

                let stacks: Vec<api_models::StackData> = match db::get_stack(&mut conn, &unique_id) {
                    Ok(stack) => {
                        let authorized = {
                            if let Ok(user_id) = wrapped::authenticate(&req, &app_data.jwt_secret) {
                                user_id == stack.owner_id
                            } else { false }
                        };
                        if stack.visibility || authorized {
                            db_stacks_to_resp_stacks(vec![stack])
                        } else {
                            Vec::new()
                        }
                    },
                    _ => Vec::new()
                };

                let mut response_struct = api_models::Response::new();
                response_struct.set_stacks(stacks);
                Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()))
            }


            api_models::CardsRequestType::GetCardsByStackId => {
                impl_try_from!( ParsedCardData {
                    unique_id: String,
                });

                let unique_id = match ParsedCardData::try_from_optional(&content) {
                    Ok(value) => value.unique_id,
                    _ => return Err(ResponseError::InvalidData),
                };

                // So i dont repeat myself
                let no_stacks_found = || -> Result<HttpResponse, ResponseError> {
                    let mut response_struct = api_models::Response::new();
                    response_struct.set_stacks(vec![]);
                    Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()))
                };

                // We first need to check if that stack exists and if it is public
                match db::get_stack(&mut conn, &unique_id) {
                    Ok(stack) => {
                        let authorized = {
                            if let Ok(user_id) = wrapped::authenticate(&req, &app_data.jwt_secret) {
                                user_id == stack.owner_id
                            } else { false }
                        };
                        if ! stack.visibility && ! authorized {
                            // Stack is private so send a response as if no cards were found
                            return no_stacks_found();
                        }
                    }
                    Err(diesel::result::Error::NotFound) => return no_stacks_found(),
                    _ => return Err(ResponseError::InternalError)
                }

                let cards = match db::get_cards_by_stack(&mut conn, &unique_id) {
                    Ok(value) => db_cards_to_resp_cards(value),
                    _ => return Err(ResponseError::InternalError)
                };

                let mut response_struct = api_models::Response::new();
                response_struct.set_cards(cards);
                Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()))
            }


            api_models::CardsRequestType::GetCardById => {
                impl_try_from!( ParsedCardData {
                    unique_id: String,
                });

                let unique_id = match ParsedCardData::try_from_optional(&content) {
                    Ok(value) => value.unique_id,
                    _ => return Err(ResponseError::InvalidData),
                };

                // So i dont repeat myself
                let no_cards_found = || -> Result<HttpResponse, ResponseError> {
                    let mut response_struct = api_models::Response::new();
                    response_struct.set_cards(vec![]);
                    Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()))
                };

                let card = match db::get_card(&mut conn, &unique_id) {
                    Ok(value) => value,
                    Err(diesel::result::Error::NotFound) => return no_cards_found(),
                    _ => return Err(ResponseError::InternalError)
                };

                // Check if the stack that this card belongs to is private
                match db::get_stack(&mut conn, &card.stack_id) {
                    Ok(stack) => {
                        let authorized = {
                            if let Ok(user_id) = wrapped::authenticate(&req, &app_data.jwt_secret) {
                                user_id == stack.owner_id
                            } else { false }
                        };
                        if ! stack.visibility && ! authorized {
                            // Stack is private so send a response as if no card was found
                            return no_cards_found();
                        }
                    }
                    Err(diesel::result::Error::NotFound) => return no_cards_found(),
                    _ => return Err(ResponseError::InternalError)
                }

                let mut response_struct = api_models::Response::new();
                response_struct.set_cards(vec![card.into()]);
                Ok(HttpResponse::Ok().content_type("application/json").body(response_struct.to_string()))
            }


            api_models::CardsRequestType::CreateStack => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                impl_try_from!( ParsedStacksData {
                    name: String,
                    tags: String,
                    visibility: bool,
                });

                let mut stack_data = match ParsedStacksData::try_from_optional(&content) {
                    Ok(value) => value,
                    _ => return Err(ResponseError::InvalidData),
                };

                // Validate stack name
                if ! utils::is_valid_stack_name(&stack_data.name) {
                    return Err(ResponseError::InvalidData)
                }

                // Validate and format tags
                stack_data.tags = utils::parse_tags(&stack_data.tags);
                if ! utils::is_valid_tags(&stack_data.tags) {
                    return Err(ResponseError::InvalidData)
                }

                let stack_id = match db::generate_stack_id(&mut conn) {
                    Ok(value) => value,
                    _ => return Err(ResponseError::InternalError),
                };

                let new_stack_data = models::Stack {
                    unique_id: stack_id,
                    owner_id: user_id,
                    name: stack_data.name,
                    tags: stack_data.tags,
                    visibility: stack_data.visibility,
                };

                if db::add_stack(&mut conn, new_stack_data).is_err() {
                    return Err(ResponseError::InternalError);
                }

                Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()))
            }


            api_models::CardsRequestType::CreateCard => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                impl_try_from!( ParsedCardData {
                    stack_id: String,
                    frontside: String,
                    backside: String,
                });

                let card_data = match ParsedCardData::try_from_optional(&content) {
                    Ok(value) => value,
                    _ => return Err(ResponseError::InvalidData),
                };

                // Validate user
                match db::get_stack(&mut conn, &card_data.stack_id) {
                    Ok(value) => {
                        if value.owner_id != user_id {
                            return Err(ResponseError::Unauthorized)
                        }
                    }
                    Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                    _ => return Err(ResponseError::InternalError)
                }

                // Validate input data
                if card_data.frontside.len() > 255 || card_data.backside.len() > 255 {
                    return Err(ResponseError::InvalidData)
                }

                let card_id = match db::generate_card_id(&mut conn) {
                    Ok(value) => value,
                    _ => return Err(ResponseError::InternalError),
                };

                let new_card_data = models::Card {
                    unique_id: card_id,
                    stack_id: card_data.stack_id,
                    frontside: card_data.frontside,
                    backside: card_data.backside,
                };

                if db::add_card(&mut conn, new_card_data).is_err() {
                    return Err(ResponseError::InternalError);
                }

                Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()))
            }


            api_models::CardsRequestType::UpdateStack => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                // Filter out unwanted fields
                match (&content.stack_id, &content.frontside, &content.backside) {
                    (Some(_), _, _) | (_, Some(_), _) | (_, _, Some(_)) => return Err(ResponseError::InvalidData),
                    _ => {}
                }

                let mut stack_data = match content.unique_id {
                    Some(id) => {
                        match db::get_stack(&mut conn, &id) {
                            Ok(value) => value,
                            Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                            _ => return Err(ResponseError::InternalError)
                        }
                    }
                    None => return Err(ResponseError::InvalidData),
                };

                // Validate user
                if stack_data.owner_id != user_id {
                    return Err(ResponseError::Unauthorized)
                }

                // Udate values
                if let Some(value) = content.name {
                    if ! utils::is_valid_stack_name(&value) {
                        return Err(ResponseError::InvalidData)
                    }
                    stack_data.name = value
                }

                if let Some(value) = content.tags {
                    let parsed_tags = utils::parse_tags(&value);
                    if ! utils::is_valid_tags(&parsed_tags) {
                        return Err(ResponseError::InvalidData)
                    }
                    stack_data.tags = parsed_tags
                }

                if let Some(value) = content.visibility {
                    stack_data.visibility = value
                }

                // Send the update call
                if db::update_stack(&mut conn, stack_data.into()).is_err() {
                    return Err(ResponseError::InternalError);
                }

                Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()))
            }


            api_models::CardsRequestType::UpdateCard => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                // Filter out unwanted fields
                match (&content.stack_id, &content.name, &content.tags, &content.visibility) {
                    (Some(_), _, _, _) | (_, Some(_), _, _) | (_, _, Some(_), _) | (_, _, _, Some(_)) => return Err(ResponseError::InvalidData),
                    _ => {}
                }

                let mut card_data = match content.unique_id {
                    Some(id) => {
                        match db::get_card(&mut conn, &id) {
                            Ok(value) => value,
                            Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                            _ => return Err(ResponseError::InternalError)
                        }
                    }
                    None => return Err(ResponseError::InvalidData),
                };

                // Validate user
                match db::get_stack(&mut conn, &card_data.stack_id) {
                    Ok(value) => {
                        if value.owner_id != user_id {
                            return Err(ResponseError::Unauthorized)
                        }
                    }
                    Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                    _ => return Err(ResponseError::InternalError),
                }

                // Udate values
                if let Some(value) = content.frontside {
                    card_data.frontside = value
                }

                if let Some(value) = content.backside {
                    card_data.backside = value
                }

                // Validate new data
                if card_data.frontside.len() > 255 || card_data.backside.len() > 255 {
                    return Err(ResponseError::InvalidData)
                }

                // Send the update call
                if db::update_card(&mut conn, card_data).is_err() {
                    return Err(ResponseError::InternalError);
                }

                Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()))
            }


            api_models::CardsRequestType::DeleteStack => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                impl_try_from!( ParsedStacksData {
                    unique_id: String,
                });

                let stack_id = match ParsedStacksData::try_from_optional(&content) {
                    Ok(value) => value.unique_id,
                    _ => return Err(ResponseError::InvalidData),
                };

                let stack_data = match db::get_stack(&mut conn, &stack_id) {
                    Ok(value) => value,
                    Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                    _ => return Err(ResponseError::InternalError),
                };

                if stack_data.owner_id != user_id {
                    return Err(ResponseError::Unauthorized)
                }

                // Send the delete call
                if db::delete_stack(&mut conn, &stack_id).is_err() {
                    return Err(ResponseError::InternalError);
                }

                Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()))
            }


            api_models::CardsRequestType::DeleteCard => {
                let user_id = wrapped::authenticate(&req, &app_data.jwt_secret)?;

                impl_try_from!( ParsedCardData {
                    unique_id: String,
                });

                let card_id = match ParsedCardData::try_from_optional(&content) {
                    Ok(value) => value.unique_id,
                    _ => return Err(ResponseError::InvalidData),
                };

                let stack_id = match db::get_card(&mut conn, &card_id) {
                    Ok(value) => value.stack_id,
                    Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                    _ => return Err(ResponseError::InternalError),
                };

                let owner_id = match db::get_stack(&mut conn, &stack_id) {
                    Ok(value) => value.owner_id,
                    Err(diesel::result::Error::NotFound) => return Err(ResponseError::Unauthorized),
                    _ => return Err(ResponseError::InternalError),
                };

                if owner_id != user_id {
                    return Err(ResponseError::Unauthorized)
                }

                // Send the delete call
                if db::delete_card(&mut conn, &card_id).is_err() {
                    return Err(ResponseError::InternalError);
                }

                Ok(HttpResponse::Ok().content_type("application/json").body(api_models::Response::empty_ok().to_string()))
            }
        }
    };

    match execute() {
        Ok(resp) => resp,
        Err(code) => generate_err_response(code)
    }
}



fn db_stacks_to_resp_stacks(db_stacks: Vec<models::StackFull>) -> Vec<api_models::StackData> {
    let mut output: Vec<api_models::StackData> = Vec::with_capacity(db_stacks.len());
    for item in db_stacks {
        output.push(item.into());
    }
    output
}

fn db_cards_to_resp_cards(db_cards: Vec<models::Card>) -> Vec<api_models::CardData> {
    let mut output: Vec<api_models::CardData> = Vec::with_capacity(db_cards.len());
    for item in db_cards {
        output.push(item.into());
    }
    output
}