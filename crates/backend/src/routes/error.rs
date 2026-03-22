use crate::services::error::ServiceError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("api error")]
pub struct ApiError {
    pub status: StatusCode,
}

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::NotFound => Self {
                status: StatusCode::NOT_FOUND,
            },
            ServiceError::Bucket(_) | ServiceError::Internal => Self {
                status: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.status.into_response()
    }
}
