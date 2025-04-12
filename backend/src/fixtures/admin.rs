use crate::controllers::user::RegisterParams;
use crate::error::app_error::AppResult;
use crate::fixtures::system::Fixture;
use crate::models::users;
use crate::models::users::UserFlags;
use crate::services::Service;
use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};

pub struct AdminFixture;

#[async_trait]
impl Fixture for AdminFixture {
    fn name(&self) -> String {
        "AdminFixture06032025".to_string()
    }

    async fn run(&self, ctx: &AppContext) -> AppResult<()> {
        let params = RegisterParams {
            email: "admin@financrr".to_string(),
            password: "Financrr123!".to_string(),
            name: "Admin".to_string(),
        };

        let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await?;
        let user = users::Model::create_with_password(&ctx.db, &snowflake_generator, &params).await?;

        let mut user = user.into_active_model().verified(&ctx.db).await?.into_active_model();
        user.flags = Set(UserFlags::Admin as i32);
        user.update(&ctx.db).await?;

        Ok(())
    }
}
