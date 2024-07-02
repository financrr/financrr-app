use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{get, web, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::api::error::api::ApiError;
use crate::ApiDoc;

pub(crate) fn configure_openapi(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/openapi")
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi/docs.json", ApiDoc::openapi()))
            .service(
                web::scope("")
                    .wrap(NormalizePath::new(TrailingSlash::Trim))
                    .service(Scalar::with_url("/scalar", ApiDoc::openapi()))
                    .service(openapi_docs_json)
                    .service(openapi_docs_yaml),
            ),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the OpenAPI documentation in JSON format.", content_type = "application/json"),
    ),
    path = "/openapi/docs.json",
    tag = "OpenAPI"
)]
#[get("/docs.json")]
pub(crate) async fn openapi_docs_json() -> impl Responder {
    let openapi = ApiDoc::openapi();

    HttpResponse::Ok().json(openapi)
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the OpenAPI documentation in YAML format.", content_type = "application/yaml"),
        (status = 500, description = "Failed to serialize the OpenAPI documentation.")
    ),
    path = "/openapi/docs.yaml",
    tag = "OpenAPI"
)]
#[get("/docs.yaml")]
pub(crate) async fn openapi_docs_yaml() -> Result<impl Responder, ApiError> {
    let openapi = ApiDoc::openapi();
    let yaml = serde_yml::to_string(&openapi)?;

    Ok(HttpResponse::Ok().content_type("application/yaml").body(yaml))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Swagger UI.", content_type = "text/html"),
    ),
    path = "/openapi/swagger-ui/",
    tag = "OpenAPI"
)]
#[allow(unused)]
pub(crate) async fn swagger_ui_route() {}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Scalar UI.", content_type = "text/html"),
    ),
    path = "/openapi/scalar",
    tag = "OpenAPI"
)]
#[allow(unused)]
pub(crate) async fn scalar_ui_route() {}
