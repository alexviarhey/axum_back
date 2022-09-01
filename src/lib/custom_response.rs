use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ResultCode {
    Ok = 0,
    Err = 1,
}

#[derive(Debug, Serialize)]
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

    pub fn success(data: B, message: &str) -> Self {
        Self {
            result_code: ResultCode::Ok,
            data: Some(data),
            message: Some(message.to_string()),
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

#[derive(Debug)]
pub struct CustomResponseBuilder<T: Serialize> {
    custom_response: CustomResponse<T>,
}

impl<T> CustomResponseBuilder<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self {
            custom_response: CustomResponse::default(),
        }
    }

    pub fn data(mut self, data: T) -> Self {
        self.custom_response.data = Some(data);
        self
    }

    pub fn result_code(mut self, result_code: ResultCode) -> Self {
        self.custom_response.result_code = result_code;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.custom_response.message = Some(message.to_string());
        self
    }

    pub fn build(self) -> CustomResponse<T> {
        self.custom_response
    }
}
