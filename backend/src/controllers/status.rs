use crate::services::status_service::StatusService;
use crate::views::status::{HealthResponse, VersionResponse};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{debug_handler, Extension, Json};
use loco_rs::prelude::Routes;

/// Health status of the server.
#[utoipa::path(get,
    path = "/api/v1/status/health",
    tag = "Status",
    responses(
        (status = StatusCode::OK, description = "Health status of the server.", content_type="application/json", body = HealthResponse),
    ),
)]
#[debug_handler]
async fn health(Extension(status_service): Extension<StatusService>) -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(status_service.get_complete_health_response().await),
    )
}

/// Version of the API.
///
/// This endpoint is used to get the version of the API.
/// This endpoint is not versioned so we try to keep it as simple as possible and not
/// include breaking changes.
#[utoipa::path(get,
    path = "/api/status/version",
    tag = "Status",
    responses(
        (status = StatusCode::OK, description = "Version of the API.", content_type="application/json", body = VersionResponse),
    ),
)]
#[debug_handler]
async fn version() -> (StatusCode, Json<VersionResponse>) {
    (StatusCode::OK, Json(VersionResponse::default()))
}

pub fn routes() -> Routes {
    Routes::new().prefix("/status").add("/health", get(health))
}

pub fn non_versioned_routes() -> Routes {
    Routes::new().prefix("/status").add("/version", get(version))
}
