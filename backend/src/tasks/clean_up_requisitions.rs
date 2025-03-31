use crate::models::go_cardless_requisitions;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::prelude::TaskInfo;
use loco_rs::task::{Task, Vars};
use tracing::info;

pub struct CleanUpRequisitions;

#[async_trait]
impl Task for CleanUpRequisitions {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "CleanUpRequisitions".to_string(),
            detail: "Start a job that cleans up all unused requisitions.".to_string(),
        }
    }

    async fn run(&self, ctx: &AppContext, _: &Vars) -> loco_rs::Result<()> {
        info!("Start cleaning up requisitions");
        let deleted_sum = go_cardless_requisitions::ActiveModel::clean_up(&ctx.db).await?;

        info!("Deleted {deleted_sum} entries!");
        Ok(())
    }
}
