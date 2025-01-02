use sea_orm::ActiveValue::Set;
use sea_orm::{EntityName, EntityTrait};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use entity::{budget, transaction};
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::entity::{count, delete, find_all_paginated, find_one_or_error, insert, update};
use crate::wrapper::entity::budget::dto::BudgetDTO;
use crate::wrapper::entity::transaction::Transaction;
use crate::wrapper::entity::user::User;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{Permission, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use crate::{permission_impl, SNOWFLAKE_GENERATOR};

pub(crate) mod dto;
pub(crate) mod event_listener;
pub(crate) mod validation;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct Budget {
    #[serde(rename = "id")]
    pub(crate) snowflake: Snowflake,
    pub(crate) user_id: Phantom<User>,
    pub(crate) amount: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
}

impl Budget {
    pub(crate) async fn new(user_id: Snowflake, dto: BudgetDTO) -> Result<Self, ApiError> {
        let snowflake = SNOWFLAKE_GENERATOR.next_id()?;
        let model = budget::ActiveModel {
            id: Set(snowflake),
            user: Set(user_id.id),
            amount: Set(dto.amount),
            name: Set(dto.name),
            description: Set(dto.description),
            created_at: Set(dto.created_at),
        };

        let model = insert(model).await?;
        let budget = Self::from(model);
        budget.add_permission(user_id, Permissions::all()).await?;

        Ok(budget)
    }

    pub(crate) async fn exists(id: Snowflake) -> Result<bool, ApiError> {
        count(budget::Entity::find_by_id(id)).await.map(|count| count > 0)
    }

    pub(crate) async fn find_all_by_user_paginated(
        user_id: Snowflake,
        page_size_param: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(budget::Entity::find_all_by_user_id(user_id), page_size_param)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub(crate) async fn count_all_by_user(user_id: Snowflake) -> Result<u64, ApiError> {
        count(budget::Entity::find_all_by_user_id(user_id)).await
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(budget::Entity::delete_by_id(self.snowflake)).await
    }

    pub(crate) async fn update(self, dto: BudgetDTO) -> Result<Self, ApiError> {
        let model = budget::ActiveModel {
            id: Set(self.snowflake.id),
            user: Set(self.user_id.get_id().id),
            amount: Set(dto.amount),
            name: Set(dto.name),
            description: Set(dto.description),
            created_at: Set(dto.created_at),
        };

        Ok(update(model).await?.into())
    }

    pub(crate) async fn find_related_transactions_paginated(
        budget_id: Snowflake,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Transaction>, ApiError> {
        let transactions = find_all_paginated(transaction::Entity::find_all_by_budget_id(budget_id), page_size)
            .await?
            .into_iter()
            .map(Transaction::from)
            .collect();

        Ok(transactions)
    }

    pub(crate) async fn count_related_transactions(budget_id: Snowflake) -> Result<u64, ApiError> {
        count(transaction::Entity::find_all_by_budget_id(budget_id)).await
    }
}

permission_impl!(Budget);

impl Identifiable for Budget {
    async fn find_by_id(id: Snowflake) -> Result<Self, ApiError> {
        Ok(Self::from(find_one_or_error(budget::Entity::find_by_id(id)).await?))
    }
}

impl TableName for Budget {
    fn table_name() -> &'static str {
        budget::Entity.table_name()
    }
}

impl WrapperEntity for Budget {
    fn get_id(&self) -> Snowflake {
        self.snowflake
    }
}

impl From<budget::Model> for Budget {
    fn from(model: budget::Model) -> Self {
        Self {
            snowflake: Snowflake::from(model.id),
            user_id: Phantom::from(model.user),
            amount: model.amount,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
        }
    }
}
