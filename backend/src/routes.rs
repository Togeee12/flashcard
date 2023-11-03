use actix_web::HttpResponse;

pub async fn http_404() -> HttpResponse {
    HttpResponse::Ok().body("404 Not found")
}
