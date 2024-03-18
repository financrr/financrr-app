use actix_web::http::Uri;
use actix_web::web::Path;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use crate::api::documentation::response::{InternalServerError, ResourceNotFound, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, PaginatedAccount};
use crate::wrapper::entity::account::dto::AccountDTO;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::Phantom;

pub(crate) fn account_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account").service(get_one).service(get_all).service(create).service(delete).service(update),
    );
}

#[utoipa::path(get,
responses(
(status = 200, description = "Successfully retrieved all Accounts.", content_type = "application/json", body = PaginatedAccount),
(status = 400, response = ValidationError),
(status = 401, response = Unauthorized),
),
params(PageSizeParam),
security(
("bearer_token" = [])
),
path = "/api/v1/account/?page={page}&size={size}",
tag = "Account")]
#[get("")]
pub(crate) async fn get_all(
    user: Phantom<User>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let total = Account::count_all_by_user(user.get_id()).await?;
    let result = Account::find_all_by_user(user.get_id()).await?;

    Ok(HttpResponse::Ok().json(PaginatedAccount::new(result, &page_size, total, uri)))
}

#[utoipa::path(get,
responses(
(status = 200, description = "Successfully retrieved Account.", content_type = "application/json", body = Account),
(status = 401, response = Unauthorized),
(status = 404, response = ResourceNotFound),
(status = 500, response = InternalServerError)
),
security(
("bearer_token" = [])
),
path = "/api/v1/account/{account_id}",
tag = "Account")]
#[get("/{account_id}")]
pub(crate) async fn get_one(user: Phantom<User>, account_id: Path<i32>) -> Result<impl Responder, ApiError> {
    let account = Account::find_by_id(account_id.into_inner()).await?;
    account.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(account))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully created AccountDTO.", content_type = "application/json", body = Account),
(status = 401, response = Unauthorized),
(status = 400, response = ValidationError),
(status = 404, response = ResourceNotFound),
(status = 500, response = InternalServerError)
),
security(
("bearer_token" = [])
),
path = "/api/v1/account",
request_body = AccountDTO,
tag = "Account")]
#[post("")]
pub(crate) async fn create(user: Phantom<User>, account: AccountDTO) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::Created().json(Account::new(account, user.get_id()).await?))
}

#[utoipa::path(delete,
responses(
(status = 204, description = "Successfully deleted an Account."),
(status = 401, response = Unauthorized),
(status = 404, response = ResourceNotFound),
(status = 500, response = InternalServerError)
),
security(
("bearer_token" = [])
),
path = "/api/v1/account/{account_id}",
tag = "Account")]
#[delete("/{account_id}")]
pub(crate) async fn delete(user: Phantom<User>, account_id: Path<i32>) -> Result<impl Responder, ApiError> {
    let account = Account::find_by_id(account_id.into_inner()).await?;
    account.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    account.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
responses(
(status = 200, description = "Successfully updated an Account.", content_type = "application/json", body = Account),
(status = 401, response = Unauthorized),
(status = 400, response = ValidationError),
(status = 404, response = ResourceNotFound),
(status = 500, response = InternalServerError)
),
security(
("bearer_token" = [])
),
path = "/api/v1/account/{account_id}",
request_body = AccountDTO,
tag = "Account")]
#[patch("/{account_id}")]
pub(crate) async fn update(
    user: Phantom<User>,
    updated_account: AccountDTO,
    account_id: Path<i32>,
) -> Result<impl Responder, ApiError> {
    let account = Account::find_by_id(account_id.into_inner()).await?;
    account.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    let account = account.update(updated_account).await?;

    Ok(HttpResponse::Ok().json(account))
}
