use crate::workers::external_bank_institutions::add_to_index::{AddInstitutionToIndex, AddInstitutionToIndexArgs};
use async_trait::async_trait;
use loco_rs::prelude::{AppContext, BackgroundWorker};
use serde::{Deserialize, Serialize};

pub struct ExternalBankInstitutionInserted {
    ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalBankInstitutionInsertedArgs {
    pub id: i64,
}

#[async_trait]
impl BackgroundWorker<ExternalBankInstitutionInsertedArgs> for ExternalBankInstitutionInserted {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: ExternalBankInstitutionInsertedArgs) -> loco_rs::Result<()> {
        _ = AddInstitutionToIndex::perform_later(&self.ctx, AddInstitutionToIndexArgs { id: args.id }).await;

        Ok(())
    }
}
