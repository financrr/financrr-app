use std::io::Result;
use std::sync::OnceLock;
use std::time::Duration;

use actix_cors::Cors;
use actix_limitation::{Limiter, RateLimiter};
use actix_web::middleware::{Compress, DefaultHeaders, NormalizePath, TrailingSlash};
use actix_web::web::Data;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use actix_web_validator5::{Error, JsonConfig, PathConfig, QueryConfig};
use dotenvy::dotenv;
use redis::Client;
use sea_orm::DatabaseConnection;
use tracing::info;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::openapi::OpenApi as OpenApiStruct;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

use entity::utility::loading::load_schema;
use migration::Migrator;
use migration::MigratorTrait;

use crate::api::error::api::ApiError;
use crate::api::routes::account::controller::account_controller;
use crate::api::routes::budget::controller::budget_controller;
use crate::api::routes::currency::controller::currency_controller;
use crate::api::routes::session::controller::session_controller;
use crate::api::routes::transaction::controller::transaction_controller;
use crate::api::routes::user::controller::user_controller;
use crate::api::status::controller::status_controller;
use crate::config::{logger, Config};
use crate::database::connection::{create_redis_client, establish_database_connection, get_database_connection};
use crate::database::redis::clear_redis;
use crate::util::panic::install_panic_hook;
use crate::util::validation::ValidationErrorJsonPayload;
use crate::wrapper::entity::session::Session;
use crate::wrapper::entity::start_wrapper;
use crate::wrapper::permission::cleanup::schedule_clean_up_task;

pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod database;
pub(crate) mod event;
pub(crate) mod scheduling;
pub(crate) mod util;
pub(crate) mod wrapper;

pub(crate) static DB: OnceLock<DatabaseConnection> = OnceLock::new();
pub(crate) static REDIS: OnceLock<Client> = OnceLock::new();
pub(crate) static CONFIG: OnceLock<Config> = OnceLock::new();

#[utoipauto(paths = "./backend/src")]
#[derive(OpenApi)]
#[openapi(
    schemas(
        crate::wrapper::permission::Permissions,
    ),
    tags(
        (name = "Status", description = "Endpoints that contain information about the health status of the server."),
        (name = "Metrics", description = "Endpoints for prometheus metrics."),
        (name = "Session", description = "Endpoints for session management."),
        (name = "User", description = "Endpoints for user management."),
        (name = "Account", description = "Endpoints for finance-account management."),
        (name = "Currency", description = "Endpoints for currency management."),
        (name = "Transaction", description = "Endpoints for transaction management."),
        (name = "Transaction-Template", description = "Endpoints for transaction template management."),
        (name = "Recurring-Transaction", description = "Endpoints for recurring transaction management."),
        (name = "Budget", description = "Endpoints for budget management.")
    ),
    modifiers(& BearerTokenAddon)
)]
pub(crate) struct ApiDoc;

pub(crate) struct BearerTokenAddon;

impl Modify for BearerTokenAddon {
    fn modify(&self, openapi: &mut OpenApiStruct) {
        let components = openapi.components.as_mut().expect("Components not found!");
        components.add_security_scheme("bearer_token", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _guard = logger::configure(); // We need to keep the guard alive to keep the logger running.

    info!("Starting up...");

    info!("Installing panic hook...");
    install_panic_hook();

    info!("Loading configuration...");
    CONFIG.set(Config::load()).expect("Could not load config!");

    info!("[*] Establishing database connection...");
    DB.set(establish_database_connection().await).expect("Could not set database!");
    info!("[*] Establishing redis connection...");
    REDIS.set(create_redis_client().await).expect("Could not set redis!");

    info!("[*] Cleaning redis...");
    clear_redis().await.expect("Could not clear redis!");

    info!("[*] Loading schema...");
    load_schema(get_database_connection()).await;

    info!("[*] Migrating database...");
    Migrator::up(get_database_connection(), None).await.expect("Could not migrate database!");

    info!("[*] Loading sessions...");
    Session::init().await.expect("Could not load sessions!");

    info!("[*] Starting up event system...");
    event::init();

    info!("[*] Scheduling clean up task...");
    schedule_clean_up_task();

    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();

    info!("\t[*] Initializing rate limiter...");
    let limiter = Data::new(build_rate_limiter());

    info!("[*] Initializing prometheus metrics...");
    let prometheus_metrics = build_prometheus_metrics();

    info!("[*] Starting wrapper...");
    start_wrapper().await;

    info!("Starting server... Listening on: {}", Config::get_config().address);

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(build_cors())
            .wrap(prometheus_metrics.clone())
            .app_data(JsonConfig::default().error_handler(|err, _| handle_validation_error(err).into()))
            .app_data(QueryConfig::default().error_handler(|err, _| handle_validation_error(err).into()))
            .app_data(PathConfig::default().error_handler(|err, _| handle_validation_error(err).into()))
            .app_data(limiter.clone())
            .configure(configure_api)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind(&Config::get_config().address)?
    .run()
    .await
}

fn handle_validation_error(err: Error) -> ApiError {
    let json_error = match &err {
        Error::Validate(error) => ValidationErrorJsonPayload::from(error),
        _ => ValidationErrorJsonPayload {
            message: err.to_string(),
            fields: Vec::new(),
        },
    };

    ApiError::from(json_error)
}

fn configure_api(cfg: &mut web::ServiceConfig) {
    let default_headers =
        DefaultHeaders::new().add(("Content-Type", "application/json")).add(("Accept", "application/json"));

    cfg.service(
        web::scope("/api")
            .wrap(RateLimiter::default())
            .wrap(default_headers)
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .configure(configure_api_v1)
            .configure(status_controller),
    );
}

fn configure_api_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(user_controller)
            .configure(account_controller)
            .configure(currency_controller)
            .configure(transaction_controller)
            .configure(budget_controller)
            .configure(session_controller),
    );
}

fn build_cors() -> Cors {
    let cors_config = &Config::get_config().cors;
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
        .max_age(3600);

    if cors_config.allow_any_origin {
        cors = cors.allow_any_origin();
    } else {
        cors = cors.allowed_origin_fn(move |origin, _req_head| {
            cors_config.allowed_origins.iter().any(|allowed_origin| {
                if allowed_origin == "*" {
                    return true;
                }
                origin == allowed_origin
            })
        });
    }

    cors
}

fn build_rate_limiter() -> Limiter {
    Limiter::builder(Config::get_config().cache.get_url())
        .key_by(|req| Some(req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".to_string())))
        .limit(Config::get_config().rate_limiter.limit as usize)
        .period(Duration::from_secs(Config::get_config().rate_limiter.duration_seconds)) // 60 minutes
        .build()
        .expect("Could not build rate limiter!")
}

fn build_prometheus_metrics() -> PrometheusMetrics {
    PrometheusMetricsBuilder::new("api").endpoint("/metrics").build().expect("Could not build prometheus metrics!")
}
