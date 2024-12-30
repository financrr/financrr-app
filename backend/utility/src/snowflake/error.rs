use std::env::VarError;
use std::num::ParseIntError;

use serde::Serialize;
use thiserror::Error;

use crate::datetime::error::TimeError;

#[derive(Debug, Clone, Error, Serialize)]
pub enum SnowflakeGeneratorError {
    #[error("Node ID is too large")]
    NodeIdTooLarge,
    #[error("Invalid system clock")]
    InvalidSystemClock,
    #[error("Environment variable error")]
    EnvVarError(
        #[from]
        #[serde(with = "crate::util::serde::env_error::var_error")]
        VarError,
    ),
    #[error("Parse int error")]
    ParseIntError(
        #[from]
        #[serde(with = "crate::util::serde::number_error::parse_int_error")]
        ParseIntError,
    ),
    #[error("Time error")]
    TimeError(#[from] TimeError),
}
