use sea_orm::{ActiveModelBehavior, DeleteMany, EntityTrait, IntoActiveModel, PaginatorTrait, Select};

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::databases::connections::psql::get_database_connection;

pub(crate) async fn find_one<T>(select_stm: Select<T>) -> Result<Option<T::Model>, ApiError>
where
    T: EntityTrait,
{
    let model = select_stm.one(get_database_connection()).await.map_err(ApiError::from)?;

    Ok(model)
}

pub(crate) async fn find_one_or_error<T: EntityTrait>(
    select_stm: Select<T>,
    resource_name: &str,
) -> Result<T::Model, ApiError> {
    find_one(select_stm).await?.ok_or_else(|| ApiError::ResourceNotFound(resource_name))
}

pub(crate) async fn find_all<T: EntityTrait>(select_stm: Select<T>) -> Result<Vec<T::Model>, ApiError> {
    select_stm.all(get_database_connection()).await.map_err(ApiError::from)
}

pub(crate) async fn find_all_paginated<T: EntityTrait>(
    select_stm: Select<T>,
    page_size: &PageSizeParam,
) -> Result<Vec<T::Model>, ApiError>
where
    <T as EntityTrait>::Model: Sync,
{
    PaginatorTrait::paginate(select_stm, get_database_connection(), page_size.limit)
        .fetch_page(page_size.page - 1)
        .await
        .map_err(ApiError::from)
}

pub(crate) async fn count<T: EntityTrait>(select_stm: Select<T>) -> Result<u64, ApiError>
where
    <T as EntityTrait>::Model: Sync,
{
    PaginatorTrait::count(select_stm, get_database_connection()).await.map_err(ApiError::from)
}

pub(crate) async fn insert<T>(active_model: T) -> Result<<T::Entity as EntityTrait>::Model, ApiError>
where
    <T::Entity as EntityTrait>::Model: IntoActiveModel<T>,
    T: ActiveModelBehavior + Send,
{
    active_model.insert(get_database_connection()).await.map_err(ApiError::from)
}

pub(crate) async fn update<T>(active_model: T) -> Result<<T::Entity as EntityTrait>::Model, ApiError>
where
    <T::Entity as EntityTrait>::Model: IntoActiveModel<T>,
    T: ActiveModelBehavior + Send,
{
    active_model.update(get_database_connection()).await.map_err(ApiError::from)
}

pub(crate) async fn delete<T: EntityTrait>(delete: DeleteMany<T>) -> Result<(), ApiError> {
    delete.exec(get_database_connection()).await.map(|_| ()).map_err(ApiError::from)
}
