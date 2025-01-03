use actix_web::{get, web, HttpResponse, Responder};

use crate::api::status::dto::HealthResponse;
use crate::config::public::PublicConfig;
use crate::database::connection::{get_database_connection, get_redis_connection};
use crate::database::redis::cmd;

pub(crate) fn status_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/status").service(health).service(coffee).service(config));
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "healthy", content_type = "application/json", body = HealthResponse),
        (status = 503, description = "Service is unhealthy", content_type = "application/json", body = HealthResponse, example = json ! (
        {
        "healthy": false,
        "details": "PostgreSQL connection failed"
        }
        ))),
    path = "/api/status/health",
    tag = "Status")]
#[get("/health")]
async fn health() -> impl Responder {
    if !is_psql_reachable().await {
        return HttpResponse::ServiceUnavailable()
            .json(HealthResponse::new(false, Some("PostgreSQL connection failed".to_string())));
    }
    if !is_redis_reachable().await {
        return HttpResponse::ServiceUnavailable()
            .json(HealthResponse::new(false, Some("Redis connection failed".to_string())));
    }

    HttpResponse::Ok().json(HealthResponse::new(true, None))
}

#[utoipa::path(get,
    responses(
        (status = 418, description = "I'm a teapot"),
    ),
    path = "/api/status/coffee",
    tag = "Status")]
#[get("/coffee")]
pub(crate) async fn coffee() -> impl Responder {
    HttpResponse::ImATeapot()
}

async fn is_psql_reachable() -> bool {
    let db = get_database_connection();
    db.ping().await.is_ok()
}

async fn is_redis_reachable() -> bool {
    if get_redis_connection().await.is_err() {
        return false;
    };

    cmd(redis::cmd("PING")).await.is_ok()
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Config", content_type = "application/json", body = PublicConfig),
    ),
    path = "/api/status/config",
    tag = "Status")]
#[get("/config")]
pub(crate) async fn config() -> impl Responder {
    HttpResponse::Ok().json(PublicConfig::get())
}
