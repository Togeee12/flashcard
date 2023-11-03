//! ## A collection of functions wrapped to return api_models::ResponseError

use crate::api::api_v1::api_models::ResponseError;
use crate::{utils, db, auth,};
use serde_json;
use actix_web::HttpRequest;

pub fn is_ascii(content: &str) -> Result<(), ResponseError> {
    if ! utils::is_ascii(content) {
        return Err(ResponseError::InvalidData);
    }
    Ok(())
}

pub fn is_utf8(content: &str) -> Result<(), ResponseError> {
    if String::from_utf8(content.as_bytes().to_vec()).is_err() {
        return Err(ResponseError::InvalidData);
    }
    Ok(())
}

pub fn parse_json<T: serde::de::DeserializeOwned>(content: &str) -> Result<T, ResponseError> {
    serde_json::from_str::<T>(&content).map_err(|_| ResponseError::ParsingRequestContent)
}

pub fn get_db_conn(pool: &db::Pool) -> Result<db::Conn, ResponseError> {
    pool.get().map_err(|_| ResponseError::InternalError)
}

pub fn authenticate(req: &HttpRequest, jwt_secret: &str) -> Result<String, ResponseError> {
    let jwt = match req.cookie("jwt_v1") {
        Some(cookie) => cookie.value().to_owned(),
        None => return Err(ResponseError::CouldntAuthenticate)
    };
    
    auth::authorize_jwt::<auth::Claims>(jwt_secret, &jwt)
    .map_err(|_| ResponseError::CouldntAuthenticate)
}