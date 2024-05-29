use std::env;

use sqlx::{mysql::MySqlPoolOptions, types::chrono, MySql, Pool};

use crate::config;

/// ## Initialize the MySQL database
///
/// return the database pool with MySQL
pub async fn init() -> Pool<MySql> {
    let database_url = &config::ENV.database.0;
    MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap()
}

/// ## User model
///
/// Map to user table
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub avatar_url: String,
    pub email: String,
    pub status: i8,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Local>,
    pub updated_at: chrono::DateTime<chrono::Local>,
}
