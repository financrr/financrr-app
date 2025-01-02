use crate::api::error::api::ApiError;
use crate::entity::db_model::transaction::{Column, Entity, Model};
use crate::entity::transaction::transaction_entity::Transaction;
use crate::repository::traits::Repository;
use crate::snowflake::snowflake_type::Snowflake;
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use tracing::error;

pub(crate) struct TransactionRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl TransactionRepository {
    pub(crate) async fn find_all_by_account_id(&self, account_id: Snowflake) -> Result<Vec<Transaction>, ApiError> {
        Ok(Entity::find()
            .filter(Condition::any()
                .add(Column::Source.eq(account_id.id))
                .add(Column::Destination.eq(account_id.id))
            )
            .order_by(Column::Id, Order::Desc)
            .all(self.conn)
            .await?
            .iter()
            .filter_map(|model| match Transaction::try_from(model) {
                Ok(transaction) => Some(transaction),
                Err(e) => {
                    error!("Failed to convert model to transaction: {:?}", e);
                    None
                }
            })
            .collect())
    }

    pub(crate) async fn find_all_by_budget_id(&self, budget_id: Snowflake) -> Result<Vec<Transaction>, ApiError> {
        Ok(Entity::find()
            .filter(Column::Budget.eq(budget_id.id))
            .order_by(Column::Id, Order::Desc)
            .all(self.conn)
            .await?
            .iter()
            .filter_map(|model| match Transaction::try_from(model) {
                Ok(transaction) => Some(transaction),
                Err(e) => {
                    error!("Failed to convert model to transaction: {:?}", e);
                    None
                }
            })
            .collect())
    }
}

impl Repository<Model, Transaction> for TransactionRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        TransactionRepository {
            conn
        }
    }
}
