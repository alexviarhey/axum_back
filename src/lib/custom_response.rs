use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResultCode {
    Ok = 0,
    Err = 1,
}

#[derive(Serialize)]
pub struct CustomResponse<B = ()> {
    pub result_code: ResultCode,
    pub data: Option<B>,
    pub message: Option<String>,
}

impl<B> CustomResponse<B> {
    fn new(result_code: ResultCode, data: Option<B>, message: Option<String>) -> Self {
        Self {
            result_code,
            data,
            message,
        }
    }

    pub fn with_error_message(message: &str) -> Self {
        Self {
            result_code: ResultCode::Err,
            data: None,
            message: Some(message.to_string()),
        }
    }
}

impl<B> Default for CustomResponse<B> {
    fn default() -> Self {
        Self::new(ResultCode::Ok, None, None)
    }
}

impl<B> IntoResponse for CustomResponse<B>
where
    B: Serialize,
{
    fn into_response(self) -> Response {
        let custom_response = serde_json::to_string(&self).unwrap();
        (StatusCode::OK, custom_response).into_response()
    }
}

pub trait IntoCustomResponse {
    type Body: Serialize;

    fn into_custom_response(self) -> CustomResponse<Self::Body>;
}
