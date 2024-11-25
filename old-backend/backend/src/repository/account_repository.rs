use crate::api::error::api::ApiError;
use crate::entity::account_entity::Account;
use crate::entity::db_model::account::{Column, Entity, Model};
use crate::entity::db_model::permissions;
use crate::repository::traits::Repository;
use migration::{Expr, JoinType, Order};
use sea_orm::sea_query::IntoCondition;
use sea_orm::{ConnectionTrait, EntityName, EntityTrait, QuerySelect};
use crate::util::permissions::find_all_by_user_id;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub(crate) struct AccountRepository<'a> {
    conn: &'a dyn ConnectionTrait,
}

impl AccountRepository {
    pub(crate) async fn find_by_id_and_user_id(&self, id: i64, user_id: i64) -> Result<Option<Account>, ApiError> {
        Ok(Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                permissions::Entity::belongs_to(Self)
                    .from(permissions::Column::EntityId)
                    .to(Column::Id)
                    .on_condition(|_left, _right| {
                        Expr::col(permissions::Column::EntityType).eq(Entity.table_name()).into_condition()
                    })
                    .into(),
            )
            .filter(permissions::Column::UserId.eq(user_id))
            .filter(Column::Id.eq(id))
            .order_by(Column::Id, Order::Desc)
            .one(self.get_conn())
            .await?
            .map(Account::from)
        )
    }
}

find_all_by_user_id!(Entity, Account, AccountRepository);

impl Repository<Model, Account> for AccountRepository {
    fn new(conn: &impl ConnectionTrait) -> Self {
        AccountRepository {
            conn
        }
    }

    fn get_conn(&self) -> &dyn ConnectionTrait {
        self.conn
    }
}
