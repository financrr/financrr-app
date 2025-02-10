use crate::services::status_service::StatusService;
use crate::views::status::{HealthReport, HealthResponse, HealthStatus, StatusComponents, VersionResponse};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{debug_handler, Extension, Json};
use loco_rs::prelude::Routes;
use serde_json::{json, Value};

/// Health status of the server.
#[utoipa::path(get,
    path = "/api/v1/status/health",
    tag = "Status",
    responses(
        (status = StatusCode::OK,
            description = "Service is healthy.",
            content_type = "application/json",
            example = healthy_example,
            ,body = HealthResponse
        ),
        (status = StatusCode::SERVICE_UNAVAILABLE,
            description = "Service is unhealthy.",
            content_type = "application/json",
            example = unhealthy_example,
            ,body = HealthResponse
        )
    ),
)]
#[debug_handler]
async fn health(Extension(status_service): Extension<StatusService>) -> (StatusCode, Json<HealthResponse>) {
    let health_response = status_service.get_complete_health_response().await;

    match &health_response.status {
        HealthStatus::Healthy => (StatusCode::OK, Json(health_response)),
        HealthStatus::Unhealthy => (StatusCode::SERVICE_UNAVAILABLE, Json(health_response)),
    }
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

fn healthy_example() -> Value {
    let health_report = HealthReport {
        status: HealthStatus::Healthy,
        failed_components: None,
    };

    json!(HealthResponse {
        status: HealthStatus::Healthy,
        database_status: health_report.clone(),
        cache_status: health_report.clone(),
        storage_status: health_report.clone(),
        opensearch_status: health_report
    })
}

fn unhealthy_example() -> Value {
    json!(HealthResponse {
        status: HealthStatus::Unhealthy,
        database_status: HealthReport {
            status: HealthStatus::Unhealthy,
            failed_components: Some(vec![StatusComponents::Database]),
        },
        cache_status: HealthReport {
            status: HealthStatus::Unhealthy,
            failed_components: Some(vec![
                StatusComponents::CacheRetrieval,
                StatusComponents::StorageDeletion,
            ]),
        },
        storage_status: HealthReport {
            status: HealthStatus::Unhealthy,
            failed_components: Some(vec![StatusComponents::StorageInsertion]),
        },
        opensearch_status: HealthReport {
            status: HealthStatus::Unhealthy,
            failed_components: Some(vec![StatusComponents::Opensearch]),
        },
    })
}

pub fn routes() -> Routes {
    Routes::new().prefix("/status").add("/health", get(health))
}

pub fn non_versioned_routes() -> Routes {
    Routes::new().prefix("/status").add("/version", get(version))
}
