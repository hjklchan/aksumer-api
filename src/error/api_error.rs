use axum::{http::StatusCode, response::IntoResponse, Json};

use super::response::Response;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    // Basic error
    #[error("The request processing has failed due to some unknown error")]
    Internal,
    // The following errors are all from Alibaba
    #[error("Specified api is not found, please check your url and method")]
    ApiNotFound,
    #[error("Specified signature is not matched with our calculation")]
    UnsupportedHTTPMethod,
    /// This parameter is indicates what ParameterName is invalid?
    #[error("The specified parameter {0} value is not valid")]
    InvalidParameter(String),
    /// This parameter is indicates what ParameterName is missing?
    #[error("The {0} is mandatory for this action")]
    MissingParameter(String),
    /// This parameter is indicates the forbidden reason?
    #[error("{0}")]
    Forbidden(String),

    // Sqlx error is from sqlx::Error
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),

    // Module-level errors
    // Authenticate error
    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),
    // User module related error
    #[error("{0}")]
    User(#[from] UserError),
    // Other module errors
}

impl ApiError {
    /// Transform to StatusCode and ErrorCode
    fn get_codes(&self) -> (StatusCode, u16) {
        match self {
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, 10001),
            Self::ApiNotFound => (StatusCode::NOT_FOUND, 1001),
            Self::UnsupportedHTTPMethod => (StatusCode::METHOD_NOT_ALLOWED, 1002),
            Self::InvalidParameter(_) => (StatusCode::BAD_REQUEST, 1003),
            Self::MissingParameter(_) => (StatusCode::BAD_GATEWAY, 1004),
            Self::Forbidden(_) => (StatusCode::FORBIDDEN, 1005),
            Self::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, 1006),
            // Modules error
            // Authenticate related errors
            Self::Authenticate(auth_err) => match auth_err {
                AuthenticateError::GenerateToken => (StatusCode::UNAUTHORIZED, 2001),
                AuthenticateError::InvalidToken => (StatusCode::UNAUTHORIZED, 2002),
                AuthenticateError::Locked => (StatusCode::LOCKED, 2003),
            },
            // User related errors
            Self::User(user_err) => match user_err {
                UserError::UserNotFound => (StatusCode::NOT_FOUND, 3001),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_code) = self.get_codes();
        Json(Response::<i32>::new(err_code, self.to_string(), None)).into_response()
    }
}

// # Sub-error extension

/// ## AuthenticateError
/// In **Authenticate** of ApiError
#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Failed to generate access token")]
    GenerateToken,
    #[error("Invalid access token")]
    InvalidToken,
    #[error("Uer is locked")]
    Locked,
}

/// UserError
/// In **User** of ApiError
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    // Other errors
}

pub type OhMyResult<T> = Result<T, ApiError>;