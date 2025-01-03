macro_rules! find_all_by_user_id {
    ($entity:ty, $model:ty, $repository:ty) => {
        impl $repository {
            pub(crate) async fn find_all_by_user_id(&self, user: crate::snowflake::snowflake_type::Snowflake) -> Result<Vec<$model>, ApiError> {
                use sea_orm::{QueryOrder, QuerySelect, EntityTrait, ColumnTrait};

                Ok(
                    <$entity as EntityTrait>::find()
                        .join_rev(
                            sea_orm::JoinType::InnerJoin,
                            crate::entity::db_model::permissions::Entity::belongs_to(<$entity as EntityTrait>::default())
                                .from(crate::entity::db_model::permissions::Column::EntityId)
                                .to(<$entity as EntityTrait>::default().primary_key())
                                .on_condition(|_left, _right| {
                                    sea_orm::entity::prelude::Expr::col(crate::entity::db_model::permissions::Column::EntityType)
                                        .eq(<$entity as EntityTrait>::default().table_name())
                                        .into_condition()
                                })
                                .into(),
                        )
                        .filter(crate::entity::db_model::permissions::Column::UserId.eq(user.id))
                        .order_by(crate::entity::db_model::permissions::Column::EntityId, sea_orm::Order::Desc)
                        .all(self.get_conn())
                        .await?
                        .into_iter()
                        .map(<$model>::from)
                        .collect()
                )
            }

            pub(crate) async fn count_all_by_user_id(&self, user: crate::snowflake::snowflake_type::Snowflake) -> Result<u64, ApiError> {
                use sea_orm::{QueryOrder, QuerySelect, EntityTrait, ColumnTrait};

                Ok(
                    <$entity as EntityTrait>::find()
                        .join_rev(
                            sea_orm::JoinType::InnerJoin,
                            crate::entity::db_model::permissions::Entity::belongs_to(<$entity as EntityTrait>::default())
                                .from(crate::entity::db_model::permissions::Column::EntityId)
                                .to(<$entity as EntityTrait>::default().primary_key())
                                .on_condition(|_left, _right| {
                                    sea_orm::entity::prelude::Expr::col(crate::entity::db_model::permissions::Column::EntityType)
                                        .eq(<$entity as EntityTrait>::default().table_name())
                                        .into_condition()
                                })
                                .into(),
                        )
                        .filter(crate::entity::db_model::permissions::Column::UserId.eq(user.id))
                        .order_by(crate::entity::db_model::permissions::Column::EntityId, sea_orm::Order::Desc)
                        .count(self.get_conn())
                        .await?
                )
            }
        }
    };
}

pub(crate) use find_all_by_user_id;
