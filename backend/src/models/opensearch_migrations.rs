pub use super::_entities::opensearch_migrations::{ActiveModel, Entity, Model};
use crate::error::app_error::AppResult;
use sea_orm::Set;
use sea_orm::entity::prelude::*;

pub type OpensearchMigrations = Entity;

impl ActiveModelBehavior for ActiveModel {}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn insert_migrated_versions(db: &DatabaseConnection, versions: Vec<String>) -> AppResult<()> {
        let active_models: Vec<Self> = versions
            .into_iter()
            .map(|version| ActiveModel {
                version: Set(version.to_string()),
                executed_at: Set(chrono::Utc::now().into()),
            })
            .collect();

        Entity::insert_many(active_models).exec_without_returning(db).await?;

        Ok(())
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn get_all_applied_migrations(db: &DatabaseConnection) -> AppResult<Vec<Model>> {
        Ok(Entity::find().all(db).await?)
    }
}
