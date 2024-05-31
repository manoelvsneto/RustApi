use sqlx::mssql::MssqlPool;
use sqlx::Pool;

pub async fn create_pool(database_url: &str) -> Result<Pool<Mssql>, sqlx::Error> {
    let pool = MssqlPool::connect(database_url).await?;
    Ok(pool)
}
