use std::collections::HashMap;

use super::custom_response::{
    CustomResponse, CustomResponseBuilder, IntoCustomResponse, ResultCode,
};
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
        match self {
            ValidationRejection::ValidationError(_) => {
                let mut validation_errors: HashMap<String, String> = HashMap::new();

                format!("{}", self)
                    .replace('\n', ", ")
                    .split(", ")
                    .for_each(|err| {
                        let splitted_err: Vec<&str> = err.split(":").collect();

                        validation_errors.insert(
                            splitted_err[0].to_string(),
                            splitted_err[1].trim().to_string(),
                        );
                    });

                CustomResponseBuilder::new()
                    .result_code(ResultCode::Err)
                    .validation_errors(validation_errors)
                    .build()
            }
            ValidationRejection::FailedToDeserializeBody(_) => CustomResponseBuilder::new()
                .result_code(ResultCode::Err)
                .message(format!("{}", self).as_str())
                .build(),
        }
    }
}

impl IntoResponse for ValidationRejection {
    fn into_response(self) -> axum::response::Response {
        self.into_custom_response().into_response()
    }
}
