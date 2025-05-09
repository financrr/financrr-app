use crate::error::app_error::AppResult;
use async_trait::async_trait;
use loco_rs::prelude::AppContext;

pub mod fixture_executor;

#[async_trait]
pub trait Fixture: Send {
    fn name(&self) -> String;

    async fn run(&self, ctx: &AppContext) -> AppResult<()>;
}
