use std::env;

use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

/// ## Initialize the MySQL database
/// 
/// return the database pool with MySQL 
pub async fn init() -> Pool<MySql> {
    let database_url: String = env::var("DATABASE_URL").unwrap();
    MySqlPoolOptions::new().connect(&database_url).await.unwrap()
}
