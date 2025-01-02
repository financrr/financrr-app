use crate::repository::traits::Repository;
use crate::repository::transaction::transaction_template_repository::TransactionTemplateRepository;
use actix_web::web;
use sea_orm::ConnectionTrait;
use web::Data;
use crate::repository::transaction::recurring_transaction_repository::RecurringTransactionRepository;

pub(crate) mod transaction_template_repository;
mod recurring_transaction_repository;
mod transaction_repository;

pub(crate) fn configure_repository_app_data(cfg: &mut web::ServiceConfig, conn: &impl ConnectionTrait) {
    cfg
        .app_data(Data::new(TransactionTemplateRepository::new(conn)))
        .app_data(Data::new(RecurringTransactionRepository::new(conn)));
}
