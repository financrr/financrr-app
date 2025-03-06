use crate::error::app_error::AppResult;
use crate::fixtures::admin::AdminFixture;
use crate::fixtures::currency::CurrencyFixture;
use crate::fixtures::system::Fixture;
use crate::models::fixtures::ActiveModel;
use crate::models::fixtures::Entity;
use loco_rs::prelude::AppContext;
use tracing::info;

pub struct FixtureExecutor;

impl FixtureExecutor {
    pub fn get_fixtures() -> Vec<Box<dyn Fixture>> {
        vec![Box::new(CurrencyFixture), Box::new(AdminFixture)]
    }

    pub async fn execute(ctx: &AppContext) -> AppResult<()> {
        let all_applied_fixtures: Vec<String> = Entity::get_all_executed_fixtures(&ctx.db)
            .await?
            .into_iter()
            .map(|e| e.version)
            .collect();

        let fixtures = Self::get_fixtures();

        let non_applied_fixtures: Vec<_> = fixtures
            .into_iter()
            .filter(|m| !all_applied_fixtures.contains(&m.name()))
            .collect();

        info!("Number of fixtures to be applied: {}", non_applied_fixtures.len());

        let mut executed_fixtures = Vec::new();

        for fixture in non_applied_fixtures {
            info!("Applying fixture: {}", fixture.name());
            fixture.run(ctx).await?;
            executed_fixtures.push(fixture.name());
        }

        if !executed_fixtures.is_empty() {
            ActiveModel::insert_executed_fixture(&ctx.db, executed_fixtures).await?;
        }

        Ok(())
    }
}
