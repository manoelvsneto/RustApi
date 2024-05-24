use sqlx::{Pool, MssqlPool};
use std::env;
use dotenv::dotenv;

pub async fn establish_connection() -> Pool<MssqlPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MssqlPool::connect(&database_url).await.expect("Failed to connect to database")
}
