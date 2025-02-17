use crate::opensearch::migration_system::migrator::OpensearchMigrator;
use crate::services::opensearch::client::OpensearchClientInner;
use crate::services::Service;
use crate::utils::type_name::type_name_only;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;

pub struct OpensearchInitializer;

#[async_trait]
impl Initializer for OpensearchInitializer {
    fn name(&self) -> String {
        type_name_only::<Self>().to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> loco_rs::Result<()> {
        let opensearch = OpensearchClientInner::get_arc(ctx).await?;
        OpensearchMigrator::migrate_up(ctx, &opensearch).await?;

        Ok(())
    }
}
