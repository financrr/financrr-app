use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{models::_entities::users, views::auth::CurrentResponse};

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
    Routes::new().prefix("/auth").add("/current", get(current))
}
