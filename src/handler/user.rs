use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::AppState;

pub async fn login_handler() -> impl IntoResponse {
    todo!()
}

#[derive(Debug, Deserialize)]
pub struct CreateReq {
    username: String,
    email: String,
    password: String,
}

pub async fn create(
    State(AppState { ref dbp }): State<AppState>,
    Json(payload): Json<CreateReq>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"INSERT INTO `users` ( `username`, `email`, `password`, `created_at`, `updated_at` ) VALUES ( ?, ?, ?, NOW(), NOW() )"#,
        &payload.username,
        &payload.email,
        &payload.password
    )
        .execute(dbp)
        .await
        .unwrap();

    result.last_insert_id();

    "created ok"
}
