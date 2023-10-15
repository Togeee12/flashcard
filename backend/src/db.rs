use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::{models, db};

pub async fn establish_connection() -> Result<MysqlConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    MysqlConnection::establish(&database_url)
}

pub async fn pseudo_db_call() -> models::FlashCard {
    models::FlashCard::new(
        2137, 
        "O której godzinie umarł papaj".to_owned(), 
        "21:37".to_owned(),
    )
}