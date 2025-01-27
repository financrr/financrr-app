use crate::error::app_error::AppError;
use crate::models::external_bank_institutions::Entity;
use crate::opensearch::external_bank_institutions::IndexableExternalBankInstitution;
use crate::services::opensearch::client::{OpensearchClientInner, OpensearchIndex};
use crate::services::Service;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::bgworker::BackgroundWorker;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

pub(super) struct AddInstitutionToIndex {
    pub ctx: AppContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct AddInstitutionToIndexArgs {
    pub id: i64,
}

#[async_trait]
impl BackgroundWorker<AddInstitutionToIndexArgs> for AddInstitutionToIndex {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: AddInstitutionToIndexArgs) -> loco_rs::Result<()> {
        let model = Entity::find_by_id(args.id)
            .one(&self.ctx.db)
            .await?
            .ok_or_else(AppError::NotFound)?;
        let indexable_entity = IndexableExternalBankInstitution::from(model);

        let opensearch = OpensearchClientInner::get_arc(&self.ctx).await?;
        opensearch
            .index(args.id, OpensearchIndex::EXTERNAL_BANK_INSTITUTIONS, &indexable_entity)
            .await?;

        Ok(())
    }
}
