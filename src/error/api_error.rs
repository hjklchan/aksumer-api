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
    /// Invalid JSON body
    #[error("Invalid JSON body")]
    InvalidJSONBody,
    /// ! Deprecated#2024/05/21: This parameter is indicates what ParameterName is invalid?
    /// This parameter is indicates what reason is invalid?
    #[error("{0}")]
    InvalidParameter(String),
    /// This parameter is indicates what ParameterName is missing?
    #[error("The {0} is mandatory for this action")]
    MissingParameter(String),
    /// Request validation error
    #[error(transparent)]
    Validation(#[from] validator::ValidationError),
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
            Self::ApiNotFound => (StatusCode::NOT_FOUND, 10002),
            Self::UnsupportedHTTPMethod => (StatusCode::METHOD_NOT_ALLOWED, 10003),
            Self::InvalidJSONBody => (StatusCode::BAD_REQUEST, 10004),
            Self::InvalidParameter(_) => (StatusCode::BAD_REQUEST, 10005),
            Self::MissingParameter(_) => (StatusCode::BAD_GATEWAY, 10006),
            Self::Validation(_) => (StatusCode::BAD_REQUEST, 10007),
            
            Self::Forbidden(_) => (StatusCode::FORBIDDEN, 10007),
            Self::Sqlx(_) => (StatusCode::INTERNAL_SERVER_ERROR, 10008),

            // Modules error
            // Authenticate related errors
            Self::Authenticate(auth_err) => match auth_err {
                AuthenticateError::MissingToken => (StatusCode::UNAUTHORIZED, 20101),
                AuthenticateError::InvalidToken => (StatusCode::UNAUTHORIZED, 20102),
                AuthenticateError::GenerateToken => (StatusCode::UNAUTHORIZED, 20103),
                AuthenticateError::IncorrectEmailLogin => (StatusCode::BAD_REQUEST, 20104),
                AuthenticateError::Locked => (StatusCode::LOCKED, 20105),
            },
            // User related errors
            Self::User(user_err) => match user_err {
                UserError::UserNotFound => (StatusCode::NOT_FOUND, 20201),
                UserError::EmailAlreadyRegistered(_) => (StatusCode::CONFLICT, 20202),
            },
        }
    }
}

impl Into<String> for ApiError {
    fn into(self) -> String {
        self.to_string()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_code) = self.get_codes();
        (
            status_code,
            Json(Response::<i32>::new(err_code, self.into(), None)),
        )
            .into_response()
    }
}

// # Sub-error extension

/// ## AuthenticateError
/// In **Authenticate** of ApiError
#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Incorrect email or password")]
    IncorrectEmailLogin,
    #[error("Failed to generate access token")]
    GenerateToken,
    #[error("Invalid access token")]
    InvalidToken,
    #[error("Missing access token")]
    MissingToken,
    #[error("Uer is locked")]
    Locked,
}

/// UserError
/// In **User** of ApiError
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("Email `{0}` already registered")]
    EmailAlreadyRegistered(String),
    // Other errors
}

