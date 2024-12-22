use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, ToSchema)]
pub struct ErrorCode {
    pub code: u16,
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
);

// Validation errors
error_codes!(
    (2000, GENERAL_VALIDATION_ERROR, "A general validation error occurred.");
    (2001, INVALID_VERIFICATION_TOKEN, "Invalid verification token.");
);

// Configuration error
error_codes!(
    (3001, QUEUE_PROVIDER_MISSING, "No provider is configured for the queue.");
);

// CLI errors
error_codes!(
    (4001, TASK_NOT_FOUND, "Task not found.");
);

// Db errors
error_codes!(
    (5000, GENERAL_DATABASE_ERROR, "A general database error occurred.");
    (5001, ENTITY_ALREADY_EXIST, "An entity with the same primary key already exists.");
    (5002, ENTITY_DOES_NOT_EXIST, "An entity that was requested does not exist.");
    (5003, CONNECTION_AQUIRE, "Database connection could not be acquired.");
    (5004, CONNECTION_ERROR, "Something went wrong while connecting to the database.");
    (5005, DB_EXECUTION_ERROR, "Could not execute operation successfully.");
    (5006, DB_QUERY_ERROR, "Error occurred while performing a query.");
    (5007, COULD_NOT_RETRIEVE_LAST_INSERT_ID, "Could not retrieve last insert id.");
    (5008, RECORD_NOT_FOUND, "Database record could not be found.");
    (5009, DB_CUSTOM_ERROR, "Custom DB error occurred.");
    (5010, ATTR_NOT_SET, "Attribute in active model not set.");
    (5011, PARSE_VALUE_AS_TARGET_TYPE, "An error occurred while trying to parse a value as a target type.");
    (5012, DB_PARSE_JSON, "Could not parse JSON from or to db.");
    (5013, MIGRATION_ERROR, "A migration error occurred.");
    (5014, RECORDS_NOT_INSERTED, "DB records could not get inserted.");
    (5015, RECORDS_NOT_UPDATED, "DB records could nto get updated.");
);

// Auth errors
error_codes!(
    (6001, INVALID_JWT, "An invalid JWT was provided.");
);
