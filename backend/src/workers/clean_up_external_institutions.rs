use crate::models::external_bank_institutions;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

pub struct CleanUpExternalInstitutions {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub external_ids: Vec<String>,
    pub provider: String,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for CleanUpExternalInstitutions {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        info!("CleanUp GoCardLess institutes.");
        match external_bank_institutions::Entity::find_unknown_institutions(
            &self.ctx.db,
            args.external_ids,
            args.provider,
        )
        .await
        {
            Ok(models) => self.delete_models(models).await,
            Err(err) => {
                error!("Error while getting unknown institutions. Error: {}", err)
            }
        }

        info!("CleanUp for institutes finished!");
        Ok(())
    }
}

impl CleanUpExternalInstitutions {
    async fn delete_models<T: ActiveModelTrait + ActiveModelBehavior + Send>(
        &self,
        models: Vec<impl IntoActiveModel<T>>,
    ) {
        info!("Deleting {} unknown external institutions.", models.len());
        for active_modle in models.into_iter().map(|i| i.into_active_model()) {
            self.delete_model(active_modle).await;
        }
    }

    async fn delete_model(&self, model: impl ActiveModelBehavior + Send) {
        if let Err(err) = model.delete(&self.ctx.db).await {
            warn!("Could not delete external bank institution. Error: {}", err)
        }
    }
}
