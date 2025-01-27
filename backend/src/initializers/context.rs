use crate::utils::type_name::type_name_only;
use async_trait::async_trait;
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;
use std::sync::OnceLock;

static GLOBAL_APP_CONTEXT: OnceLock<AppContext> = OnceLock::new();

/// This method should not be used if not necessary!
/// Use "try_get_global_app_context" when possible!
///
/// It was introduced to get the AppContext inside hooks like ActiveModelBehavior.
pub fn get_global_app_context() -> &'static AppContext {
    GLOBAL_APP_CONTEXT
        .get()
        .expect("GLOBAL_APP_CONTEXT should be set by now!")
}

pub fn try_get_global_app_context() -> Option<&'static AppContext> {
    GLOBAL_APP_CONTEXT.get()
}

pub struct ContextInitializer;

#[async_trait]
impl Initializer for ContextInitializer {
    fn name(&self) -> String {
        type_name_only::<Self>().to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> loco_rs::Result<()> {
        let _ = GLOBAL_APP_CONTEXT.get_or_init(|| ctx.clone());

        Ok(())
    }
}
