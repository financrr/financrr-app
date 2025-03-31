use crate::utils::type_name::type_name_only;
use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::{AppContext, Initializer};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

pub struct PathNormalizationInitializer;

#[async_trait]
impl Initializer for PathNormalizationInitializer {
    fn name(&self) -> String {
        type_name_only::<Self>().to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> loco_rs::Result<AxumRouter> {
        let router = NormalizePathLayer::trim_trailing_slash().layer(router);

        Ok(AxumRouter::new().fallback_service(router))
    }
}
