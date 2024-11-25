use argon2::Error;
use sea_orm::error::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("Failed to hash password")]
    HashingFailed(#[from] Error),
    #[error("Internal database error occurred")]
    DatabaseError(#[from] DbErr),
    #[error("An parsing error occurred")]
    ParsingError,
}
