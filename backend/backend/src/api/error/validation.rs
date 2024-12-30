use std::borrow::Cow;

use const_format::concatcp;
use serde::Serialize;
use utoipa::ToSchema;
use validator::ValidationError;

use crate::util::validation::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use crate::wrapper::entity::transaction::recurring::recurring_rule::dto::SPECIALS_STR;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct ValidationCode {
    pub(crate) code: &'static str,
    pub(crate) message: &'static str,
}

macro_rules! validation_codes {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $code:expr, $message:expr);
        )+
    ) => {
        impl ValidationCode {
        $(
            $(#[$docs])*
            pub(crate) const $konst: ValidationCode = ValidationCode{code: $code, message: $message};
        )+
        }
    }
}

// General
validation_codes!(
    (DATETIME_IN_PAST, "DATETIME_IN_PAST", "The datetime has to be in the future.");
);

// Entity related
validation_codes!(
    (ENTITY_NOT_FOUND, "ENTITY_NOT_FOUND", "The entity was not found.");
);

// Auth related
validation_codes!(
    // Password related
    (PASSWORD_TOO_SHORT, "PASSWORD_TOO_SHORT", concatcp!("Password is too short. Minimum length is ", MIN_PASSWORD_LENGTH));
    (PASSWORD_TOO_LONG, "PASSWORD_TOO_LONG", concatcp!("Password is too long. Maximum length is ", MAX_PASSWORD_LENGTH));
    (MISSING_UPPERCASE_LETTER, "MISSING_UPPERCASE_LETTER", "Missing at least one uppercase letter.");
    (MISSING_LOWERCASE_LETTER, "MISSING_LOWERCASE_LETTER", "Missing at least one lowercase letter.");
    (MISSING_DIGIT, "MISSING_DIGIT", "Missing at least one digit.");
    (MISSING_SPECIAL_CHARACTER, "MISSING_SPECIAL_CHARACTER", "Missing at least one special character.");

    // Username related
    (USERNAME_NOT_UNIQUE, "USERNAME_NOT_UNIQUE", "The chosen username is not unique.");
);

// Account related
validation_codes!(
    // IBAN related
    (IBAN_INVALID, "IBAN_INVALID", "IBAN is invalid");
);

// Transaction related
validation_codes!(
    (SOURCE_AND_DESTINATION_MISSING, "SOURCE_AND_DESTINATION_MISSING", "Source or destination must be present.");
);

// Recurring Rule related
validation_codes!(
    (INVALID_CRON_PATTERN, "INVALID_CRON_PATTERN", "Invalid cron pattern. At least one of day_of_month, month, day_of_week must be set to a value other than *");
    (INVALID_SPECIAL_FIELD, "INVALID_SPECIAL_FIELD", concatcp!("Invalid special field. Allowed values: {:?}", SPECIALS_STR));
);

// Server errors
validation_codes!(
    (INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR", "An internal server error occurred blocking us from validating.");
);

pub(crate) struct ValidationErrorBuilder {
    pub(crate) validation_error: ValidationError,
}

impl ValidationErrorBuilder {
    pub(crate) fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            validation_error: ValidationError::new(code).with_message(Cow::from(message)),
        }
    }
}

impl From<ValidationCode> for ValidationErrorBuilder {
    fn from(value: ValidationCode) -> Self {
        Self::new(value.code, value.message)
    }
}

impl From<ValidationErrorBuilder> for ValidationError {
    fn from(value: ValidationErrorBuilder) -> Self {
        value.validation_error
    }
}

impl From<ValidationErrorBuilder> for Result<(), ValidationError> {
    fn from(val: ValidationErrorBuilder) -> Self {
        Err(val.validation_error)
    }
}

impl From<ValidationCode> for Result<(), ValidationError> {
    fn from(val: ValidationCode) -> Self {
        ValidationErrorBuilder::from(val).into()
    }
}

impl From<ValidationCode> for ValidationError {
    fn from(val: ValidationCode) -> Self {
        ValidationErrorBuilder::from(val).into()
    }
}
