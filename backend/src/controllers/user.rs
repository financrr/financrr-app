use crate::error::app_error::AppError;
use axum::extract::State;
use axum::routing::post;
use axum::{debug_handler, Json};
use loco_rs::app::AppContext;
use loco_rs::prelude::Routes;

#[utoipa::path(post, path = "/api/v1/users/register", tag = "User")]
#[debug_handler]
async fn register(State(_ctx): State<AppContext>) -> Result<Json<()>, AppError> {
    Ok(Json(()))
}

pub fn routes() -> Routes {
    Routes::new().prefix("/users").add("/register", post(register))
}
