use actix_web::HttpRequest;

use crate::api::error::api::ApiError;

pub(crate) fn extract_bearer_token(req: &HttpRequest) -> Result<String, ApiError> {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            let parts: Vec<&str> = header.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .ok_or(ApiError::NoTOkenProvided())
}
