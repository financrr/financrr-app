use crate::models::_entities::instances;
use crate::services::Service;
use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use loco_rs::Error;
use loco_rs::app::AppContext;
use sea_orm::{DatabaseConnection, IntoActiveModel};
use std::process::abort;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::error;

pub const INSTANCE_HEARTBEAT_INTERVAL_SECONDS: u64 = 10;
pub const INSTANCE_HEARTBEAT_TOLERANCE_SECONDS: u64 = INSTANCE_HEARTBEAT_INTERVAL_SECONDS * 3;

pub type InstanceHandler = Arc<InstanceHandlerInner>;

#[derive(Debug)]
pub struct InstanceHandlerInner {
    instance_id: u16,
}

impl Service for InstanceHandlerInner {
    async fn new(ctx: &AppContext) -> loco_rs::Result<Self> {
        let instance = {
            let instance = instances::Model::get_node_id_and_create_new_instance(&ctx.db).await?;

            SnowflakeGeneratorInner::validate_node_id(instance.node_id)?;

            instance
        };

        let handler = Self::with_node_id(instance.node_id);
        handler
            .start_heartbeat(ctx.db.clone())
            .await
            .map_err(|err| Error::Any(err.into()))?;

        Ok(handler)
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<Arc<InstanceHandlerInner>> = OnceLock::new();

        &INSTANCE
    }
}

impl InstanceHandlerInner {
    pub fn with_node_id(node_id: i16) -> Self {
        Self {
            instance_id: node_id as u16,
        }
    }

    async fn start_heartbeat(&self, db: DatabaseConnection) -> loco_rs::Result<(), JobSchedulerError> {
        let scheduler = JobScheduler::new().await?;

        let node_id = self.instance_id as i16;
        let job = Job::new_repeated_async(
            Duration::from_secs(INSTANCE_HEARTBEAT_INTERVAL_SECONDS),
            move |_uuid, _l| {
                let db = db.clone();
                Box::pin(async move {
                    match instances::Model::find_by_node_id(&db, node_id).await {
                        Ok(instance) => {
                            let active_model = instance.into_active_model();
                            if let Err(e) = active_model.update_heartbeat(&db).await {
                                error!("Failed to update heartbeat: {}", e);
                                abort()
                            }
                        }
                        Err(err) => {
                            error!(
                                "Failed to get instance by current node id: {} | Error: {}",
                                node_id, err
                            );
                            abort()
                        }
                    }
                })
            },
        )?;

        scheduler.add(job).await?;

        scheduler.shutdown_on_ctrl_c();
        scheduler.start().await?;

        Ok(())
    }

    pub fn get_instance_id(&self) -> u16 {
        self.instance_id
    }
}
