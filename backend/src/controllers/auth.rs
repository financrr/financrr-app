use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{
    models::{_entities::users, users::LoginParams},
    views::auth::{CurrentResponse, LoginResponse},
};

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    let valid = user.verify_password(&params.password);

    if !valid {
        return unauthorized("unauthorized!");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let id: i64 = auth
        .claims
        .pid
        .parse()
        .map_err(|_| Error::BadRequest("invalid user id".to_string()))?;
    let user = users::Model::find_by_id(&ctx.db, id).await?;
    format::json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/auth")
        .add("/login", post(login))
        .add("/current", get(current))
}
