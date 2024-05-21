use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{error::api_error::{ApiError, OhMyResult, UserError}, AppState, extractor::json_validator::ValidatedJson};


#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateReq {
    #[validate(length(min = 5, max = 12))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 20))]
    password: String,
}

pub async fn create_handler(
    State(AppState { ref dbp }): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateReq>,
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