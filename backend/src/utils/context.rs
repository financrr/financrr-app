use crate::services::custom_config::{CustomConfig, CustomConfigInner};
use crate::services::Service;
use loco_rs::app::AppContext;
use std::future::Future;

pub trait AdditionalAppContextMethods {
    fn is_mailer_enabled(&self) -> bool;

    fn get_custom_config(&self) -> impl Future<Output = loco_rs::Result<CustomConfig>>;
}

impl AdditionalAppContextMethods for AppContext {
    fn is_mailer_enabled(&self) -> bool {
        self.mailer.is_some()
    }

    async fn get_custom_config(&self) -> loco_rs::Result<CustomConfig> {
        CustomConfigInner::get_arc(self).await
    }
}
