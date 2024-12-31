use validator::ValidationError;

pub mod user;

pub type ValidationResult = Result<(), ValidationError>;
