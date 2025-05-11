use crate::services::Service;
use crate::services::status_service::StatusServiceInner;
use crate::views::status::HealthStatus;
use loco_rs::prelude::*;
use tracing::{error, info};

pub struct CheckHealth;

#[async_trait]
impl Task for CheckHealth {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "healthy".to_string(),
            detail: "Checks the health of the system".to_string(),
        }
    }
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        let status_service = StatusServiceInner::get_arc(ctx).await?;
        let health = status_service.get_complete_health_response().await;

        info!("Health response: {:#?}", health);

        match health.status {
            HealthStatus::Healthy => info!("System healthy!"),
            HealthStatus::Unhealthy => {
                error!("System unhealthy!");

                return Err(Error::Message("System is not healthy!".to_string()));
            }
        };

        Ok(())
    }
}
