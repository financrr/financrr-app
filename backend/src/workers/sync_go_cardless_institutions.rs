use crate::bank_account_linking::constants::GO_CARDLESS_PROVIDER;
use crate::error::app_error::AppError;
use crate::models::external_bank_institutions;
use crate::services::bank_linking_data::BankLinkingDataInner;
use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use crate::services::Service;
use crate::workers::clean_up_external_institutions::CleanUpExternalInstitutions;
use crate::workers::clean_up_external_institutions::WorkerArgs as CleanUpExternalInstitutionsArgs;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

pub struct SyncGoCardlessInstitutionsWorker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs;

#[async_trait]
impl BackgroundWorker<WorkerArgs> for SyncGoCardlessInstitutionsWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, _args: WorkerArgs) -> Result<()> {
        info!("Syncing GoCardless institutions");

        let config = BankLinkingDataInner::get_arc(&self.ctx).await?;
        if !config.is_go_cardless_client_configured() {
            error!("Could not sync GoCardless institutions because the Client is not configured!");

            return Err(AppError::ConfigurationError("GoCardless client not configured.".to_string()).into());
        }

        let client = config.get_go_cardless_client().unwrap();
        let snowflake_generator = SnowflakeGeneratorInner::get_arc(&self.ctx).await?;

        let institutions = client.get_supported_institutions().await?;

        let mut external_ids = Vec::with_capacity(institutions.len());

        for institution in institutions {
            external_ids.push(institution.id.clone());

            if let Err(err) = external_bank_institutions::Entity::add_or_update_from_go_cardless(
                &self.ctx.db,
                &snowflake_generator,
                institution,
            )
            .await
            {
                warn!("Could not insert institution. Error: {}", err)
            }
        }

        CleanUpExternalInstitutions::perform_later(
            &self.ctx,
            CleanUpExternalInstitutionsArgs {
                external_ids,
                provider: GO_CARDLESS_PROVIDER.to_string(),
            },
        )
        .await?;

        info!("Finish syncing institutions.");

        Ok(())
    }
}