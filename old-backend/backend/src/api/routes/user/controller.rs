use actix_web::{get, post, web, HttpResponse, Responder};

use crate::api::documentation::response::{InternalServerError, ResourceNotFound, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::wrapper::entity::user::dto::UserRegistration;
use crate::wrapper::entity::user::User;
use crate::wrapper::types::phantom::Phantom;

pub(crate) fn user_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(me).service(register));
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved your own User.", content_type = "application/json", body = User),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/user/@me",
    tag = "User"
)]
#[get("/@me")]
pub(crate) async fn me(mut user: Phantom<User>) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::Ok().json(user.get_inner().await?))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully registered.", content_type = "application/json", body = User),
(status = 409, description = "User is signed in."),
ValidationError,
InternalServerError,
),
path = "/api/v1/user/register",
request_body = UserRegistration,
tag = "User"
)]
#[post("/register")]
pub(crate) async fn register(registration: UserRegistration) -> Result<impl Responder, ApiError> {
    let user = User::register(registration).await?;

    Ok(HttpResponse::Created().json(user))
}
