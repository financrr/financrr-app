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

// General errors
error_codes!(
    (1000, GENERAL_INTERNAL_SERVER_ERROR, "A general internal server error occurred.");
);

// Validation errors
error_codes!(
    (2001, EMAIL_NOT_UNQIUE, "Provided email is not unique");
);

// Configuration error
error_codes!(
    (3001, QUEUE_PROVIDER_MISSING, "No provider is configured for the queue.");
);

// CLI errors
error_codes!(
    (4001, TASK_NOT_FOUND, "Task not found.");
);
