use financrr::models::external_bank_institutions;
use financrr::services::Service;
use financrr::services::snowflake_generator::SnowflakeGeneratorInner;
use loco_rs::prelude::AppContext;
use sea_orm::{ActiveModelTrait, Set};

pub async fn generate_institutions(
    ctx: &AppContext,
    provider: &str,
    count: usize,
) -> Vec<external_bank_institutions::Model> {
    let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await.unwrap();

    let mut institutions = vec![];
    for i in 0..count {
        institutions.push(
            external_bank_institutions::ActiveModel {
                id: Set(snowflake_generator.next_id().unwrap()),
                external_id: Set(format!("external_id_{}", i)),
                provider: Set(provider.to_string()),
                name: Set(format!("name_{}", i)),
                bic: Set(None),
                countries: Set(vec!["de".to_string()]),
                logo_link: Set(None),
                access_valid_for_days: Set(None),
                created_at: Set(chrono::Utc::now().into()),
                updated_at: Default::default(),
            }
            .insert(&ctx.db)
            .await
            .unwrap(),
        );
    }

    institutions
}
