use actix_web::http::Uri;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use actix_web_validation::Validated;
use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::ValidationError;
use crate::api::documentation::response::{InternalServerError, ResourceNotFound, Unauthorized};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, Pagination};
use crate::wrapper::entity::session::dto::PublicSession;
use crate::wrapper::entity::session::Session;
use crate::wrapper::entity::user::dto::Credentials;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::Phantom;

pub(crate) fn session_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/session")
            .service(get_current_session)
            .service(get_all_sessions)
            .service(refresh_session)
            .service(delete_current_session)
            .service(delete_session)
            .service(delete_all_sessions)
            .service(create_session)
            .service(get_one_session),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the current Session.", content_type = "application/json", body = Session),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/session/current",
    tag = "Session"
)]
#[get("/current")]
pub(crate) async fn get_current_session(session: Session) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::Ok().json(session))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved the Session.", content_type = "application/json", body = PublicSession),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("session_id" = Snowflake,)),
    path = "/api/v1/session/{session_id}",
    tag = "Session"
)]
#[get("/{session_id}")]
pub(crate) async fn get_one_session(
    user: Phantom<User>,
    session_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let session = Session::find_by_id(session_id.into_inner()).await?;
    session.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(PublicSession::from(session)))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Sessions.", content_type = "application/json", body = PaginatedSession),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    params(PageSizeParam),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/session",
    tag = "Session"
)]
#[get("")]
pub(crate) async fn get_all_sessions(
    user: Phantom<User>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let sessions = Session::find_all_by_user_paginated(user.get_id(), &page_size).await?;
    let public_sessions: Vec<PublicSession> = sessions.into_iter().map(PublicSession::from).collect();
    let total = Session::count_all_by_user(user.get_id()).await?;

    Ok(HttpResponse::Ok().json(Pagination::new(public_sessions, &page_size, total, uri)))
}

#[utoipa::path(patch,
    responses(
        (status = 200, description = "Successfully renewed the current Session.", content_type = "application/json", body = Session),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/session/refresh",
    tag = "Session"
)]
#[patch("/refresh")]
pub(crate) async fn refresh_session(session: Session) -> Result<impl Responder, ApiError> {
    let new_session = session.renew().await?;

    Ok(HttpResponse::Ok().json(new_session))
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted the current Session."),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/session/current",
    tag = "Session"
)]
#[delete("/current")]
pub(crate) async fn delete_current_session(session: Session) -> Result<impl Responder, ApiError> {
    session.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted the Session."),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("session_id" = Snowflake,)),
    path = "/api/v1/session/{session_id}",
    tag = "Session"
)]
#[delete("/{session_id}")]
pub(crate) async fn delete_session(
    user: Phantom<User>,
    session_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let session = Session::find_by_id(session_id.into_inner()).await?;
    session.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    session.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted all Sessions."),
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/session",
    tag = "Session"
)]
#[delete("")]
pub(crate) async fn delete_all_sessions(user: Phantom<User>) -> Result<impl Responder, ApiError> {
    Session::delete_all_with_user(user.get_id()).await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(post,
    responses(
        (status = 201, description = "Successfully created a new Session.", content_type = "application/json", body = Session),
        ValidationError,
        Unauthorized,
        InternalServerError,
    ),
    path = "/api/v1/session",
    tag = "Session"
)]
#[post("")]
pub(crate) async fn create_session(credentials: Validated<Json<Credentials>>) -> Result<impl Responder, ApiError> {
    let credentials = credentials.into_inner().into_inner();
    let user = User::authenticate(&credentials).await?;
    let session = Session::new(user, credentials).await?;

    Ok(HttpResponse::Created().json(session))
}
