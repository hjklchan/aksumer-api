use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::error::api_error::{ApiError, OhMyResult};
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

pub async fn create_handler(
    State(AppState { ref dbp }): State<AppState>,
    Json(payload): Json<CreateReq>,
) -> OhMyResult<impl IntoResponse> {
    let new_id = sqlx::query!(
        r#"INSERT INTO `users` ( `username`, `email`, `password`, `created_at`, `updated_at` ) VALUES ( ?, ?, ?, NOW(), NOW() )"#,
        &payload.username,
        &payload.email,
        &payload.password
    )
        .execute(dbp)
        .await
        .map(|result| result.last_insert_id())
        .map_err(|err| ApiError::Sqlx(err))?;

    Ok(Json(serde_json::json!({"new_id": new_id})))
}
