use async_trait::async_trait;
use axum::extract::FromRequest;
use serde::{de::DeserializeOwned, Deserialize};
use validator::Validate;

#[derive(Debug)]
struct ValidatedDto<T>(pub T);
