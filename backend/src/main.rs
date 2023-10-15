use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use env_logger;

use serde_json;


mod db;
mod models;
mod routes;

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::var("RUST_LOG").unwrap_or("actix=info".to_string());
    env_logger::init();

    let _pool = db::establish_connection().await.expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .service(root_get)
            .service(web::scope("/api").configure(routes::api))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
