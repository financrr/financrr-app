use crate::services::configure_services;
use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;

pub struct ServicesInitializer;

#[async_trait]
impl Initializer for ServicesInitializer {
    fn name(&self) -> String {
        "Services".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> loco_rs::Result<AxumRouter> {
        Ok(configure_services(router, ctx).await?)
    }
}
