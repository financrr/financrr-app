use axum::body::Body;
use axum::routing::get;
use loco_rs::prelude::{format, Response};
use loco_rs::prelude::{Result, Routes};
use loco_rs::Error;
use utoipa::OpenApi;

#[utoipa::path(get,
path="/api/v1/openapi.json",
responses(
    (status=200, description="OpenAPI JSON", content_type="application/json"),
    (status=500, description="Internal Server Error")
),
tag = "OpenAPI",
)]
async fn openapi_json() -> Result<Response> {
    let doc = crate::app::ApiDocs::openapi();

    format::json(doc)
}

#[utoipa::path(get,
path="/api/v1/openapi.yaml",
responses(
    (status=200, description="OpenAPI YAML", content_type="application/x-yaml"),
    (status=500, description="Internal Server Error")
),
tag = "OpenAPI",
)]
async fn openapi_yaml() -> Result<Response> {
    let doc = crate::app::ApiDocs::openapi().to_yaml().map_err(Error::YAML)?;

    let mut res = Response::new(Body::new(doc));
    let headers = res.headers_mut();
    headers.insert("Content-Type", "application/yaml".parse()?);

    Ok(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/openapi.json", get(openapi_json))
        .add("/openapi.yaml", get(openapi_yaml))
}
