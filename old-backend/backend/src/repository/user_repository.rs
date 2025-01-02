use crate::api::error::api::ApiError;
use crate::entity::db_model::user::{ActiveModel, Column, Entity, Model};
use crate::entity::user_entity::User;
use crate::repository::traits::Repository;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct UserRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl UserRepository {
    pub(crate) async fn insert(&self, user: ActiveModel) -> Result<User, ApiError> {
        let model = user.insert(self.conn).await?;

        Ok(User::from(model))
    }

    pub(crate) async fn find_by_username(&self, username: &str) -> Result<Option<User>, ApiError> {
        let model_opt = Entity::find()
            .filter(Column::Username.eq(username.to_string()))
            .order_by(Column::Id, Order::Desc)
            .one(self.conn)
            .await?;

        Ok(model_opt.map(User::from))
    }
}

impl Repository<Model, User> for UserRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        UserRepository {
            conn
        }
    }
}
