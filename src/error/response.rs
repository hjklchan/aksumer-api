// use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn new(code: &'static str, message: String, data: Option<T>) -> Self {
        Self {
            code,
            message,
            data,
        }
    }
}
