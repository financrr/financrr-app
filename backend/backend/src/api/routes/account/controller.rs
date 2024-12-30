use actix_web::http::Uri;
use actix_web::web::Path;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use utility::snowflake::entity::Snowflake;

use crate::api::documentation::response::{InternalServerError, ResourceNotFound, Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, PaginatedAccount, Pagination};
use crate::wrapper::entity::account::dto::AccountDTO;
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::user::User;
use crate::wrapper::permission::{HasPermissionByIdOrError, HasPermissionOrError, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) fn account_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(get_all_accounts)
            .service(get_transactions_from_account)
            .service(create_account)
            .service(delete_account)
            .service(update_account)
            .service(get_one_account),
    );
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Accounts.", content_type = "application/json", body = PaginatedAccount),
        ValidationError,
        Unauthorized,
    ),
    params(PageSizeParam),
    security(
        ("bearer_token" = [])
    ),
    path = "/api/v1/account",
    tag = "Account")]
#[get("")]
pub(crate) async fn get_all_accounts(
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
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("account_id" = Snowflake,)),
    path = "/api/v1/account/{account_id}",
    tag = "Account")]
#[get("/{account_id}")]
pub(crate) async fn get_one_account(
    user: Phantom<User>,
    account_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let account = Account::find_by_id(account_id.into_inner()).await?;
    account.has_permission_or_error(user.get_id(), Permissions::READ).await?;

    Ok(HttpResponse::Ok().json(account))
}

#[utoipa::path(get,
    responses(
        (status = 200, description = "Successfully retrieved all Transactions.", content_type = "application/json", body = PaginatedTransaction),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("account_id" = Snowflake,), PageSizeParam),
    path = "/api/v1/account/{account_id}/transactions",
    tag = "Account")]
#[get("/{account_id}/transactions")]
pub(crate) async fn get_transactions_from_account(
    user: Phantom<User>,
    account_id: Path<Snowflake>,
    page_size: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let account_id = account_id.into_inner();
    Account::has_permission_by_id_or_error(account_id, user.get_id(), Permissions::READ).await?;

    let transactions = Account::find_transactions_by_account_id_paginated(account_id, &page_size).await?;
    let total = Account::count_transactions_by_account_id(account_id).await?;
    let pagination = Pagination::new(transactions, &page_size, total, uri);

    Ok(HttpResponse::Ok().json(pagination))
}

#[utoipa::path(post,
responses(
(status = 201, description = "Successfully created AccountDTO.", content_type = "application/json", body = Account),
Unauthorized,
ValidationError,
ResourceNotFound,
InternalServerError,
),
security(
("bearer_token" = [])
),
path = "/api/v1/account",
request_body = AccountDTO,
tag = "Account")]
#[post("")]
pub(crate) async fn create_account(user: Phantom<User>, account: AccountDTO) -> Result<impl Responder, ApiError> {
    Ok(HttpResponse::Created().json(Account::new(account, user.get_id()).await?))
}

#[utoipa::path(delete,
    responses(
        (status = 204, description = "Successfully deleted an Account."),
        Unauthorized,
        ResourceNotFound,
        InternalServerError,
    ),
    security(
        ("bearer_token" = [])
    ),
    params(("account_id" = Snowflake,)),
    path = "/api/v1/account/{account_id}",
    tag = "Account")]
#[delete("/{account_id}")]
pub(crate) async fn delete_account(
    user: Phantom<User>,
    account_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let account = Account::find_by_id(account_id.into_inner()).await?;
    account.has_permission_or_error(user.get_id(), Permissions::READ_DELETE).await?;

    account.delete().await?;

    Ok(HttpResponse::NoContent())
}

#[utoipa::path(patch,
responses(
(status = 200, description = "Successfully updated an Account.", content_type = "application/json", body = Account),
Unauthorized,
ValidationError,
ResourceNotFound,
InternalServerError,
),
security(
("bearer_token" = [])
),
params(("account_id" = Snowflake,)),
path = "/api/v1/account/{account_id}",
request_body = AccountDTO,
tag = "Account")]
#[patch("/{account_id}")]
pub(crate) async fn update_account(
    user: Phantom<User>,
    updated_account: AccountDTO,
    account_id: Path<Snowflake>,
) -> Result<impl Responder, ApiError> {
    let account = Account::find_by_id(account_id.into_inner()).await?;
    account.has_permission_or_error(user.get_id(), Permissions::READ_WRITE).await?;

    let account = account.update(updated_account).await?;

    Ok(HttpResponse::Ok().json(account))
}
