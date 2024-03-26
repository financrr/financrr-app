pub(crate) mod documentation;
pub(crate) mod error;
pub(crate) mod pagination;
pub(crate) mod routes;
pub(crate) mod status;

// Use this once impl Trait is stable (https://github.com/rust-lang/rust/issues/63063)
//pub(crate) type ApiResponse = Result<impl Responder, ApiError>;
