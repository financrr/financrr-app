use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::entity::db_model::account;
use crate::entity::db_model::permissions::{Column, Entity, Model};
use crate::entity::permissions_entity::Permissions;
use crate::repository::traits::Repository;
use crate::snowflake::snowflake_type::Snowflake;
use futures_util::StreamExt;
use sea_orm::{ConnectionTrait, EntityName, EntityTrait, Order, PaginatorTrait, QueryOrder};

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct PermissionsRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl PermissionsRepository {
    pub(crate) async fn find_all_paginated(&self, page_size: &PageSizeParam) -> Result<Vec<Permissions>, ApiError> {
        Ok(Entity::find()
            .order_by(Column::EntityId, QueryOrder::Desc)
            .paginate(self.conn, page_size.limit)
            .fetch_page(page_size.page - 1)
            .await?
            .into_iter()
            .map(Permissions::from)
            .collect())
    }

    pub(crate) async fn count_all(&self) -> Result<u64, ApiError> {
        Entity::find().count(self.conn).await.map_err(ApiError::from)
    }

    pub(crate) async fn find_permission(&self, user_id: Snowflake, entity_type: &str, entity_id: Snowflake) -> Result<Option<Permissions>, ApiError> {
        Ok(Entity::find()
            .filter(Column::UserId.eq(user_id.id))
            .filter(Column::EntityType.eq(entity_type))
            .filter(Column::EntityId.eq(entity_id.id))
            .one(self.conn)
            .await?
            .map(Permissions::from))
    }

    pub(crate) async fn find_all_accounts_for_user_id(&self, user_id: Snowflake) -> Result<Vec<Permissions>, ApiError> {
        Ok(Entity::find()
            .filter(Column::UserId.eq(user_id.id))
            .filter(Column::EntityType.eq(account::Entity.table_name().to_string()))
            .order_by(Column::EntityId, Order::Desc)
            .all(self.conn)
            .await?
            .into_iter()
            .map(Permissions::from)
            .collect())
    }

    pub(crate) async fn find_account_by_id_and_user_id(&self, account_id: Snowflake, user_id: Snowflake) -> Result<Option<Permissions>, ApiError> {
        Ok(Entity::find()
            .filter(Column::UserId.eq(user_id.id))
            .filter(Column::EntityId.eq(account_id.id))
            .filter(Column::EntityType.eq(account::Entity.table_name().to_string()))
            .order_by(Column::EntityId, Order::Desc)
            .one(self.conn)
            .await?
            .map(Permissions::from))
    }

    pub(crate) async fn find_all_by_type_and_id(&self, entity_type: &str, entity_id: Snowflake) -> Result<Vec<Permissions>, ApiError> {
        Ok(Entity::find()
            .filter(Column::EntityType.eq(entity_type))
            .filter(Column::EntityId.eq(entity_id.id))
            .order_by(Column::EntityId, Order::Desc)
            .all(self.conn)
            .await?
            .into_iter()
            .map(Permissions::from)
            .collect())
    }
}

impl Repository<Model, Permissions> for PermissionsRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        PermissionsRepository {
            conn
        }
    }
}
