use iban::Iban;
use lazy_regex::regex;
use sea_orm::EntityTrait;
use time::OffsetDateTime;
use tokio::runtime::Handle;
use validator::ValidationError;

use entity::currency;
use entity::prelude::User;
use entity::utility::time::get_now;
use utility::snowflake::entity::Snowflake;

use crate::api::error::validation::ValidationCode;
use crate::database::connection::get_database_connection;

pub(crate) const MIN_PASSWORD_LENGTH: usize = 8;
pub(crate) const MAX_PASSWORD_LENGTH: usize = 255;

pub(crate) fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return ValidationCode::PASSWORD_TOO_SHORT.into();
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return ValidationCode::PASSWORD_TOO_LONG.into();
    }

    let uppercase = regex!(r"[A-Z]");
    if !uppercase.is_match(password) {
        return ValidationCode::MISSING_UPPERCASE_LETTER.into();
    }

    let lowercase = regex!(r"[a-z]");
    if !lowercase.is_match(password) {
        return ValidationCode::MISSING_LOWERCASE_LETTER.into();
    }

    let digit = regex!(r"\d");
    if !digit.is_match(password) {
        return ValidationCode::MISSING_DIGIT.into();
    }

    let special_character = regex!("[€§!@#$%^&*(),.?\":{}|<>]");
    if !special_character.is_match(password) {
        return ValidationCode::MISSING_SPECIAL_CHARACTER.into();
    }

    Ok(())
}

pub(crate) fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    Handle::current().block_on(async {
        match User::find_by_username(username).one(get_database_connection()).await {
            Ok(Some(_)) => ValidationCode::USERNAME_NOT_UNIQUE.into(),
            Err(_) => ValidationCode::INTERNAL_SERVER_ERROR.into(),
            _ => Ok(()),
        }
    })
}

pub(crate) fn validate_iban(iban: &str) -> Result<(), ValidationError> {
    match iban.parse::<Iban>() {
        Ok(_) => Ok(()),
        Err(_) => ValidationCode::IBAN_INVALID.into(),
    }
}

pub(crate) fn validate_datetime_not_in_future(datetime: &OffsetDateTime) -> Result<(), ValidationError> {
    if datetime > &get_now() {
        return ValidationCode::DATETIME_IN_PAST.into();
    }

    Ok(())
}

pub(crate) fn validate_currency_exists(id: &Snowflake) -> Result<(), ValidationError> {
    Handle::current().block_on(async {
        match currency::Entity::find_by_id(id).one(get_database_connection()).await {
            Ok(Some(_)) => Ok(()),
            _ => ValidationCode::ENTITY_NOT_FOUND.into(),
        }
    })
}
