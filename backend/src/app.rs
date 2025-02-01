use crate::initializers::context::ContextInitializer;
use crate::initializers::openapi::OpenApiInitializer;
use crate::initializers::opensearch::OpensearchInitializer;
use crate::initializers::path_normalization::PathNormalizationInitializer;
use crate::initializers::services::ServicesInitializer;
use crate::models::_entities::{instances, sessions};
use crate::models::external_bank_institutions;
use crate::services::custom_configs::base::CustomConfigInner;
use crate::services::instance_handler::InstanceHandlerInner;
use crate::services::Service;
use crate::utils::folder::{create_necessary_folders, STORAGE_FOLDER};
use crate::utils::routes::ExtendedAppRoutes;
use crate::workers::external_bank_institutions as external_bank_institutions_workers;
use crate::workers::session_used::SessionUsedWorker;
use crate::{controllers, models::_entities::users, tasks};
use async_trait::async_trait;
use loco_rs::cache::Cache;
use loco_rs::config::Config;
use loco_rs::storage::Storage;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    cache,
    controller::AppRoutes,
    db::truncate_table,
    environment::Environment,
    storage,
    task::Tasks,
    Result,
};
use migration::Migrator;
use mimalloc::MiMalloc;
use std::path::Path;
use tracing::{debug, info};

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

    async fn boot(mode: StartMode, environment: &Environment, config: Config) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn load_config(env: &Environment) -> Result<Config> {
        if let Err(err) = create_necessary_folders() {
            eprintln!("Could not create necessary directories. Error: {}", err);

            return Err(err);
        }

        env.load()
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(ContextInitializer),
            Box::new(PathNormalizationInitializer),
            Box::new(OpenApiInitializer),
            Box::new(ServicesInitializer),
            Box::new(OpensearchInitializer),
        ])
    }

    async fn before_run(ctx: &AppContext) -> Result<()> {
        // Load and parse CustomConfig
        let conf = CustomConfigInner::get_arc(ctx).await?;
        debug!(
            "Bank account linking configured: {}",
            conf.is_bank_data_linking_configured()
        );

        let instance_handler = InstanceHandlerInner::get_arc(ctx).await?;
        info!("Instance started with id: {}", instance_handler.get_instance_id());

        Ok(())
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
            .add_route(controllers::external_bank_institutions::routes())
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
        // External Bank institutions
        external_bank_institutions_workers::connect_worker(ctx, queue).await?;

        queue.register(SessionUsedWorker::build(ctx)).await?;

        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::sync_institutions::SyncInstitutions);
        tasks.register(tasks::check_health::CheckHealth);
        // tasks-inject (do not remove)
    }

    async fn truncate(ctx: &AppContext) -> Result<()> {
        let db = &ctx.db;
        // TODO add all other tables
        truncate_table(db, users::Entity).await?;
        truncate_table(db, sessions::Entity).await?;
        truncate_table(db, instances::Entity).await?;
        truncate_table(db, external_bank_institutions::Entity).await?;

        Ok(())
    }

    async fn seed(_ctx: &AppContext, _path: &Path) -> Result<()> {
        Ok(())
    }
}
