use std::env::VarError;
use std::num::ParseIntError;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub enum SnowflakeGeneratorError {
    #[error("Node ID is too large")]
    NodeIdTooLarge,
    #[error("Invalid system clock")]
    InvalidSystemClock,
    #[error("Environment variable error")]
    EnvVarError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        VarError,
    ),
    #[error("Parse int error")]
    ParseIntError(
        #[from]
        #[serde_as(as = "DisplayFromStr")]
        ParseIntError,
    ),
}
