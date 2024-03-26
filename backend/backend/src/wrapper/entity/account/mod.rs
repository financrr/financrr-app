use futures_util::future::join_all;
use sea_orm::{EntityName, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use entity::account;
use entity::utility::time::get_now;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::databases::entity::{count, delete, find_all, find_all_paginated, find_one_or_error, insert, update};
use crate::wrapper::entity::account::dto::AccountDTO;
use crate::wrapper::entity::currency::Currency;
use crate::wrapper::entity::transaction::Transaction;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{
    HasPermissionByIdOrError, HasPermissionOrError, Permission, PermissionByIds, Permissions,
};
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use crate::wrapper::util::handle_async_result_vec;

pub(crate) mod dto;
pub(crate) mod event_listener;
pub(crate) mod phantom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct Account {
    pub(crate) id: i32,
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
    pub(crate) async fn new(dto: AccountDTO, user_id: i32) -> Result<Self, ApiError> {
        let active_model = account::ActiveModel {
            id: Default::default(),
            name: Set(dto.name),
            description: Set(dto.description),
            iban: Set(dto.iban),
            balance: Set(dto.original_balance),
            original_balance: Set(dto.original_balance),
            currency: Set(dto.currency_id),
            created_at: Set(get_now()),
        };
        let model = insert(active_model).await?;

        let account = Self::from(model);
        account.add_permission(user_id, Permissions::all()).await?;

        Ok(account)
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(account::Entity::delete_by_id(self.id)).await
    }

    pub(crate) async fn update(&self, dto: AccountDTO) -> Result<Self, ApiError> {
        self.update_with_balance(dto, self.balance).await
    }

    pub(crate) async fn update_with_balance(&self, dto: AccountDTO, balance: i64) -> Result<Self, ApiError> {
        let balance = Self::calculate_new_balance(balance, dto.original_balance, self.original_balance);
        let active_model = account::ActiveModel {
            id: Set(self.id),
            name: Set(dto.name),
            description: Set(dto.description),
            iban: Set(dto.iban),
            balance: Set(balance),
            original_balance: Set(dto.original_balance),
            currency: Set(dto.currency_id),
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

    pub(crate) async fn exists(id: i32) -> Result<bool, ApiError> {
        Ok(count(account::Entity::find_by_id(id)).await? > 0)
    }

    pub(crate) async fn find_by_id(id: i32) -> Result<Self, ApiError> {
        Ok(Self::from(find_one_or_error(account::Entity::find_by_id(id), "Account").await?))
    }

    pub(crate) async fn find_all_by_user(user_id: i32) -> Result<Vec<Self>, ApiError> {
        let results = join_all(
            find_all(account::Entity::find_all_for_user(&user_id))
                .await?
                .into_iter()
                .map(|model| Self::find_by_id(model.entity_id)),
        )
        .await;

        handle_async_result_vec(results)
    }

    pub(crate) async fn count_all_by_user(user_id: i32) -> Result<u64, ApiError> {
        count(account::Entity::find_all_for_user(&user_id)).await
    }

    pub(crate) async fn find_transactions_by_account_id_paginated(
        account_id: i32,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Transaction>, ApiError> {
        let results = find_all_paginated(account::Entity::find_related_transactions(account_id), page_size)
            .await?
            .into_iter()
            .map(Transaction::from)
            .collect();

        Ok(results)
    }

    pub(crate) async fn count_transactions_by_account_id(account_id: i32) -> Result<u64, ApiError> {
        count(account::Entity::count_related_transactions(account_id)).await
    }
}

impl WrapperEntity for Account {
    fn get_id(&self) -> i32 {
        self.id
    }
}

impl TableName for Account {
    fn table_name() -> &'static str {
        account::Entity.table_name()
    }
}

impl PermissionByIds for Account {}

impl Permission for Account {}

impl HasPermissionOrError for Account {}

impl HasPermissionByIdOrError for Account {}

impl Identifiable for Account {
    async fn from_id(id: i32) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        Self::find_by_id(id).await
    }
}

impl From<account::Model> for Account {
    fn from(value: account::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            iban: value.iban,
            balance: value.balance,
            original_balance: value.original_balance,
            currency_id: Phantom::new(value.currency),
            created_at: value.created_at,
        }
    }
}
