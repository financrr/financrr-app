use crate::repository::account_repository::AccountRepository;
use crate::repository::budget_repository::BudgetRepository;
use crate::repository::currency_repository::CurrencyRepository;
use crate::repository::permissions_repository::PermissionsRepository;
use crate::repository::session_repository::SessionRepository;
use crate::repository::traits::Repository;
use crate::repository::user_repository::UserRepository;
use actix_web::web;
use actix_web::web::Data;
use sea_orm::ConnectionTrait;

pub(crate) mod traits;
pub(crate) mod user_repository;
pub(crate) mod session_repository;
pub(crate) mod permissions_repository;
pub(crate) mod currency_repository;
pub(crate) mod account_repository;
pub(crate) mod budget_repository;
pub(crate) mod transaction;

pub(crate) fn configure_repository_app_data(cfg: &mut web::ServiceConfig, conn: &impl ConnectionTrait) {
    cfg
        .app_data(Data::new(UserRepository::new(conn)))
        .app_data(Data::new(SessionRepository::new(conn)))
        .app_data(Data::new(PermissionsRepository::new(conn)))
        .app_data(Data::new(CurrencyRepository::new(conn)))
        .app_data(Data::new(AccountRepository::new(conn)))
        .app_data(Data::new(BudgetRepository::new(conn)))
        .configure(|cfg| transaction::configure_repository_app_data(cfg, conn));
}
