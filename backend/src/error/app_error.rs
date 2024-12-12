use crate::error::error_code::ErrorCode;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::{Display, Error};
use loco_rs::prelude::{Error as LocoError, ModelError};
use sea_orm::DbErr;
use serde::Serialize;
use tracing::{error, warn};
use utoipa::ToSchema;
use validator::ValidationErrors;

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
                    reference: None,
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
            reference: None,
        }
    }
}

// Validation errors
impl AppError {
    #[allow(non_snake_case)]
    pub fn GeneralValidationError(msg: String, reference: Option<JsonReference>) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            error_code: ErrorCode::GENERAL_VALIDATION_ERROR,
            details: msg,
            reference,
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
            reference: None,
        }
    }
}

// DB Errors
app_errors!(
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::GENERAL_DATABASE_ERROR, "A general database error occurred. Please take look at the logs", GeneralDatabaseError);
    (StatusCode::BAD_REQUEST, ErrorCode::ENTITY_ALREADY_EXIST, "An entity with the same primary key already exists.", EntityAlreadyExists);
    (StatusCode::NOT_FOUND, ErrorCode::ENTITY_DOES_NOT_EXIST, "An entity that was requested does not exist", EntityNotFound);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::CONNECTION_ERROR, "Something went wrong while connecting to the database.", ConnectionError);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_EXECUTION_ERROR, "Could not execute operation successfully.", DbExecutionError);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_QUERY_ERROR, "Error occurred while performing a query.", DbQueryError);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::COULD_NOT_RETRIEVE_LAST_INSERT_ID, "Could not retrieve last insert id.", CouldNotRetrieveLastInsertId);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::RECORDS_NOT_INSERTED, "Records could not be inserted.", RecordsNotInserted);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::RECORDS_NOT_UPDATED, "Records could not be updated.", RecordsNotUpdated);
);

impl AppError {
    #[allow(non_snake_case)]
    pub fn ConnectionAcquire(reference: Option<JsonReference>) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::CONNECTION_AQUIRE,
            details: "Connection could not be acquired. Maybe your DB Pool is to small configured.".to_string(),
            reference,
        }
    }

    #[allow(non_snake_case)]
    pub fn RecordNotFound(reference: Option<JsonReference>) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND,
            error_code: ErrorCode::RECORD_NOT_FOUND,
            details: "Database record could not be found.".to_string(),
            reference,
        }
    }

    #[allow(non_snake_case)]
    pub fn DbCustomError(msg: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::DB_CUSTOM_ERROR,
            details: "A custom DB error occurred.".to_string(),
            reference: JsonReference::new_with_default_none(&msg),
        }
    }

    #[allow(non_snake_case)]
    pub fn AttrNotSet(attribute: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::ATTR_NOT_SEND,
            details: "Attribute in active model not set.".to_string(),
            reference: JsonReference::new_with_default_none(&attribute),
        }
    }

    #[allow(non_snake_case)]
    pub fn ParseValueAsTargetType(typ: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::PARSE_VALUE_AS_TARGET_TYPE,
            details: ErrorCode::PARSE_VALUE_AS_TARGET_TYPE.message.to_string(),
            reference: JsonReference::new_with_default_none(&typ),
        }
    }

    #[allow(non_snake_case)]
    pub fn DbParseJson(json: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::DB_PARSE_JSON,
            details: ErrorCode::DB_PARSE_JSON.message.to_string(),
            reference: JsonReference::new_with_default_none(&json),
        }
    }

    #[allow(non_snake_case)]
    pub fn MigrationError(err: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ErrorCode::MIGRATION_ERROR,
            details: "A migration error occurred.".to_string(),
            reference: JsonReference::new_with_default_none(&err),
        }
    }
}

// Auth errors
impl AppError {
    #[allow(non_snake_case)]
    pub fn InvalidJwt(reference: Option<JsonReference>) -> Self {
        Self {
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ErrorCode::INVALID_JWT,
            details: "An invalid JWT was provided.".to_string(),
            reference,
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
            ModelError::ModelValidation { errors } => AppError::GeneralValidationError(
                "Validation error occurred.".to_string(),
                JsonReference::new_with_default_none(&errors),
            ),
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
        AppError::GeneralValidationError(
            "Validation error occurred.".to_string(),
            JsonReference::new_with_default_none(&value),
        )
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
