use actix_web::error::ResponseError;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use croner::errors::CronError;
use derive_more::{Display, Error};
use deschuler::cron_builder::config::BuilderConfigBuilderError;
use redis::RedisError;
use sea_orm::DbErr;
use serde::{Serialize, Serializer};
use tracing::error;
use utoipa::ToSchema;
use validator::{ValidationError, ValidationErrors};

use entity::error::EntityError;
use utility::datetime::error::TimeError;
use utility::snowflake::error::SnowflakeGeneratorError;

use crate::api::error::api_codes::ApiCode;
use crate::api::error::validation;
use crate::util::validation::ValidationErrorJsonPayload;

#[derive(Debug, Display, Error, Serialize, ToSchema)]
#[display("{}", serde_json::to_string(self).expect("Failed to serialize ApiError"))]
pub(crate) struct ApiError {
    #[serde(skip)]
    pub(crate) status_code: StatusCode,
    pub(crate) api_code: ApiCode,
    pub(crate) details: String,
    pub(crate) reference: Option<SerializableStruct>,
}

#[derive(Debug, ToSchema)]
pub(crate) struct SerializableStruct {
    serialized: serde_json::Value,
}

impl SerializableStruct {
    pub(crate) fn new<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        let serialized = serde_json::to_value(value)?;
        Ok(Self {
            serialized,
        })
    }
}

impl Serialize for SerializableStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.serialized.serialize(serializer)
    }
}

macro_rules! api_errors {
    (
        $(
            $(#[$docs:meta])*
            ($status_code:expr, $api_code:expr, $details:expr, $func:ident);
        )+
    ) => {
        impl ApiError {
        $(
            $(#[$docs])*
            #[allow(non_snake_case)]
            pub(crate) fn $func() -> Self {
                Self {
                    status_code: $status_code,
                    api_code: $api_code,
                    details: String::from($details),
                    reference: None,
                }
            }
        )+
        }
    }
}

api_errors!(
    (StatusCode::UNAUTHORIZED, ApiCode::INVALID_SESSION, "Invalid session!", InvalidSession);
    (StatusCode::UNAUTHORIZED, ApiCode::INVALID_CREDENTIALS, "Invalid credentials!", InvalidCredentials);
    (StatusCode::UNAUTHORIZED, ApiCode::UNAUTHORIZED, "Unauthorized!", Unauthorized);
    (StatusCode::FORBIDDEN, ApiCode::MISSING_PERMISSIONS, "Missing permissions!", MissingPermissions);
    (StatusCode::UNAUTHORIZED, ApiCode::NO_TOKEN_PROVIDED, "No token provided!", NoTOkenProvided);
);

impl ApiError {
    #[allow(non_snake_case)]
    pub(crate) fn ResourceNotFound(resource_name: &str) -> Self {
        Self {
            status_code: StatusCode::NOT_FOUND,
            api_code: ApiCode::RESOURCE_NOT_FOUND,
            details: format!("Could not found {}!", resource_name),
            reference: None,
        }
    }

    pub(crate) fn from_error_vec(errors: Vec<Self>, status_code: StatusCode) -> Self {
        Self {
            status_code,
            api_code: ApiCode::UNKNOWN,
            details: "Multiple errors occurred!".to_string(),
            reference: SerializableStruct::new(&errors).ok(),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        if self.status_code.eq(&StatusCode::INTERNAL_SERVER_ERROR) {
            error!("Internal server error: {}", self);
            //TODO insert sentry error here
        }

        HttpResponse::build(self.status_code()).insert_header(ContentType::json()).json(self)
    }
}

impl From<EntityError> for ApiError {
    fn from(error: EntityError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::ENTITY_ERROR,
            details: error.to_string(),
            reference: None,
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::DB_ERROR,
            details: value.to_string(),
            reference: None,
        }
    }
}

impl From<RedisError> for ApiError {
    fn from(value: RedisError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::REDIS_ERROR,
            details: value.to_string(),
            reference: None,
        }
    }
}

impl From<ValidationErrorJsonPayload> for ApiError {
    fn from(value: ValidationErrorJsonPayload) -> Self {
        let serializable_struct = SerializableStruct::new(&value).ok();
        Self {
            status_code: StatusCode::BAD_REQUEST,
            api_code: ApiCode::JSON_PAYLOAD_VALIDATION_ERROR,
            details: value.message,
            reference: serializable_struct,
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(value: ValidationError) -> Self {
        Self::from(ValidationErrorJsonPayload::from(value))
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        Self::from(ValidationErrorJsonPayload::from(&value))
    }
}

impl From<validation::ValidationError> for ApiError {
    fn from(value: validation::ValidationError) -> Self {
        Self::from(value.get_error().to_owned())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            api_code: ApiCode::SERIALIZATION_ERROR,
            details: value.to_string(),
            reference: None,
        }
    }
}

impl From<serde_yml::Error> for ApiError {
    fn from(value: serde_yml::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::SERIALIZATION_ERROR,
            details: value.to_string(),
            reference: None,
        }
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(error: actix_web::Error) -> Self {
        Self {
            status_code: error.as_response_error().status_code(),
            api_code: ApiCode::ACTIX_ERROR,
            details: error.to_string(),
            reference: None,
        }
    }
}

impl From<CronError> for ApiError {
    fn from(value: CronError) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            api_code: ApiCode::CRON_ERROR,
            details: value.to_string(),
            reference: None,
        }
    }
}

impl From<BuilderConfigBuilderError> for ApiError {
    fn from(value: BuilderConfigBuilderError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::CRON_BUILDER_ERROR,
            details: value.to_string(),
            reference: None,
        }
    }
}

impl From<TimeError> for ApiError {
    fn from(value: TimeError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::TIME_ERROR,
            details: value.to_string(),
            reference: SerializableStruct::new(&value).ok(),
        }
    }
}

impl From<SnowflakeGeneratorError> for ApiError {
    fn from(value: SnowflakeGeneratorError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            api_code: ApiCode::SNOWFLAKE_ERROR,
            details: value.to_string(),
            reference: SerializableStruct::new(&value).ok(),
        }
    }
}
