use std::fmt::Debug;

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::{FromRequest, RequestParts},
    BoxError, Form,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::errors::ValidationRejection;

#[derive(Debug)]
pub struct ValidatedDto<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedDto<T>
where
    T: DeserializeOwned + Validate + Debug,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ValidationRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req).await.unwrap();
        let value = serde_json::from_slice::<T>(&bytes)
            .map_err(|e| ValidationRejection::FailedToDeserializeBody(e.to_string()))?;
        value.validate()?;
        Ok(ValidatedDto(value))
    }
}
