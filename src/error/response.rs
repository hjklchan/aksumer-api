// use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize
{
    pub fn new(code: u16, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
}
