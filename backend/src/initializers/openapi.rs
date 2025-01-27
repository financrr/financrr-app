use crate::utils::type_name::type_name_only;
use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::openapi::OpenApi as OpenApiStruct;
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Status", description = "Endpoints that contain information about the health status of the server."),
        (name = "OpenAPI", description = "Endpoints for OpenAPI documentation."),
        (name = "Metrics", description = "Endpoints for prometheus metrics."),
        (name = "Session", description = "Endpoints for session management."),
        (name = "User", description = "Endpoints for user management."),
        (name = "External Bank Institutions", description = "Endpoints for external bank institutions."),
    ),
    modifiers(&ApiKeyModifier)
)]
pub struct ApiDocs;

struct ApiKeyModifier;

impl Modify for ApiKeyModifier {
    fn modify(&self, openapi: &mut OpenApiStruct) {
        let components = openapi.components.as_mut().expect("Components not found!");
        components.add_security_scheme("bearer_token", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
    }
}

pub struct OpenApiInitializer;

#[async_trait]
impl Initializer for OpenApiInitializer {
    fn name(&self) -> String {
        type_name_only::<Self>().to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> loco_rs::Result<AxumRouter> {
        let doc = ApiDocs::openapi();

        let routes = AxumRouter::new()
            .merge(SwaggerUi::new("/api/openapi/swagger-ui").url("/api/openapi/openapi.json", doc.clone()))
            .merge(Scalar::with_url("/api/openapi/scalar", doc));

        Ok(router.merge(routes))
    }
}
