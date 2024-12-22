use crate::error::error_code::ErrorCode;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::{Display, Error};
use financrr_macros::app_errors;
use loco_rs::prelude::{Error as LocoError, ModelError};
use sea_orm::DbErr;
use serde::Serialize;
use tracing::{error, warn};
use utoipa::{IntoResponses, ToSchema};
use validator::ValidationErrors;

pub type AppResult<T> = Result<T, AppError>;

/// AppError is a custom error type that we use to return errors in the API with a specific structure.
#[derive(Debug, Clone, Serialize, ToSchema, Display, Error)]
#[display("{}", serde_json::to_string(self).expect("Failed to serialize AppError"))]
pub struct AppError {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub error_code: ErrorCode,
    pub details: String,
    pub reference: Option<JsonReference>,
}

#[derive(Debug, Clone, Serialize, ToSchema, Display, Error)]
pub struct JsonReference {
    payload: serde_json::Value,
}

impl JsonReference {
    pub fn new<T: Serialize>(payload: &T) -> Result<Self, serde_json::Error> {
        let payload = serde_json::to_value(payload)?;

        Ok(Self { payload })
    }

    pub fn new_with_default_none<T: Serialize>(payload: &T) -> Option<Self> {
        match Self::new(payload) {
            Ok(payload) => Some(payload),
            Err(e) => {
                warn!("Failed to serialize reference payload. Error: {}", e);

                None
            }
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
            reference: None,
        }
    }
}

// Validation errors
app_errors!(
    (StatusCode::BAD_REQUEST, ErrorCode::GENERAL_VALIDATION_ERROR, GeneralValidationError, argument=Option<JsonReference>);
    (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken);
);

// Configuration error
app_errors!(
  (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::QUEUE_PROVIDER_MISSING, QueueProviderMissing);
);

// CLI errors
app_errors!(
    (StatusCode::NOT_FOUND, ErrorCode::TASK_NOT_FOUND, TaskNotFound, argument=String);
);

// DB Errors
app_errors!(
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::GENERAL_DATABASE_ERROR, GeneralDatabaseError);
    (StatusCode::BAD_REQUEST, ErrorCode::ENTITY_ALREADY_EXIST, EntityAlreadyExists);
    (StatusCode::NOT_FOUND, ErrorCode::ENTITY_DOES_NOT_EXIST, EntityNotFound);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::CONNECTION_ERROR, ConnectionError);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_EXECUTION_ERROR, DbExecutionError);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_QUERY_ERROR, DbQueryError);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::COULD_NOT_RETRIEVE_LAST_INSERT_ID, CouldNotRetrieveLastInsertId);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::RECORDS_NOT_INSERTED, RecordsNotInserted);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::RECORDS_NOT_UPDATED, RecordsNotUpdated);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::CONNECTION_AQUIRE, ConnectionAcquire, argument=Option<JsonReference>);
    (StatusCode::NOT_FOUND, ErrorCode::RECORD_NOT_FOUND, RecordNotFound, argument=Option<JsonReference>);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_CUSTOM_ERROR, DbCustomError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::ATTR_NOT_SET, AttrNotSet, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::PARSE_VALUE_AS_TARGET_TYPE, ParseValueAsTargetType, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_PARSE_JSON, DbParseJson, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::MIGRATION_ERROR, MigrationError, argument=String);
);

// Auth errors
app_errors!(
    (StatusCode::UNAUTHORIZED, ErrorCode::INVALID_JWT, InvalidJwt, argument=Option<JsonReference>);
);

impl From<LocoError> for AppError {
    fn from(value: LocoError) -> Self {
        match value {
            LocoError::WithBacktrace { inner, .. } => Self::from(*inner),
            LocoError::Message(msg) => AppError::GeneralInternalServerError(msg),
            LocoError::QueueProviderMissing => AppError::QueueProviderMissing(),
            LocoError::TaskNotFound(msg) => AppError::TaskNotFound(msg),
            // TODO: Add missing mappings
            e => {
                error!("An unmapped loco error occurred: {:?}", e);

                AppError::GeneralInternalServerError("An unknown error occurred.".to_string())
            }
        }
    }
}

impl From<ModelError> for AppError {
    fn from(value: ModelError) -> Self {
        match value {
            ModelError::EntityAlreadyExists => AppError::EntityAlreadyExists(),
            ModelError::EntityNotFound => AppError::EntityNotFound(),
            ModelError::ModelValidation { errors } => {
                AppError::GeneralValidationError(JsonReference::new_with_default_none(&errors))
            }
            ModelError::Jwt(err) => AppError::InvalidJwt(JsonReference::new_with_default_none(&err.to_string())),
            ModelError::DbErr(err) => AppError::from(err),
            ModelError::Any(err) => {
                error!("An general model error occurred: {:?}", err);

                AppError::GeneralInternalServerError("An unknown model error occurred.".to_string())
            }
        }
    }
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::ConnectionAcquire(err) => {
                AppError::ConnectionAcquire(JsonReference::new_with_default_none(&err.to_string()))
            }
            DbErr::TryIntoErr { into, from, source } => {
                error!(
                    "An error occurred while converting from {} to {}. Source: {:?}",
                    from, into, source
                );

                AppError::GeneralDatabaseError()
            }
            DbErr::Conn(err) => {
                error!("An error occurred while connecting to the database. Error: {:?}", err);

                AppError::ConnectionError()
            }
            DbErr::Exec(err) => {
                error!("An error occurred while executing an operation. Error: {:?}", err);

                AppError::DbExecutionError()
            }
            DbErr::Query(err) => {
                error!("An error occurred while performing a query. Error: {:?}", err);

                AppError::DbQueryError()
            }
            DbErr::ConvertFromU64(err) => {
                // Still mapping this error to prevent crashes/panicking.
                // Documentation from Sea-ORM states that this is not a runtime error.
                error!(
                    "Conversion error from u64. This should not occur during runtime! Error: {:?}",
                    err
                );

                AppError::GeneralDatabaseError()
            }
            DbErr::UnpackInsertId => AppError::CouldNotRetrieveLastInsertId(),
            DbErr::UpdateGetPrimaryKey => AppError::GeneralDatabaseError(),
            DbErr::RecordNotFound(err) => AppError::RecordNotFound(JsonReference::new_with_default_none(&err)),
            DbErr::AttrNotSet(attr) => {
                error!("An attribute was not set. Attribute: {}", attr);

                AppError::AttrNotSet(attr)
            }
            DbErr::Custom(err) => {
                error!("A custom error occurred. Error: {}", err);

                AppError::DbCustomError(err)
            }
            DbErr::Type(typ) => {
                error!(
                    "An error occurred while trying to parse a value as a target type. Type: {}",
                    typ
                );

                AppError::ParseValueAsTargetType(typ)
            }
            DbErr::Json(json) => AppError::DbParseJson(json),
            DbErr::Migration(err) => AppError::MigrationError(err),
            DbErr::RecordNotInserted => AppError::RecordsNotInserted(),
            DbErr::RecordNotUpdated => AppError::RecordsNotUpdated(),
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(value: ValidationErrors) -> Self {
        AppError::GeneralValidationError(JsonReference::new_with_default_none(&value))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, axum::Json(self)).into_response()
    }
}

impl From<AppError> for LocoError {
    fn from(value: AppError) -> Self {
        LocoError::Any(value.into())
    }
}
