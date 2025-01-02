use crate::models::_entities::sessions;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

pub struct SessionUsedWorker {
    pub ctx: AppContext,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionUsedWorkerArgs {
    pub session_id: i64,
}

#[async_trait]
impl BackgroundWorker<SessionUsedWorkerArgs> for SessionUsedWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, args: SessionUsedWorkerArgs) -> Result<()> {
        let session = sessions::Model::find_by_id(&self.ctx.db, args.session_id).await?;
        if let Some(session) = session {
            let session = session.into_active_model();
            session.update_last_accessed_at(&self.ctx.db).await?;
        }

        Ok(())
    }
}
