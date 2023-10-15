use dotenv;
use sqlx::MySqlPool;

pub fn initialize_dotenv() {
    dotenv::from_filename(r"\.env").ok();
}

pub async fn establish_connection() -> Result<MySqlPool, sqlx::Error> {
    // Define your database URL
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL not found in .env file");

    // Create a database connection pool
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    Ok(pool)
}