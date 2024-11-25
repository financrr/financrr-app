use actix_web::http::Uri;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::Data;
use crate::api::documentation::response::{Unauthorized, ValidationError};
use crate::api::error::api::ApiError;
use crate::api::pagination::{PageSizeParam, PaginatedAccount};
use crate::entity::user_entity::User;
use crate::repository::account_repository::AccountRepository;
use crate::util::phantom::Phantom;

pub(crate) fn configure_account_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(get_all_accounts)
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
    account_repository: Data<AccountRepository>,
    page_size_param: PageSizeParam,
    uri: Uri,
) -> Result<impl Responder, ApiError> {
    let total = account_repository.count_all_by_user_id(user.get_snowflake()).await?;
    let result = account_repository.find_all_by_user_id(user.get_snowflake()).await?;

    Ok(HttpResponse::Ok().json(PaginatedAccount::new(result, &page_size_param, total, uri)))
}