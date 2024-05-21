use crate::error::api_error::ApiError;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| ApiError::InvalidJSONBody)?;

        value
            .validate()
            // Get first item(field) in the errors iterator
            .map_err(|errs| match errs.field_errors().into_iter().next() {
                Some(value) => ApiError::InvalidParameter(value.0.into()),
                None => ApiError::InvalidParameter("unknown".into()),
            })?;

        Ok(ValidatedJson(value))
    }
}