use sea_orm::{EntityName, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::error;
use utoipa::ToSchema;

use entity::utility::time::get_now;
use entity::{account, transaction};
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::entity::{count, delete, find_all, find_all_paginated, find_one_or_error, insert, update};
use crate::wrapper::entity::account::dto::AccountDTO;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::transaction::Transaction;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{Permission, Permissions, PermissionsEntity};
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use crate::{permission_impl, SNOWFLAKE_GENERATOR};

pub(crate) mod dto;
pub(crate) mod event_listener;
pub(crate) mod phantom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct Account {
    #[serde(rename = "id")]
    pub(crate) snowflake: Snowflake,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) iban: Option<String>,
    pub(crate) balance: i64,
    pub(crate) original_balance: i64,
    pub(crate) currency_id: Phantom<Currency>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
}

impl Account {
    pub(crate) async fn new(dto: AccountDTO, user_id: Snowflake) -> Result<Self, ApiError> {
        let snowflake = SNOWFLAKE_GENERATOR.next_id()?;
        let active_model = account::ActiveModel {
            id: Set(snowflake),
            name: Set(dto.name),
            description: Set(dto.description),
            iban: Set(dto.iban),
            balance: Set(dto.original_balance),
            original_balance: Set(dto.original_balance),
            currency: Set(dto.currency_id.id),
            created_at: Set(get_now()),
        };
        let model = insert(active_model).await?;

        let account = Self::from(model);
        account.add_permission(user_id, Permissions::all()).await?;

        Ok(account)
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(account::Entity::delete_by_id(self.snowflake.id)).await
    }

    pub(crate) async fn update(&self, dto: AccountDTO) -> Result<Self, ApiError> {
        self.update_with_balance(dto, self.balance).await
    }

    pub(crate) async fn update_with_balance(&self, dto: AccountDTO, balance: i64) -> Result<Self, ApiError> {
        let balance = Self::calculate_new_balance(balance, dto.original_balance, self.original_balance);
        let active_model = account::ActiveModel {
            id: Set(self.snowflake.id),
            name: Set(dto.name),
            description: Set(dto.description),
            iban: Set(dto.iban),
            balance: Set(balance),
            original_balance: Set(dto.original_balance),
            currency: Set(dto.currency_id.id),
            created_at: Set(self.created_at),
        };
        let model = update(active_model).await?;
        let account = Self::from(model);

        //AccountUpdate::new(self.clone(), account.clone()).fire();

        Ok(account)
    }

    fn calculate_new_balance(balance: i64, new_original_balance: i64, old_original_balance: i64) -> i64 {
        balance + new_original_balance - old_original_balance
    }

    pub(crate) async fn exists(id: Snowflake) -> Result<bool, ApiError> {
        Ok(count(account::Entity::find_by_id(id)).await? > 0)
    }

    pub(crate) async fn find_all_by_user(user_id: Snowflake) -> Result<Vec<Self>, ApiError> {
        Ok(find_all(account::Entity::find_all_by_user_id(user_id)).await?.into_iter().map(Self::from).collect())
    }

    pub(crate) async fn count_all_by_user(user_id: Snowflake) -> Result<u64, ApiError> {
        count(account::Entity::find_all_by_user_id(user_id)).await
    }

    pub(crate) async fn find_transactions_by_account_id_paginated(
        account_id: Snowflake,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Transaction>, ApiError> {
        let results = find_all_paginated(transaction::Entity::find_all_by_account_id(account_id), page_size)
            .await?
            .into_iter()
            .map(Transaction::from)
            .collect();

        Ok(results)
    }

    pub(crate) async fn count_transactions_by_account_id(account_id: Snowflake) -> Result<u64, ApiError> {
        count(transaction::Entity::find_all_by_account_id(account_id)).await
    }

    pub(crate) async fn assign_permissions_from_account(
        obj: &impl Permission,
        account_id: Snowflake,
    ) -> Result<(), ApiError> {
        let permissions = PermissionsEntity::find_all_by_type_and_id(Self::table_name(), account_id).await?;
        for permission in permissions {
            if let Err(err) = obj.add_permission(permission.user_id, permission.permissions).await {
                error!("Failed to add permission to user: {}", err);
            }
        }

        Ok(())
    }
}

permission_impl!(Account);

impl WrapperEntity for Account {
    fn get_id(&self) -> Snowflake {
        self.snowflake
    }
}

impl TableName for Account {
    fn table_name() -> &'static str {
        account::Entity.table_name()
    }
}

impl Identifiable for Account {
    async fn find_by_id(id: Snowflake) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        find_one_or_error(account::Entity::find_by_id(id)).await.map(Self::from)
    }
}

impl From<account::Model> for Account {
    fn from(value: account::Model) -> Self {
        Self {
            snowflake: Snowflake::from(value.id),
            name: value.name,
            description: value.description,
            iban: value.iban,
            balance: value.balance,
            original_balance: value.original_balance,
            currency_id: Phantom::from(value.currency),
            created_at: value.created_at,
        }
    }
}
