use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use std::env;
use env_logger;
use actix_web::web;


mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::var("RUST_LOG").unwrap_or("actix=info".to_string());
    env_logger::init();

    let pool = db::establish_connection().await.expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone())) // Updated to use app_data
            .service(web::scope("/api").configure(routes::flashcards_config))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
