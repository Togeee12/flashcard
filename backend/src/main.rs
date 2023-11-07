#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use env_logger;
use log::{info, error};
use std::process::exit;

use actix_web::{web, App, HttpServer};
use argon2::Argon2;

mod auth;
mod db;
mod models;
mod schema;
mod routes;
mod tests;
mod utils;
mod api;

#[cfg(feature = "demo")]
async fn demo() -> Result<actix_web::HttpResponse, std::io::Error> {
    Ok(actix_web::HttpResponse::Ok().content_type("text/html").body(DEMO_HTML_CONTENT))
}

#[cfg(feature = "demo")]
const DEMO_HTML_CONTENT: &str = include_str!("./demo.html");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Setting up logger
    std::env::set_var("RUST_LOG", "backend=info, actix_web=info, actix_server=info");
    env_logger::init();

    // Creating necessary app data and reading from .env file
    let pool: db::Pool = {
        let url = dotenv::var("DATABASE_URL").unwrap_or_else(|err| {
            error!("DATABASE_URL not set in .env file.: {}", err);
            exit(1);
        });
        db::establish_connection(url)
    };

    let jwt_secret = dotenv::var("JWT_SECRET").unwrap_or_else(|err| {
        error!("JWT_SECRET not set in .env file.: {}", err);
        exit(1);
    });

    let jwt_duration = dotenv::var("JWT_DURATION").unwrap_or_else(|err| {
        error!("JWT_DURATION not set in .env file.: {}", err);
        exit(1);
    })
    .parse::<u64>().unwrap_or_else(|err| {
        error!("Invalid JWT_DURATION value.: {}", err);
        exit(1);
    });

    let domain = dotenv::var("DOMAIN").unwrap_or_else(|err| {
        error!("DOMAIN not set in .env file. Ex: (example.com): {}", err);
        exit(1);
    });

    let socket = {
        use std::net::ToSocketAddrs;

        let socket_str = dotenv::var("SOCKET").unwrap_or_else(|err| {
            error!("SOCKET not set in .env file. Ex: (127.0.0.1:443): {}", err);
            exit(1);
        });

        match socket_str.to_socket_addrs() {
            Ok(mut iter) => {
                match iter.next() {
                    Some(socket_addr) => {
                        if iter.next().is_none() { Some(socket_addr) } 
                        else { None } // More than one address found
                    }
                    _ => None
                }
            }
            Err(_) => None, // Invalid address
        }.unwrap_or_else(|| {
            error!("Invalid SOCKET value. Ex: (127.0.0.1:443)");
            exit(1);
        })
    };

    let app_data = models::AppData {
        pool,
        jwt_secret,
        jwt_duration,
        domain,
        argon2: Argon2::default(),
    };
    
    // Starting actix (http server) instances
    info!("Binding on socket {}", socket);
    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(web::scope("/api/v1").configure(api::api_v1))
            .default_service(web::to(routes::http_404));

        #[cfg(feature = "demo")] // Add the demo endpoint
        let app = app.route("/demo", web::get().to(demo));

        app
    })
    .bind(socket).unwrap_or_else(|err| {
        error!("Couldn't bind on specified socket.: {}", err);
        exit(1);
    })
    .run()
    .await
}
