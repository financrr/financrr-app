use crate::entity::db_model::recurring_transaction::Model;
use crate::entity::transaction::recurring_transaction_entity::RecurringTransaction;
use crate::repository::traits::Repository;
use sea_orm::ConnectionTrait;

pub(crate) struct RecurringTransactionRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl Repository<Model, RecurringTransaction> for RecurringTransactionRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        RecurringTransactionRepository {
            conn
        }
    }
}
