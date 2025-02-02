use crate::error::app_error::AppResult;
use crate::initializers::opensearch_migrations::version_02022025::Version02022025;
use crate::models::opensearch_migrations::{ActiveModel, Entity};
use crate::opensearch::migration_system::OpensearchMigration;
use crate::services::opensearch::client::OpensearchClient;
use loco_rs::prelude::AppContext;
use tracing::info;

pub struct OpensearchMigrator;

impl OpensearchMigrator {
    pub fn get_migrations() -> Vec<Box<dyn OpensearchMigration>> {
        vec![Box::new(Version02022025)]
    }

    pub async fn migrate_up(ctx: &AppContext, client: &OpensearchClient) -> AppResult<()> {
        let all_applied_migrations = Entity::get_all_applied_migrations(&ctx.db).await?;
        let applied_versions: Vec<String> = all_applied_migrations.iter().map(|m| m.version.clone()).collect();

        let migrations = Self::get_migrations();
        let non_applied_migrations: Vec<_> = migrations
            .into_iter()
            .filter(|m| !applied_versions.contains(&m.name()))
            .collect();

        info!("Number of migrations to be applied: {}", non_applied_migrations.len());

        let mut executed_migrations = Vec::new();

        for migration in non_applied_migrations {
            info!("Applying migration: {}", migration.name());
            migration.up(ctx, client).await?;
            executed_migrations.push(migration.name());
        }

        if !executed_migrations.is_empty() {
            ActiveModel::insert_migrated_versions(&ctx.db, executed_migrations).await?;
        }

        Ok(())
    }
}
