#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use env_logger;

mod tests;
mod db;
mod models;
mod schema;
mod routes;
mod utils;

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::var("RUST_LOG").unwrap_or("actix=info".to_string());
    env_logger::init();

    let pool: db::Pool = db::establish_connection();

    { // DEMO
        let mut conn = pool.get().expect("Failed to get connection from pool");
        
        let new_user_id = db::generate_user_id(&mut conn).expect("Error generating new user ID");
        let new_user = models::User {
            unique_id: new_user_id.clone(),
            email: "john.smith@hotmail.com".to_owned(),
            username: "FlashCardEnjoyer69".to_owned(),
            password_hash: "cac35ec206d868b7d7c".to_owned(),
            date_of_registration: utils::get_unix_timestamp() as i64,
            country: "USA".to_owned(),
        };
        db::add_user(&mut conn, new_user).expect("Failed adding a user record.");

        println!("Added a new user!");
        
        let new_stack_id = db::generate_stack_id(&mut conn).expect("Error generating new user ID");
        let new_stack = models::Stack {
            owner_id: new_user_id.clone(),
            unique_id: new_stack_id.clone(),
            name: "My first flashcards stack".to_owned(),
            visibility: true,
            tags: "test, favourites".to_owned(),
        };
        db::add_stack(&mut conn, new_stack).expect("Failed adding a stack record.");

        println!("Added a new stack!");


        let new_card_id = db::generate_card_id(&mut conn).expect("Error generating new card ID");
        let new_card = models::Card {
            unique_id: new_card_id.to_owned(),
            stack_id: new_stack_id.clone(),
            frontside: "Am i going insane?".to_owned(),
            backside: "Yes".to_owned(),
        };
        db::add_cards(&mut conn, new_card).expect("Failed adding a card record.");

        println!("Added a new card!");

        let new_card_id = db::generate_card_id(&mut conn).expect("Error generating new card ID");
        let new_card = models::Card {
            unique_id: new_card_id.to_owned(),
            stack_id: new_stack_id.clone(),
            frontside: "Do i belong in a mental health ward?".to_owned(),
            backside: "Yes".to_owned(),
        };
        db::add_cards(&mut conn, new_card).expect("Failed adding a card record.");

        println!("Added a new card!");
    }

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(root_get)
            .service(web::scope("/api").configure(routes::api))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
