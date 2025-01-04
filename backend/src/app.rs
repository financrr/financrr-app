use crate::initializers::openapi::OpenApiInitializer;
use crate::initializers::services::ServicesInitializer;
use crate::models::_entities::instances;
use crate::utils::folder::{create_necessary_folders, STORAGE_FOLDER};
use crate::utils::routes::ExtendedAppRoutes;
use crate::workers::session_used::SessionUsedWorker;
use crate::{controllers, models::_entities::users, tasks};
use async_trait::async_trait;
use loco_rs::cache::Cache;
use loco_rs::storage::Storage;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    cache,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    storage,
    task::Tasks,
    Result,
};
use migration::Migrator;
use mimalloc::MiMalloc;
use sea_orm::DatabaseConnection;
use std::path::Path;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
        create_necessary_folders()?;

        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(OpenApiInitializer), Box::new(ServicesInitializer)])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        // TODO fix AppRoutes somehow and remove custom ExtendedAppRoutes
        //  Currently fucked. See issue: https://github.com/loco-rs/loco/issues/1116

        ExtendedAppRoutes::empty()
            // All routes MUST be prefixed with /api
            // This helps with routing between the api and the frontend
            .prefix("/api")
            .add_route(controllers::status::non_versioned_routes())
            .add_route(controllers::openapi::non_versioned_routes())
            .prefix("/v1")
            .add_route(controllers::user::routes())
            .add_route(controllers::session::routes())
            .add_route(controllers::status::routes())
            .into()
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        Ok(AppContext {
            cache: Cache::new(cache::drivers::inmem::new()).into(),
            storage: Storage::single(storage::drivers::local::new_with_prefix(STORAGE_FOLDER)?).into(),
            ..ctx
        })
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(SessionUsedWorker::build(ctx)).await?;
        Ok(())
    }
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
        // tasks-inject (do not remove)
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        // TODO add all other tables
        truncate_table(db, users::Entity).await?;
        truncate_table(db, instances::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}
