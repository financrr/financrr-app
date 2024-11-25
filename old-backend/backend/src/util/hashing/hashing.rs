use crate::api::error::api::ApiError;
use crate::util::salt::generate_salt;
use argon2::Config;

pub(crate) const DEFAULT_SALT_LENGTH: u32 = 32;

pub(crate) fn hash_string(str: &str, salt: Option<String>) -> Result<String, ApiError> {
    let salt = salt.unwrap_or(generate_salt(DEFAULT_SALT_LENGTH));
    let config = Config::rfc9106_low_mem();

    Ok(argon2::hash_encoded(str.as_bytes(), salt.as_bytes(), &config)?)
}
