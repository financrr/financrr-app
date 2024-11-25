use crate::api::error::api::ApiError;
use crate::entity::budget_entity::Budget;
use crate::entity::db_model::budget::{Column, Model};
use crate::repository::traits::Repository;
use crate::snowflake::snowflake_type::Snowflake;
use sea_orm::ConnectionTrait;

pub(crate) struct BudgetRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl BudgetRepository {
    pub(crate) async fn find_all_by_user_id(&self, user_id: Snowflake) -> Result<Vec<Budget>, ApiError> {
        Ok(Model::find()
            .filter(Column::User.eq(user_id.id))
            .all(self.conn)
            .await?
            .into_iter()
            .map(Budget::from)
            .collect())
    }
}

impl Repository<Model, Budget> for BudgetRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        BudgetRepository {
            conn
        }
    }
}
