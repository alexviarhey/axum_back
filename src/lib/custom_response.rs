use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
enum ResultCode {
    Ok,
    Err,
}

#[derive(Serialize)]
pub struct CustomResponse<T> {
    result_code: ResultCode,
    messages: Option<Vec<String>>,
    data: Option<T>,
}

impl<T> Default for CustomResponse<T> {
    fn default() -> Self {
        Self {
            result_code: ResultCode::Ok,
            messages: None,
            data: None,
        }
    }
}

impl<T> IntoResponse for CustomResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let custom_response = serde_json::to_string(&self).unwrap();
        (StatusCode::OK, custom_response).into_response()
    }
}

pub trait IntoCustomResponse {
    type Data: Serialize;
    fn into_custom_response(self) -> CustomResponse<Self::Data>;
}

impl<T> IntoCustomResponse for T
where
    T: Serialize,
{
    type Data = T;

    fn into_custom_response(self) -> CustomResponse<T> {
        CustomResponse {
            result_code: ResultCode::Ok,
            messages: None,
            data: Some(self),
        }
    }
}
