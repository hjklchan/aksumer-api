use axum::response::IntoResponse;

use crate::error::api_error::ApiError;

pub async fn not_found_handler() -> impl IntoResponse {
    ApiError::ApiNotFound
}
