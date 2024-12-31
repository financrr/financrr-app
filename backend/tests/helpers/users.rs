use crate::helpers::faker::generate_random_email;
use chrono::Utc;
use financrr::models::users::{ActiveModel, Model};
use financrr::services::snowflake_generator::SnowflakeGeneratorInner;
use financrr::services::Service;
use loco_rs::app::AppContext;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

pub async fn create_user_with_email_and_id(ctx: &AppContext, id: i64, email: &str) -> Model {
    let active_model = ActiveModel {
        id: Set(id),
        email: Set(email.to_string()),
        name: Default::default(),
        flags: Default::default(),
        password: Default::default(),
        reset_token: Default::default(),
        reset_sent_at: Default::default(),
        email_verification_token: Default::default(),
        email_verification_sent_at: Default::default(),
        email_verified_at: Set(Some(Utc::now().into())),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };

    active_model.insert(&ctx.db).await.unwrap()
}

pub async fn generate_test_user(ctx: &AppContext) -> Model {
    let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await.unwrap();

    create_user_with_email_and_id(
        ctx,
        snowflake_generator.next_id().unwrap(),
        generate_random_email().as_str(),
    )
    .await
}
