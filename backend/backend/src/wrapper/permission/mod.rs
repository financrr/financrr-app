use std::future::Future;

use bitflags::bitflags;
use sea_orm::{EntityTrait, Set};
use serde::Serialize;
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, SchemaType};
use utoipa::ToSchema;

use entity::permissions;
use entity::permissions::Model;
use entity::utility::table::{does_entity_exist, does_table_exists};
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::connection::get_database_connection;
use crate::database::entity::{count, delete, find_all, find_all_paginated, find_one, insert, update};
use crate::wrapper::entity::{TableName, WrapperEntity};

pub(crate) mod cleanup;

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub(crate) struct Permissions: u32 {
        const READ = 0b001;
        const WRITE = 0b010;
        const DELETE = 0b100;

        const READ_WRITE = Self::READ.bits() | Self::WRITE.bits();
        const READ_DELETE = Self::READ.bits() | Self::DELETE.bits();
    }
}

impl ToSchema<'static> for Permissions {
    fn schema() -> (&'static str, RefOr<Schema>) {
        (
            "Permissions",
            ObjectBuilder::new()
                .schema_type(SchemaType::Integer)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::UInt32)))
                .build()
                .into(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub(crate) struct PermissionsEntity {
    pub(crate) user_id: Snowflake,
    pub(crate) entity_type: String,
    pub(crate) entity_id: Snowflake,
    pub(crate) permissions: Permissions,
}

impl PermissionsEntity {
    pub(crate) async fn find_all_paginated(page_size: PageSizeParam) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(permissions::Entity::find_all(), &page_size).await?.into_iter().map(Self::from).collect())
    }

    pub(crate) async fn count_all() -> Result<u64, ApiError> {
        count(permissions::Entity::count_all()).await
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(permissions::Entity::delete_by_id((self.user_id.id, self.entity_type, self.entity_id.id))).await
    }

    pub(crate) async fn should_be_cleaned_up(&self) -> Result<bool, ApiError> {
        let table_exists = does_table_exists(self.entity_type.as_str(), get_database_connection()).await?;
        if !table_exists {
            return Ok(true);
        }

        let entity_exists =
            does_entity_exist(self.entity_type.as_str(), self.entity_id, get_database_connection()).await?;
        if !entity_exists {
            return Ok(true);
        }

        Ok(false)
    }

    pub(crate) async fn find_all_by_type_and_id(
        entity_type: &str,
        entity_id: Snowflake,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all(permissions::Entity::find_all_by_type_and_id(entity_type, entity_id))
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }
}

impl From<Model> for PermissionsEntity {
    fn from(value: Model) -> Self {
        Self {
            user_id: Snowflake::from(value.user_id),
            entity_type: value.entity_type,
            entity_id: Snowflake::from(value.entity_id),
            permissions: Permissions::from_bits(value.permissions as u32).unwrap_or(Permissions::empty()),
        }
    }
}

impl From<Model> for Permissions {
    fn from(value: Model) -> Self {
        Self::from_bits(value.permissions as u32).unwrap_or(Self::empty())
    }
}

impl From<Option<Model>> for Permissions {
    fn from(value: Option<Model>) -> Self {
        value.map_or(Self::empty(), |v| v.into())
    }
}

#[macro_export]
macro_rules! permission_impl {
    ($type:ty) => {
        impl $crate::wrapper::permission::PermissionByIds for $type {}

        impl $crate::wrapper::permission::Permission for $type {}

        impl $crate::wrapper::permission::HasPermissionOrError for $type {}

        impl $crate::wrapper::permission::HasPermissionByIdOrError for $type {}
    };
}

#[allow(unused)] //TODO remove once multiple users per account has been implemented
pub(crate) trait Permission: PermissionByIds + WrapperEntity {
    fn get_permissions(&self, user_id: Snowflake) -> impl Future<Output = Result<Permissions, ApiError>> {
        async move { Self::get_permissions_by_id(self.get_id(), user_id).await }
    }

    fn has_permission(
        &self,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<bool, ApiError>> {
        async move { Self::has_permission_by_id(self.get_id(), user_id, permissions).await }
    }

    fn add_permission(
        &self,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<(), ApiError>> {
        async move { Self::add_permission_by_id(self.get_id(), user_id, permissions).await }
    }

    fn remove_permission(
        &self,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<(), ApiError>> {
        async move { Self::remove_permission_by_id(self.get_id(), user_id, permissions).await }
    }
}

pub(crate) trait HasPermissionOrError: Permission {
    fn has_permission_or_error(
        &self,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<(), ApiError>> {
        async move {
            let user_permissions = self.get_permissions(user_id).await?;

            has_permission_or_error_raw(user_permissions, permissions, Self::table_name())
        }
    }
}

#[allow(unused)]
pub(crate) trait PermissionByIds: TableName {
    fn get_permissions_by_id(
        entity_id: Snowflake,
        user_id: Snowflake,
    ) -> impl Future<Output = Result<Permissions, ApiError>> {
        async move {
            Ok(Permissions::from(
                find_one(permissions::Entity::find_permission(user_id, Self::table_name(), entity_id)).await?,
            ))
        }
    }

    fn has_permission_by_id(
        entity_id: Snowflake,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<bool, ApiError>> {
        async move {
            Ok(Permissions::from(
                find_one(permissions::Entity::find_permission(user_id, Self::table_name(), entity_id)).await?,
            )
            .contains(permissions))
        }
    }

    fn add_permission_by_id(
        entity_id: Snowflake,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<(), ApiError>> {
        async move {
            let current_permissions = Self::get_permissions_by_id(entity_id, user_id).await?;
            let permissions = current_permissions | permissions;

            save_permission_active_model(entity_id, Self::table_name(), user_id, permissions).await
        }
    }

    fn remove_permission_by_id(
        entity_id: Snowflake,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<(), ApiError>> {
        async move {
            let mut current_permissions = Self::get_permissions_by_id(entity_id, user_id).await?;
            current_permissions.remove(permissions);

            save_permission_active_model(entity_id, Self::table_name(), user_id, current_permissions).await
        }
    }
}

pub(crate) trait HasPermissionByIdOrError: PermissionByIds {
    fn has_permission_by_id_or_error(
        entity_id: Snowflake,
        user_id: Snowflake,
        permissions: Permissions,
    ) -> impl Future<Output = Result<(), ApiError>> {
        async move {
            let user_permissions = Self::get_permissions_by_id(entity_id, user_id).await?;

            has_permission_or_error_raw(user_permissions, permissions, Self::table_name())
        }
    }
}

fn has_permission_or_error_raw(
    user_permissions: Permissions,
    permissions: Permissions,
    table_name: &str,
) -> Result<(), ApiError> {
    if !user_permissions.contains(permissions.clone()) {
        if permissions.contains(Permissions::READ) && !user_permissions.contains(Permissions::READ) {
            return Err(ApiError::ResourceNotFound(table_name));
        }
        return Err(ApiError::MissingPermissions());
    }

    Ok(())
}

async fn save_permission_active_model(
    entity_id: Snowflake,
    table_name: &str,
    user_id: Snowflake,
    permissions: Permissions,
) -> Result<(), ApiError> {
    let active_model = permissions::ActiveModel {
        user_id: Set(user_id.id),
        entity_type: Set(table_name.to_string()),
        entity_id: Set(entity_id.id),
        permissions: Set(permissions.bits() as i32),
    };
    if count(permissions::Entity::find_permission(user_id, table_name, entity_id)).await? > 0 {
        update(active_model).await?;
    } else {
        insert(active_model).await?;
    }

    Ok(())
}
