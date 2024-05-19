use std::env;

use sqlx::{MySql, Pool, mysql::{MySqlPoolOptions}};


pub async fn init() -> Result<Pool<MySql>, Box<dyn std::error::Error>> {
    let database_url: String = env::var("DATABASE_URL")?;
    Ok(MySqlPoolOptions::new().connect(&database_url).await?)
}