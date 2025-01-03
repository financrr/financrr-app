use crate::api::error::api::ApiError;
use crate::entity::currency_entity::Currency;
use crate::entity::db_model::currency::{Column, Entity, Model};
use crate::repository::traits::Repository;
use crate::snowflake::snowflake_type::Snowflake;
use sea_orm::{ColumnTrait, Condition, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct CurrencyRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl CurrencyRepository {
    pub(crate) async fn find_all_with_no_user(&self) -> Result<Vec<Currency>, ApiError> {
        Ok(Entity::find()
            .filter(Column::User.is_null())
            .order_by(Column::Id, Order::Desc)
            .all(self.conn)
            .await?
            .into_iter()
            .map(Currency::from)
            .collect())
    }

    pub(crate) async fn find_all_with_user_id(&self, user_id: Snowflake) -> Result<Vec<Currency>, ApiError> {
        Ok(Entity::find()
            .filter(Column::User.eq(user_id.id))
            .order_by(Column::Id, Order::Desc)
            .all(self.conn)
            .await?
            .into_iter()
            .map(Currency::from)
            .collect())
    }

    pub(crate) async fn find_all_with_no_user_and_user_id(&self, user_id: Snowflake) -> Result<Vec<Currency>, ApiError> {
        Ok(Entity::find()
            .filter(Condition::any()
                .add(Column::User.is_null())
                .add(Column::User.eq(user_id.id))
            )
            .order_by(Column::Id, Order::Desc)
            .all(self.conn)
            .await?
            .into_iter()
            .map(Currency::from)
            .collect())
    }

    pub(crate) async fn find_by_id_with_no_user(&self, id: Snowflake) -> Result<Option<Currency>, ApiError> {
        Ok(Entity::find()
            .filter(Column::Id.eq(id.id))
            .filter(Column::User.is_null())
            .order_by(Column::Id, Order::Desc)
            .one(self.conn)
            .await?
            .map(Currency::from))
    }

    pub(crate) async fn find_by_id_with_user_id(&self, id: Snowflake, user_id: Snowflake) -> Result<Option<Currency>, ApiError> {
        Ok(Entity::find()
            .filter(Column::Id.eq(id.id))
            .filter(Column::User.eq(user_id.id))
            .order_by(Column::Id, Order::Desc)
            .one(self.conn)
            .await?
            .map(Currency::from))
    }

    pub(crate) async fn find_by_id_include_user_id(&self, id: Snowflake, user_id: Snowflake) -> Result<Option<Currency>, ApiError> {
        Ok(Entity::find()
            .filter(Column::Id.eq(id.id))
            .filter(Condition::any()
                .add(Column::User.is_null())
                .add(Column::User.eq(user_id.id))
            )
            .order_by(Column::Id, Order::Desc)
            .one(self.conn)
            .await?
            .map(Currency::from))
    }
}

impl Repository<Model, Currency> for CurrencyRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        CurrencyRepository {
            conn
        }
    }
}
