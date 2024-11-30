use crate::models::_entities::instances::Model;
use loco_rs::prelude::*;
use tracing::{info, warn};

pub struct CleanupInstances;
#[async_trait]
impl Task for CleanupInstances {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "cleanup_instances".to_string(),
            detail: "Task generator".to_string(),
        }
    }

    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let instances = Model::find_all_inactive_instances(&_app_context.db).await?;
        info!("Found {} inactive instances", instances.len());

        for instance in instances {
            if let Err(e) = instance.delete(&_app_context.db).await {
                warn!("Failed to delete instance: {}", e);
            }
        }

        Ok(())
    }
}
