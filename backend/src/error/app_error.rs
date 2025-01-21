use crate::error::error_code::ErrorCode;
use axum::http::header::InvalidHeaderValue;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::{Display, Error};
use financrr_macros::app_errors;
use loco_rs::prelude::{Error as LocoError, ModelError};
use sea_orm::DbErr;
use serde::Serialize;
use serde_yaml::Error as YamlError;
use tracing::{error, warn};
use utoipa::{IntoResponses, ToSchema};
use validator::{ValidationError, ValidationErrors};

pub type AppResult<T> = Result<T, AppError>;

/// AppError is a custom error type that we use to return errors in the API with a specific structure.
#[derive(Debug, Clone, Serialize, ToSchema, Display, Error)]
#[display("{:#?}", self)]
pub struct AppError {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub error_code: ErrorCode,
    pub details: Option<String>,
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
app_errors!(
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::GENERAL_INTERNAL_SERVER_ERROR, GeneralInternalServerError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::SCHEDULER_ERROR, SchedulerError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::AXUM_ERROR, AxumError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::TERA_ERROR, TeraError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::ENV_VAR_ERROR, EnvVarError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::LETTRE_ERROR, LettreError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::IO_ERROR, IOError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::HASH_ERROR, HashError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::TASK_JOIN_ERROR, TaskJoinError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::REDIS_ERROR, RedisError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::STORAGE_ERROR, StorageError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::CACHE_ERROR, CacheError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::VERSION_CHECK_ERROR, VersionCheckError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::SMTP_ERROR, SmtpError, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::CONFIGURATION_ERROR, ConfigurationError, argument=String);
);

// Validation errors
app_errors!(
    (StatusCode::BAD_REQUEST, ErrorCode::GENERAL_VALIDATION_ERROR, GeneralValidationError, argument=Option<JsonReference>);
    (StatusCode::BAD_REQUEST, ErrorCode::INVALID_VERIFICATION_TOKEN, InvalidVerificationToken);
    (StatusCode::BAD_REQUEST, ErrorCode::JSON_ERROR, JsonError, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::JSON_REJECTION_ERROR, JsonRejectionError, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::YAML_FILE_ERROR, YamlFileError, argument=YamlFileErrorArgs);
    (StatusCode::BAD_REQUEST, ErrorCode::YAML_ERROR, YamlError, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::EMAIL_ADDRESS_PARSING_ERROR, EmailAddressParsingError, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::GENERAL_BAD_REQUEST, GeneralBadRequest, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::INVALID_HEADER_VALUE, InvalidHeaderValue, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::INVALID_HEADER_NAME, InvalidHeaderName, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::INVALID_HTTP_METHOD, InvalidHttpMethod, argument=String);
    (StatusCode::BAD_REQUEST, ErrorCode::INVALID_EMAIL_OR_PASSWORD, InvalidEmailOrPassword);
    (StatusCode::BAD_REQUEST, ErrorCode::EMAIL_NOT_VERIFIED, EmailNotVerified);
);

#[derive(Debug, Clone, Default, Serialize)]
pub struct YamlFileErrorArgs(String, String);

// User errors
app_errors!(
    (StatusCode::UNAUTHORIZED, ErrorCode::UNAUTHORIZED, Unauthorized, argument=String);
    (StatusCode::NOT_FOUND, ErrorCode::NOT_FOUND, NotFound);
);

// Configuration error
app_errors!(
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::QUEUE_PROVIDER_MISSING, QueueProviderMissing);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::EMAIL_CONFIGURATION_MISSING, EmailConfigurationMissing);
);

// CLI errors
app_errors!(
    (StatusCode::NOT_FOUND, ErrorCode::TASK_NOT_FOUND, TaskNotFound, argument=String);
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::GENERATOR_ERROR, GeneratorError, argument=String);
);

// DB Errors
app_errors!(
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::GENERAL_DATABASE_ERROR, GeneralDatabaseError, argument=Option<String>);
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
    (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::DB_MESSAGE_ERROR, DbMessageError, argument=String);
);

// Auth errors
app_errors!(
    (StatusCode::UNAUTHORIZED, ErrorCode::AUTH_HEADER_MISSING, AuthHeaderMissing);
    (StatusCode::UNAUTHORIZED, ErrorCode::INVALID_BEARER_TOKEN, InvalidBearerToken);
);

impl From<LocoError> for AppError {
    fn from(value: LocoError) -> Self {
        match value {
            LocoError::WithBacktrace { inner, .. } => Self::from(*inner),
            LocoError::Message(msg) => AppError::GeneralInternalServerError(msg),
            LocoError::QueueProviderMissing => AppError::QueueProviderMissing(),
            LocoError::TaskNotFound(msg) => AppError::TaskNotFound(msg),
            LocoError::Scheduler(err) => AppError::SchedulerError(err.to_string()),
            LocoError::Axum(err) => AppError::AxumError(err.to_string()),
            LocoError::Tera(err) => AppError::TeraError(err.to_string()),
            LocoError::JSON(err) => AppError::JsonError(err.to_string()),
            LocoError::JsonRejection(rej) => AppError::JsonRejectionError(rej.to_string()),
            LocoError::YAMLFile(err, path) => AppError::YamlFileError(YamlFileErrorArgs(err.to_string(), path)),
            LocoError::YAML(err) => AppError::YamlError(err.to_string()),
            LocoError::EnvVar(err) => AppError::EnvVarError(err.to_string()),
            LocoError::EmailSender(err) => AppError::LettreError(err.to_string()),
            LocoError::Smtp(err) => AppError::SmtpError(err.to_string()),
            LocoError::IO(err) => AppError::IOError(err.to_string()),
            LocoError::DB(err) => AppError::from(err),
            LocoError::ParseAddress(err) => AppError::EmailAddressParsingError(err.to_string()),
            LocoError::Hash(reference) => AppError::HashError(reference),
            LocoError::Unauthorized(str) => AppError::Unauthorized(str),
            LocoError::NotFound => AppError::NotFound(),
            LocoError::BadRequest(str) => AppError::GeneralBadRequest(str),
            LocoError::CustomError(status, detail) => AppError {
                status_code: status,
                error_code: ErrorCode::CUSTOM_ERROR,
                details: None,
                reference: JsonReference::new_with_default_none(&detail),
            },
            LocoError::InternalServerError => AppError::GeneralInternalServerError(Default::default()),
            LocoError::InvalidHeaderValue(err) => AppError::InvalidHeaderValue(err.to_string()),
            LocoError::InvalidHeaderName(err) => AppError::InvalidHeaderName(err.to_string()),
            LocoError::InvalidMethod(err) => AppError::InvalidHttpMethod(err.to_string()),
            LocoError::TaskJoinError(err) => AppError::TaskJoinError(err.to_string()),
            LocoError::Model(err) => AppError::from(err),
            LocoError::RedisPool(err) => AppError::RedisError(err.to_string()),
            LocoError::Redis(err) => AppError::RedisError(err.to_string()),
            LocoError::Sqlx(err) => AppError::GeneralDatabaseError(Some(err.to_string())),
            LocoError::Storage(err) => AppError::StorageError(err.to_string()),
            LocoError::Cache(err) => AppError::CacheError(err.to_string()),
            LocoError::Generators(err) => AppError::GeneratorError(err.to_string()),
            LocoError::VersionCheck(err) => AppError::VersionCheckError(err.to_string()),
            LocoError::Any(err) => AppError::GeneralInternalServerError(err.to_string()),
            LocoError::RequestError(err) => AppError::GeneralInternalServerError(err.to_string()),
            LocoError::SemVer(err) => AppError::VersionCheckError(err.to_string()),
            LocoError::ValidationError(err) => {
                AppError::GeneralValidationError(JsonReference::new_with_default_none(&err))
            }
            LocoError::AxumFormRejection(err) => {
                AppError::GeneralValidationError(JsonReference::new_with_default_none(&err.to_string()))
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
            ModelError::Jwt(err) => AppError::GeneralInternalServerError(err.to_string()),
            ModelError::DbErr(err) => AppError::from(err),
            ModelError::Any(err) => {
                error!("An general model error occurred: {:?}", err);

                AppError::GeneralInternalServerError("An unknown model error occurred.".to_string())
            }
            ModelError::Message(err) => AppError::DbMessageError(err.to_string()),
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

                AppError::GeneralDatabaseError(None)
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

                AppError::GeneralDatabaseError(None)
            }
            DbErr::UnpackInsertId => AppError::CouldNotRetrieveLastInsertId(),
            DbErr::UpdateGetPrimaryKey => AppError::GeneralDatabaseError(None),
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

impl From<YamlError> for AppError {
    fn from(value: YamlError) -> Self {
        AppError::YamlError(value.to_string())
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(value: InvalidHeaderValue) -> Self {
        AppError::InvalidHeaderValue(value.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::GeneralInternalServerError(value.to_string())
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

impl From<url::ParseError> for AppError {
    fn from(value: url::ParseError) -> Self {
        AppError::ConfigurationError(value.to_string())
    }
}

impl From<opensearch::http::transport::BuildError> for AppError {
    fn from(value: opensearch::http::transport::BuildError) -> Self {
        AppError::ConfigurationError(value.to_string())
    }
}

/// This is the worst case scenario error handler.
impl From<AppError> for ValidationError {
    fn from(value: AppError) -> Self {
        error!("An error occurred in custom validation function: {:?}", value);

        // TODO somehow fix this?
        //  - serialize the error into json
        //  - Create custom extractor that validates
        //      and converts error in GeneralValidationError or the given AppError
        //      if one is serialized into the field
        //  - This requires ValidationError to take non 'static strings
        ValidationError::new("INTERNAL_SERVER_ERROR")
    }
}
