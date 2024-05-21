pub mod api_error;
pub mod response;

pub type OhMyResult<T> = Result<T, api_error::ApiError>;