use sea_orm::{EntityName, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use dto::TransactionTemplateDTO;
use entity::transaction_template;
use entity::utility::time::get_now;
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::entity::{count, delete, find_all_paginated, find_one_or_error, insert, update};
use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::budget::Budget;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{Permission, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use crate::{permission_impl, SNOWFLAKE_GENERATOR};

pub(crate) mod dto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct TransactionTemplate {
    #[serde(rename = "id")]
    pub(crate) snowflake: Snowflake,
    pub(crate) source_id: Option<Phantom<Account>>,
    pub(crate) destination_id: Option<Phantom<Account>>,
    pub(crate) amount: i64,
    pub(crate) currency_id: Phantom<Currency>,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) budget_id: Option<Phantom<Budget>>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
}

impl TransactionTemplate {
    pub(crate) async fn new(dto: TransactionTemplateDTO, user_id: Snowflake) -> Result<Self, ApiError> {
        let snowflake = SNOWFLAKE_GENERATOR.next_id()?;
        let active_model = transaction_template::ActiveModel {
            id: Set(snowflake),
            source: Set(dto.source_id.map(|source| source.get_id().id)),
            destination: Set(dto.destination_id.map(|destination| destination.get_id().id)),
            amount: Set(dto.amount),
            currency: Set(dto.currency_id.get_id().id),
            name: Set(dto.name),
            description: Set(dto.description),
            budget: Set(dto.budget_id.map(|budget| budget.get_id().id)),
            created_at: Set(get_now()),
        };
        let model = insert(active_model).await?;
        let template = Self::from(model);

        //grant permission
        template.add_permission(user_id, Permissions::all()).await?;

        Ok(template)
    }

    pub(crate) async fn count_all_by_user_id(user_id: Snowflake) -> Result<u64, ApiError> {
        count(transaction_template::Entity::find_all_by_user_id(user_id)).await
    }

    pub(crate) async fn find_all_by_user_id_paginated(
        user_id: Snowflake,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(transaction_template::Entity::find_all_by_user_id(user_id), page_size)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub(crate) async fn update(self, updated_dto: TransactionTemplateDTO) -> Result<Self, ApiError> {
        let active_model = transaction_template::ActiveModel {
            id: Set(self.snowflake.id),
            source: Set(updated_dto.source_id.map(|source| source.get_id().id)),
            destination: Set(updated_dto.destination_id.map(|destination| destination.get_id().id)),
            amount: Set(updated_dto.amount),
            currency: Set(updated_dto.currency_id.get_id().id),
            name: Set(updated_dto.name),
            description: Set(updated_dto.description),
            budget: Set(updated_dto.budget_id.map(|budget| budget.get_id().id)),
            created_at: Set(get_now()),
        };
        let model = update(active_model).await?;
        let template = Self::from(model);

        Ok(template)
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(transaction_template::Entity::delete_by_id(self.snowflake)).await?;

        Ok(())
    }
}

permission_impl!(TransactionTemplate);

impl From<transaction_template::Model> for TransactionTemplate {
    fn from(model: transaction_template::Model) -> Self {
        Self {
            snowflake: Snowflake::from(model.id),
            source_id: Phantom::from_option(model.source),
            destination_id: Phantom::from_option(model.destination),
            amount: model.amount,
            currency_id: Phantom::from(model.currency),
            name: model.name,
            description: model.description,
            budget_id: Phantom::from_option(model.budget),
            created_at: model.created_at,
        }
    }
}

impl Identifiable for TransactionTemplate {
    async fn find_by_id(id: Snowflake) -> Result<Self, ApiError> {
        find_one_or_error(transaction_template::Entity::find_by_id(id)).await.map(Self::from)
    }
}

impl TableName for TransactionTemplate {
    fn table_name() -> &'static str {
        transaction_template::Entity.table_name()
    }
}

impl WrapperEntity for TransactionTemplate {
    fn get_id(&self) -> Snowflake {
        self.snowflake
    }
}
