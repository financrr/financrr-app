use crate::error::app_error::AppResult;
use crate::fixtures::system::Fixture;
use crate::models::_entities::currencies;
use crate::services::Service;
use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use async_trait::async_trait;
use chrono::Utc;
use iso_currency::{Currency, IntoEnumIterator};
use loco_rs::app::AppContext;
use sea_orm::{EntityTrait, Set};

pub struct CurrencyFixture;

#[async_trait]
impl Fixture for CurrencyFixture {
    fn name(&self) -> String {
        "InitialCurrencyFixture06032025".to_string()
    }

    async fn run(&self, ctx: &AppContext) -> AppResult<()> {
        let snowflake_generator = SnowflakeGeneratorInner::get_arc(ctx).await?;

        let currencies = Currency::iter()
            .filter(|c| c.is_superseded().is_none())
            .filter(|c| !c.is_fund())
            .collect::<Vec<_>>();

        let mut currency_models = vec![];
        for currency in currencies {
            currency_models.push(currencies::ActiveModel {
                id: Set(snowflake_generator.next_id()?),
                user_id: Set(None),
                name: Set(currency.name().to_string()),
                symbol: Set(currency.symbol().to_string()),
                iso_code: Set(Some(currency.code().to_string())),
                decimal_places: Set(currency.exponent().unwrap_or(0) as i32),
                created_at: Set(Utc::now().into()),
                updated_at: Set(Utc::now().into()),
            });
        }

        currencies::Entity::insert_many(currency_models)
            .exec_without_returning(&ctx.db)
            .await?;

        Ok(())
    }
}
