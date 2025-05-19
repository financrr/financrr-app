use crate::bank_account_linking::accounts::{AccountBalances, BalanceType, BankAccountDetails, BankAccountInformation};
use crate::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use crate::error::app_error::AppResult;
use crate::models::_entities::bank_accounts::Model;
use crate::models::_entities::{bank_accounts, currencies};
use crate::models::imported_bank_accounts;
use crate::services::Service;
use crate::services::snowflake_generator::{SnowflakeGenerator, SnowflakeGeneratorInner};
use loco_rs::app::AppContext;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::sync::{Arc, OnceLock};

#[derive(Debug)]
pub struct BankAccountFactory {
    snowflake_generator: SnowflakeGenerator,
    db: DatabaseConnection,
}

impl Service for BankAccountFactory {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        Ok(Self {
            snowflake_generator: SnowflakeGeneratorInner::get_arc(&ctx).await?,
            db: ctx.db.clone(),
        })
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<Arc<BankAccountFactory>> = OnceLock::new();

        &INSTANCE
    }
}

impl BankAccountFactory {
    pub async fn from_go_cardless(
        &self,
        information: BankAccountInformation,
        details: BankAccountDetails,
        balances: AccountBalances,
    ) -> AppResult<Model> {
        let currency = currencies::Model::get_by_iso_code_with_default(&self.db, details.currency.as_str()).await?;

        let imported_bank_account = imported_bank_accounts::ActiveModel {
            id: Set(self.snowflake_generator.next_id()?),
            external_id: Set(information.id),
            provider: Set(GO_CARDLESS_PROVIDER.to_string()),
            last_import: Set(None),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        let imported_bank_account = imported_bank_account.insert(&self.db).await?;

        // TODO:
        //  - extract
        //  - add tests
        //  - throw error
        let available_balance = balances
            .balances
            .iter()
            .filter(|v| v.balance_type.eq(&BalanceType::InterimAvailable))
            .last()
            .map(|b| b.balance_amount.amount.parse::<i64>().ok())
            .flatten()
            .unwrap_or(0);

        let expected_balance = balances
            .balances
            .iter()
            .filter(|v| v.balance_type.eq(&BalanceType::Expected))
            .last()
            .map(|b| b.balance_amount.amount.parse::<i64>().ok())
            .flatten()
            .unwrap_or(0);

        let active_model = bank_accounts::ActiveModel {
            id: Set(self.snowflake_generator.next_id()?),
            currency_id: Set(currency.id),
            imported_bank_account_id: Set(Some(imported_bank_account.id)),
            name: Set(details.name),
            description: Set(None),
            iban: Set(Some(information.iban)),
            available_balance: Set(available_balance),
            original_balance: Set(expected_balance),
            expected_balance: Set(expected_balance),
            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let model = active_model.insert(&self.db).await?;

        Ok(model)
    }
}
