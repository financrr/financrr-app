use crate::error::error_code::ErrorCode;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::{Display, Error};
use loco_rs::prelude::Error as LocoError;
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

/// AppError is a custom error type that we use to return errors in the API with a specific structure.
#[derive(Debug, Clone, Serialize, ToSchema, Display, Error)]
#[display("{}", serde_json::to_string(self).expect("Failed to serialize AppError"))]
pub struct AppError {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub error_code: ErrorCode,
    pub details: String,
}

macro_rules! app_errors {
    (
        $(
            $(#[$docs:meta])*
            ($status_code:expr, $error_code:expr, $details:expr, $func:ident);
        )+
    ) => {
        impl AppError {
        $(
            $(#[$docs])*
            #[allow(non_snake_case)]
            pub(crate) fn $func() -> Self {
                Self {
                    status_code: $status_code,
                    error_code: $error_code,
                    details: String::from($details),
                }
            }
        )+
        }
    }
}

// General errors
impl AppError {
    #[allow(non_snake_case)]
    pub fn GeneralInternalServerError(msg: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::GENERAL_INTERNAL_SERVER_ERROR,
            details: msg,
        }
    }
}

// Configuration error
app_errors!(
  (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::QUEUE_PROVIDER_MISSING, "No provider is configured for the queue.", QueueProviderMissing);
);

// CLI errors

impl AppError {
    #[allow(non_snake_case)]
    pub fn TaskNotFound(msg: String) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND,
            error_code: ErrorCode::TASK_NOT_FOUND,
            details: msg,
        }
    }
}

impl From<LocoError> for AppError {
    fn from(value: LocoError) -> Self {
        match value {
            LocoError::WithBacktrace { inner, .. } => Self::from(*inner),
            LocoError::Message(msg) => AppError::GeneralInternalServerError(msg),
            LocoError::QueueProviderMissing => AppError::QueueProviderMissing(),
            LocoError::TaskNotFound(msg) => AppError::TaskNotFound(msg),
            e => {
                error!("An unmapped loco error occurred: {:?}", e);

                AppError::GeneralInternalServerError("An unknown error occurred.".to_string())
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        let body = serde_json::to_string(&self).expect("Failed to serialize AppError");

        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .expect("Failed to build response")
    }
}
