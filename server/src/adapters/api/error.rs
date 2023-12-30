use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::domain::repositories::error::RepositoryError;

pub(in crate::adapters::api) enum ControllerError {
    NotFound,
    BadInput,
    Unknown,
}

pub(in crate::adapters::api) type ControllerResult<T> = axum::response::Result<T, ControllerError>;

impl From<RepositoryError> for ControllerError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound => ControllerError::NotFound,
            RepositoryError::Unknown(_) => ControllerError::Unknown,
        }
    }
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> axum::response::Response {
        let response_code = match self {
            ControllerError::NotFound => StatusCode::NOT_FOUND,
            ControllerError::Unknown => StatusCode::SERVICE_UNAVAILABLE,
            ControllerError::BadInput => StatusCode::BAD_REQUEST,
        };

        response_code.into_response()
    }
}
