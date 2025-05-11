use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::prelude::BackgroundWorker;
use serde::Serialize;

pub struct SyncAccountData {
    _ctx: AppContext,
}

#[derive(Debug, Serialize)]
pub struct SyncAccountDataArgs {
    pub user_id: i64,
}

#[async_trait]
impl BackgroundWorker<SyncAccountDataArgs> for SyncAccountData {
    fn build(ctx: &AppContext) -> Self {
        Self { _ctx: ctx.clone() }
    }

    async fn perform(&self, _: SyncAccountDataArgs) -> loco_rs::Result<()> {
        todo!()
    }
}
