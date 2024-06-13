use std::borrow::Cow;
use std::collections::HashMap;

use actix_web_validator5::error::flatten_errors;
use iban::Iban;
use lazy_regex::regex;
use sea_orm::EntityTrait;
use serde::Serialize;
use serde_json::Value;
use time::OffsetDateTime;
use tracing::error;
use utoipa::ToSchema;
use validator::ValidationError;

use entity::currency;
use entity::prelude::User;
use utility::datetime::get_now;

use crate::database::entity::find_one;

pub(crate) const MIN_PASSWORD_LENGTH: usize = 8;
pub(crate) const MAX_PASSWORD_LENGTH: usize = 128;

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct ValidationErrorJsonPayload {
    pub(crate) message: String,
    pub(crate) fields: Vec<FieldError>,
}

#[derive(Debug, Serialize, ToSchema)]
pub(crate) struct FieldError {
    pub(crate) field_name: String,
    pub(crate) code: String,
    pub(crate) params: HashMap<String, Value>,
}

impl From<&validator::ValidationErrors> for ValidationErrorJsonPayload {
    fn from(error: &validator::ValidationErrors) -> Self {
        let errors = flatten_errors(error);
        let mut field_errors: Vec<FieldError> = Vec::new();
        for (_, field, error) in errors {
            field_errors.push(map_field_error(field.as_str(), error))
        }

        Self {
            message: "Validation error".to_owned(),
            fields: field_errors,
        }
    }
}

impl From<ValidationError> for ValidationErrorJsonPayload {
    fn from(error: ValidationError) -> Self {
        let mut field_errors: Vec<FieldError> = Vec::new();
        field_errors.insert(0, map_field_error("", &error));
        Self {
            message: "Validation error".to_owned(),
            fields: field_errors,
        }
    }
}

fn map_field_error(field: &str, error: &ValidationError) -> FieldError {
    FieldError {
        field_name: field.to_owned(),
        code: error.code.clone().into_owned(),
        params: error.params.clone().into_iter().map(|(key, value)| (key.into_owned(), value)).collect(),
    }
}

pub(crate) fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut error = ValidationError::new("Password is invalid");
    if password.len() < MIN_PASSWORD_LENGTH {
        error.add_param(Cow::from("min"), &MIN_PASSWORD_LENGTH);
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        error.add_param(Cow::from("max"), &MAX_PASSWORD_LENGTH);
    }

    let uppercase = regex!("[A-Z]");
    if !uppercase.is_match(password) {
        error.add_param(Cow::from("uppercase"), &"Must contain at least one uppercase letter");
    }

    let lowercase = regex!("[a-z]");
    if !lowercase.is_match(password) {
        error.add_param(Cow::from("lowercase"), &"Must contain at least one lowercase letter");
    }

    let digit = regex!(r"\d");
    if !digit.is_match(password) {
        error.add_param(Cow::from("digit"), &"Must contain at least one digit");
    }

    let special_character = regex!("[€§!@#$%^&*(),.?\":{}|<>]");
    if !special_character.is_match(password) {
        error.add_param(Cow::from("special_character"), &"Must contain at least one special character");
    }

    if !error.params.is_empty() {
        return Err(error);
    }

    Ok(())
}

pub(crate) async fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    match find_one(User::find_by_username(username)).await {
        Ok(Some(_)) => Err(ValidationError::new("Username is not unique")),
        Err(_) => Err(ValidationError::new("Internal server error")),
        _ => Ok(()),
    }
}

pub(crate) fn validate_iban(iban: &str) -> Result<(), ValidationError> {
    match iban.parse::<Iban>() {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("IBAN is invalid")),
    }
}

pub(crate) fn validate_datetime_not_in_future(datetime: &OffsetDateTime) -> Result<(), ValidationError> {
    let now = get_now().map_err(|err| {
        error!("Failed to get current time: {:?}", err);
        ValidationError::new(concat!("Internal Server Error"))
    })?;

    if datetime > &now {
        return Err(ValidationError::new("Date and time cannot be in the future"));
    }

    Ok(())
}

pub(crate) async fn validate_currency_exists(id: i64) -> Result<(), ValidationError> {
    match find_one(currency::Entity::find_by_id(id)).await {
        Ok(Some(_)) => Ok(()),
        _ => Err(ValidationError::new("Currency does not exist")),
    }
}
