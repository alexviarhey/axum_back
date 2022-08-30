use super::custom_response::{CustomResponse, IntoCustomResponse};
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationRejection {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("{0}")]
    FailedToDeserializeBody(String),
}

impl IntoCustomResponse for ValidationRejection {
    type Body = ();

    fn into_custom_response(self) -> CustomResponse<Self::Body> {
        let message = match self {
            ValidationRejection::ValidationError(_) => format!("[{}]", self).replace('\n', ", "),
            ValidationRejection::FailedToDeserializeBody(_) => format!("{}", self),
        };

        CustomResponse::with_error_message(&message)
    }
}

impl IntoResponse for ValidationRejection {
    fn into_response(self) -> axum::response::Response {
        self.into_custom_response().into_response()
    }
}
