use crate::error::app_error::AppResult;
use axum::body::Body;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{debug_handler, Json};
use loco_rs::prelude::Response;
use loco_rs::prelude::Routes;
use utoipa::openapi::OpenApi as OpenApiStruct;
use utoipa::OpenApi;

#[utoipa::path(get,
path="/api/openapi.json",
responses(
    (status=200, description="OpenAPI JSON", content_type="application/json"),
    (status=500, description="Internal Server Error")
),
tag = "OpenAPI",
)]
#[debug_handler]
async fn openapi_json() -> AppResult<(StatusCode, Json<OpenApiStruct>)> {
    let doc = crate::initializers::openapi::ApiDocs::openapi();

    Ok((StatusCode::OK, Json(doc)))
}

#[utoipa::path(get,
path="/api/openapi.yaml",
responses(
    (status=200, description="OpenAPI YAML", content_type="application/yaml"),
    (status=500, description="Internal Server Error")
),
tag = "OpenAPI",
)]
#[debug_handler]
async fn openapi_yaml() -> AppResult<(StatusCode, Response)> {
    let doc = crate::initializers::openapi::ApiDocs::openapi().to_yaml()?;

    let mut res = Response::new(Body::new(doc));
    let headers = res.headers_mut();
    headers.insert("Content-Type", "application/yaml".parse()?);

    Ok((StatusCode::OK, res))
}

pub fn non_versioned_routes() -> Routes {
    Routes::new()
        .add("/openapi.json", get(openapi_json))
        .add("/openapi.yaml", get(openapi_yaml))
}
