use actix_web::{web, HttpResponse};

mod api_models;
mod wrapped;
mod endpoints;

pub fn generate_err_response(err_code: api_models::ResponseError) -> HttpResponse {
    let response_struct = api_models::Response::new_err( vec![err_code.parse()] );
    HttpResponse::BadRequest().content_type("application/json").body(response_struct.to_string())
}


pub fn api_v1(cfg: &mut web::ServiceConfig) {
    cfg
        .route("auth", web::post().to(endpoints::auth_handler))
        .route("auth/", web::post().to(endpoints::auth_handler))

        .route("users", web::post().to(endpoints::users_handler))
        .route("users/", web::post().to(endpoints::users_handler))

        .route("cards", web::post().to(endpoints::cards_handler))
        .route("cards/", web::post().to(endpoints::cards_handler));
}