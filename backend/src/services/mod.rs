use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use crate::services::user_verification::UserVerificationServiceInner;
use axum::{Extension, Router as AxumRouter};
use loco_rs::app::AppContext;
use loco_rs::prelude::Result;
use std::future::Future;
use std::sync::Arc;

pub mod snowflake_generator;
pub mod user_verification;

pub async fn configure_services(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
    Ok(router
        .layer(UserVerificationServiceInner::new_extension(ctx).await?)
        .layer(SnowflakeGeneratorInner::new_extension(ctx).await?))
}

pub trait Service
where
    Self: Sized,
{
    fn new(ctx: &AppContext) -> impl Future<Output = Result<Self>>;

    fn new_arc(ctx: &AppContext) -> impl Future<Output = Result<Arc<Self>>> {
        async { Ok(Arc::new(Self::new(ctx).await?)) }
    }

    fn new_extension(ctx: &AppContext) -> impl Future<Output = Result<Extension<Arc<Self>>>> {
        async { Ok(Extension(Self::new_arc(ctx).await?)) }
    }
}
