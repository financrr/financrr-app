use sea_orm::ActiveValue::Set;
use sea_orm::{EntityName, EntityTrait};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use entity::budget;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::entity::{count, delete, find_all, find_all_paginated, find_one_or_error, insert, update};
use crate::wrapper::entity::budget::dto::BudgetDTO;
use crate::wrapper::entity::user::User;
use crate::wrapper::entity::WrapperEntity;
use crate::wrapper::permission::{HasPermissionOrError, Permission, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub mod dto;
pub mod event_listener;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Budget {
    pub id: i32,
    pub user: Phantom<User>,
    pub amount: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl Budget {
    pub async fn new(user_id: i32, dto: BudgetDTO) -> Result<Self, ApiError> {
        let model = budget::ActiveModel {
            id: Default::default(),
            user: Set(user_id),
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

    pub async fn find_all_by_user(user_id: i32) -> Result<Vec<Self>, ApiError> {
        Ok(find_all(budget::Entity::find_all_by_user_id(user_id)).await?.into_iter().map(Self::from).collect())
    }

    pub async fn find_all_by_user_paginated(
        user_id: i32,
        page_size_param: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(budget::Entity::find_all_by_user_id(user_id), page_size_param)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub async fn count_all_by_user(user_id: i32) -> Result<u64, ApiError> {
        count(budget::Entity::find_all_by_user_id(user_id)).await
    }

    pub async fn delete(self) -> Result<(), ApiError> {
        delete(budget::Entity::delete_by_id(self.id)).await
    }

    pub async fn update(self, dto: BudgetDTO) -> Result<Self, ApiError> {
        let model = budget::ActiveModel {
            id: Set(self.id),
            user: Set(self.user.get_id()),
            amount: Set(dto.amount),
            name: Set(dto.name),
            description: Set(dto.description),
            created_at: Set(dto.created_at),
        };

        Ok(update(model).await?.into())
    }
}

impl Identifiable for Budget {
    async fn from_id(id: i32) -> Result<Self, ApiError> {
        Ok(Self::from(find_one_or_error(budget::Entity::find_by_id(id), "Budget").await?))
    }
}

impl WrapperEntity for Budget {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn table_name(&self) -> String {
        budget::Entity.table_name().to_string()
    }
}

impl Permission for Budget {}

impl HasPermissionOrError for Budget {}

impl From<budget::Model> for Budget {
    fn from(model: budget::Model) -> Self {
        Self {
            id: model.id,
            user: Phantom::new(model.user),
            amount: model.amount,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
        }
    }
}