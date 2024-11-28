use crate::services::configure_services;
use crate::{controllers, models::_entities::users, tasks, workers::downloader::DownloadWorker};
use async_trait::async_trait;
use axum::routing::Router as AxumRouter;
use loco_rs::cache::Cache;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    cache,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use mimalloc::MiMalloc;
use sea_orm::DatabaseConnection;
use std::path::Path;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::openapi::OpenApi as OpenApiStruct;
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[utoipauto]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Status", description = "Endpoints that contain information about the health status of the server."),
        (name = "OpenAPI", description = "Endpoints for OpenAPI documentation."),
        (name = "Metrics", description = "Endpoints for prometheus metrics."),
        (name = "Session", description = "Endpoints for session management."),
        (name = "User", description = "Endpoints for user management.")
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

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA").or(option_env!("GITHUB_SHA")).unwrap_or("dev")
        )
    }

    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .prefix("/api/v1")
            .add_route(controllers::auth::routes())
            .add_route(controllers::openapi::routes())
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        Ok(AppContext {
            cache: Cache::new(cache::drivers::inmem::new()).into(),
            ..ctx
        })
    }

    async fn after_routes(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let service_injected_router = configure_services(router, ctx).await?;

        Ok(service_injected_router.merge(open_api_routes()))
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}

fn open_api_routes() -> AxumRouter {
    let doc = ApiDocs::openapi();

    AxumRouter::new()
        .merge(SwaggerUi::new("/api/v1/openapi/swagger-ui").url("/api/v1/openapi/openapi.json", doc.clone()))
        .merge(Scalar::with_url("/api/v1/openapi/scalar", doc))
}
