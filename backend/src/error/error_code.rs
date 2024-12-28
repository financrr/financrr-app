use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, ToSchema)]
pub struct ErrorCode {
    pub code: u32,
    pub message: &'static str,
}

macro_rules! error_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl ErrorCode {
        $(
            $(#[$docs])*
            pub const $konst: ErrorCode = ErrorCode{code: $num, message: $phrase};
        )+

        }
    }
}

// Internal server errors
error_codes!(
    (1000, GENERAL_INTERNAL_SERVER_ERROR, "A general internal server error occurred.");
    (1001, SCHEDULER_ERROR, "A general error with the scheduler occurred.");
    (1002, AXUM_ERROR, "A general error with axum occurred.");
    (1003, TERA_ERROR, "A general error occurred while rendering a template");
    (1004, ENV_VAR_ERROR, "Unable to parse or load env var.");
    (1005, LETTRE_ERROR, "A general error occurred while sending an email.");
    (1006, IO_ERROR, "An io error occurred.");
    (1007, HASH_ERROR, "An error occurred while hashing a string.");
    (1008, TASK_JOIN_ERROR, "An error that occurred when not handling threads correctly.");
    (1009, REDIS_ERROR, "A general redis error occurred.");
    (1009, STORAGE_ERROR, "A general storage error occurred.");
    (1010, CACHE_ERROR, "A general cache error occurred.");
    (1011, VERSION_CHECK_ERROR, "Could not complete version check");
    (1012, SMTP_ERROR, "A general smtp error occurred.");
);

// Validation/User errors
error_codes!(
    (2000, GENERAL_VALIDATION_ERROR, "A general validation error occurred.");
    (2001, INVALID_VERIFICATION_TOKEN, "Invalid verification token.");
    (2002, JSON_REJECTION_ERROR, "A json input was rejected.");
    (2003, JSON_ERROR, "A general json serialization error occurred.");
    (2004, YAML_FILE_ERROR, "Could not parse yaml file.");
    (2005, YAML_ERROR, "A general yaml serialization error occurred.");
    (2006, EMAIL_ADDRESS_PARSING_ERROR, "Could not parse E-Mail address.");
    (2007, GENERAL_BAD_REQUEST, "A general error for bad requests.");
    (2008, INVALID_HEADER_VALUE, "A invalid header value was given.");
    (2009, INVALID_HEADER_NAME, "A invalid header name was given");
    (2010, INVALID_HTTP_METHOD, "A invalid http method was used.");
    (2011, INVALID_EMAIL_OR_PASSWORD, "Invalid E-Mail or Password given.");
);

// User errors
error_codes!(
    (3001, UNAUTHORIZED, "You are not authorized for this.");
    (3002, NOT_FOUND, "Requested resource could not be found.");
);

// Configuration error
error_codes!(
    (4001, QUEUE_PROVIDER_MISSING, "No provider is configured for the queue.");
    (4002, EMAIL_CONFIGURATION_MISSING, "No email configuration is set.");
);

// CLI errors
error_codes!(
    (5001, TASK_NOT_FOUND, "Task not found.");
    (5002, GENERATOR_ERROR, "Could not generate code template.");
);

// Db errors
error_codes!(
    (6000, GENERAL_DATABASE_ERROR, "A general database error occurred.");
    (6001, ENTITY_ALREADY_EXIST, "An entity with the same primary key already exists.");
    (6002, ENTITY_DOES_NOT_EXIST, "An entity that was requested does not exist.");
    (6003, CONNECTION_AQUIRE, "Database connection could not be acquired.");
    (6004, CONNECTION_ERROR, "Something went wrong while connecting to the database.");
    (6005, DB_EXECUTION_ERROR, "Could not execute operation successfully.");
    (6006, DB_QUERY_ERROR, "Error occurred while performing a query.");
    (6007, COULD_NOT_RETRIEVE_LAST_INSERT_ID, "Could not retrieve last insert id.");
    (6008, RECORD_NOT_FOUND, "Database record could not be found.");
    (6009, DB_CUSTOM_ERROR, "Custom DB error occurred.");
    (6010, ATTR_NOT_SET, "Attribute in active model not set.");
    (6011, PARSE_VALUE_AS_TARGET_TYPE, "An error occurred while trying to parse a value as a target type.");
    (6012, DB_PARSE_JSON, "Could not parse JSON from or to db.");
    (6013, MIGRATION_ERROR, "A migration error occurred.");
    (6014, RECORDS_NOT_INSERTED, "DB records could not get inserted.");
    (6015, RECORDS_NOT_UPDATED, "DB records could nto get updated.");
);

// Auth errors
error_codes!(
    (7001, INVALID_JWT, "An invalid JWT was provided.");
);

// Misc Errors
error_codes!(
    (99999, CUSTOM_ERROR, "Some kind of unmappable/custom error.");
);
