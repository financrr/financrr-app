use crate::fixtures::system::fixture_executor::FixtureExecutor;
use crate::utils::type_name::type_name_only;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;

pub struct FixtureInitializer;

#[async_trait]
impl Initializer for FixtureInitializer {
    fn name(&self) -> String {
        type_name_only::<Self>().to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> loco_rs::Result<()> {
        FixtureExecutor::execute(ctx).await?;

        Ok(())
    }
}
