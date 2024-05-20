use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::env;

use crate::error::api_error::{ApiError, AuthenticateError, OhMyResult, UserError};
use crate::utils::jwt;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRep {
    token: String,
}

/// login_handler
/// 
/// It's used to login user
pub async fn login_handler(
    State(AppState { ref dbp }): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> OhMyResult<Json<LoginRep>> {
    // Find user by email and password
    let (id, username) = sqlx::query!(
        "SELECT `id`, `username` FROM `users` WHERE `email` = ? AND `password` = ? LIMIT 1",
        &payload.email,
        &payload.password
    )
    .fetch_one(dbp)
    .await
    .map(|rec| (rec.id, rec.username))
    .map_err(|err| match err {
        // If record not found
        sqlx::Error::RowNotFound => ApiError::Authenticate(AuthenticateError::IncorrectEmailLogin),
        // Other error...
        other_err => ApiError::Sqlx(other_err),
    })?;

    // Get secret and generate token
    let token = jwt::generate(jwt::Payload { id, username }).map_err(|err| {
        tracing::error!(
            "an error occurred while generating the token, err: {}",
            err.to_string()
        );

        ApiError::Authenticate(AuthenticateError::GenerateToken)
    })?;

    // Return OK
    Ok(Json(LoginRep { token }))
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
    // Check if the user exists
    let exist = sqlx::query!(
        "SELECT EXISTS ( SELECT 1 FROM `users` WHERE `email` = ? LIMIT 1 ) AS `exists`",
        &payload.email
    )
    .fetch_one(dbp)
    .await
    .map(|rec| rec.exists == 1)
    .map_err(|err| {
        tracing::error!("an error occurred while creating a user");
        ApiError::Sqlx(err)
    })?;

    // If already exist
    if exist {
        return Err(ApiError::User(UserError::EmailAlreadyRegistered(
            payload.email.clone(),
        )));
    }

    // Otherwise create a new user
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

    // Return OK
    Ok(Json(serde_json::json!({"new_id": new_id})))
}
