use serde::Serialize;
use utoipa::openapi::{RefOr, Schema};
use utoipa::{schema, ToSchema};

#[derive(Debug, Serialize)]
pub(crate) struct ApiCode {
    pub(crate) code: u16,
    pub(crate) message: &'static str,
}

impl ToSchema<'static> for ApiCode {
    fn schema() -> (&'static str, RefOr<Schema>) {
        (
            "ApiCode",
            schema!(
                #[inline]
                i32
            )
            .nullable(false)
            .into(),
        )
    }
}

macro_rules! api_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl ApiCode {
        $(
            $(#[$docs])*
            pub(crate) const $konst: ApiCode = ApiCode{code: $num, message: $phrase};
        )+

        }
    }
}

// Auth related
api_codes!(
    (1000, INVALID_SESSION, "Invalid session!");
    (1002, INVALID_CREDENTIALS, "Invalid credentials provided!");
    (1004, UNAUTHORIZED, "Unauthorized!");
    (1006, NO_TOKEN_PROVIDED, "No bearer token provided!");
);

// User-causes errors
api_codes!(
    (1100, RESOURCE_NOT_FOUND, "Requested resource was not found!");
    (1101, SERIALIZATION_ERROR, "Serialization error!");
    (1102, MISSING_PERMISSIONS, "Missing permissions!");
    (1103, CRON_ERROR, "Error while parsing to cron!");
);

//validation errors
api_codes!(
    (1200, JSON_PAYLOAD_VALIDATION_ERROR, "JSON payload validation error!");
);

// internal server-errors
api_codes!(
    (1300, ENTITY_ERROR, "DB-Entity error!");
    (1301, DB_ERROR, "Database error!");
    (1302, REDIS_ERROR, "Redis error!");
    (1303, CRON_BUILDER_ERROR, "Cron builder error!");
    (1304, TIME_ERROR, "An internal time-error!");
    (1305, SNOWFLAKE_ERROR, "An internal error that occurs when a snowflake could not be generated!");
);

// misc
api_codes!(
    (9000, ACTIX_ERROR, "Actix error!");
    (9999, UNKNOWN, "Unknown error!");
);
