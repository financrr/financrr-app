use crate::api::error::api::ApiError;
use crate::entity::db_model::session::{Column, Entity, Model};
use crate::entity::session_entity::Session;
use crate::repository::traits::Repository;
use crate::snowflake::snowflake_type::Snowflake;
use futures_util::FutureExt;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct SessionRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl SessionRepository {
    pub(crate) async fn find_by_token(&self, token: &str) -> Result<Option<Session>, ApiError> {
        Ok(Entity::find()
            .filter(Column::Token.eq(token))
            .one(self.conn)
            .await?
            .map(Session::from))
    }

    pub(crate) async fn find_by_user_id(&self, user_id: Snowflake) -> Result<Vec<Session>, ApiError> {
        Ok(Entity::find()
            .filter(Column::User.eq(user_id.id))
            .all(self.conn)
            .await?
            .map(Session::from))
    }

    pub(crate) async fn find_oldest_from_user_id(&self, user_id: Snowflake) -> Result<Option<Session>, ApiError> {
        Ok(Entity::find()
            .filter(Column::User.eq(user_id))
            .order_by(Column::Id, Order::Asc)
            .one(self.conn)
            .await?
            .map(Session::from))
    }

    pub(crate) async fn delete_by_token(&self, token: &str) -> Result<u64, ApiError> {
        Ok(Entity::delete_many()
            .filter(Column::Token.eq(token))
            .exec(self.conn)
            .await?
            .rows_affected)
    }
}

impl Repository<Model, Session> for SessionRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        SessionRepository {
            conn
        }
    }
}
