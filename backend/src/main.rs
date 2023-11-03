#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use env_logger;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use argon2::Argon2;

mod auth;
mod db;
mod models;
mod schema;
mod routes;
mod tests;
mod utils;
mod api;

#[get("/")]
async fn root_get(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("Hello world!")
}

// use actix_files::NamedFile;
// use std::io;
// async fn demo() -> Result<NamedFile, io::Error> {
//     Ok(NamedFile::open("demo.html")?)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "backend=trace, actix_web=debug, actix_server=info");
    env_logger::init();

    let app_data = models::AppData {
        pool: db::establish_connection(),
        jwt_secret: dotenv::var("JWT_SECRET").expect("JWT_SECRET not set in .env file"),
        jwt_duration: dotenv::var("JWT_DURATION").expect("JWT_DURATION not set in .env file")
            .parse::<u64>().expect("Ivalid JWT_DURATION value, it should be the duration in seconds."),
        domain: dotenv::var("DOMAIN").expect("DOMAIN not set in .env file (example.com)"),
        argon2: Argon2::default(),
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            // .route("/demo", web::get().to(demo))
            .service(root_get)
            .service(web::scope("/api/v1").configure(api::api_v1))
            .default_service(web::to(routes::http_404))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
