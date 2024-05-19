
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    // Sqlx error
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),

    // Auth error
    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),
    // User module related error
    #[error("{0}")]
    User(#[from] UserError),
    // Other module errors
}

/// ## AuthenticateError
/// In **Authenticate** of ApiError
#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum AuthenticateError {
    #[error("failed to generate access token")]
    GenerateToken,
    #[error("invalid access token")]
    InvalidToken,
    #[error("user is locked")]
    Locked
}

/// UserError
/// In **User** of ApiError
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("user not found")]
    UserNotFound,
    // Other errors
}