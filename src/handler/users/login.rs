use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{
        api_error::{ApiError, AuthenticateError},
        OhMyResult,
    },
    utils::jwt,
    AppState,
};

#[derive(Debug, Deserialize, validator::Validate)]
pub struct LoginReq {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 20))]
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
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> OhMyResult<Json<LoginRep>> {
    // Find user by email and password
    let (id, username) = sqlx::query!(
        "SELECT `id`, `username` FROM `users` WHERE `email` = ? AND `password` = ? LIMIT 1",
        &payload.email,
        &payload.password
    )
    .fetch_one(&state.dbp)
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
