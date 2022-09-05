use std::collections::HashMap;

use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize_repr)]
#[repr(u8)]

pub enum ResultCode {
    Ok = 0,
    Err = 1,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomResponse<B = ()> {
    pub result_code: ResultCode,
    pub data: Option<B>,
    pub message: Option<String>,
    pub validation_errors: Option<HashMap<String, String>>,
}

impl<B> CustomResponse<B> {
    fn new(
        result_code: ResultCode,
        data: Option<B>,
        message: Option<String>,
        validation_errors: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            result_code,
            data,
            message,
            validation_errors,
        }
    }
}

impl<B> Default for CustomResponse<B> {
    fn default() -> Self {
        Self::new(ResultCode::Ok, None, None, None)
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

    pub fn validation_errors(mut self, errors: HashMap<String, String>) -> Self {
        self.custom_response.validation_errors = Some(errors);
        self
    }

    pub fn build(self) -> CustomResponse<T> {
        self.custom_response
    }
}
