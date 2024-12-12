use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use crate::services::user_verification::UserVerificationServiceInner;
use axum::{Extension, Router as AxumRouter};
use loco_rs::app::AppContext;
use loco_rs::prelude::Result;
use std::future::Future;
use std::sync::{Arc, OnceLock};

pub mod snowflake_generator;
pub mod user_verification;

pub async fn configure_services(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
    Ok(router
        .layer(UserVerificationServiceInner::get_extension(ctx).await?)
        .layer(SnowflakeGeneratorInner::get_extension(ctx).await?))
}

pub trait Service
where
    Self: Sized + 'static,
{
    fn new(ctx: &AppContext) -> impl Future<Output = Result<Self>>;

    fn get_static_once() -> &'static OnceLock<Arc<Self>>;

    fn new_arc(ctx: &AppContext) -> impl Future<Output = Result<Arc<Self>>> {
        async { Ok(Arc::new(Self::new(ctx).await?)) }
    }

    fn get_arc(ctx: &AppContext) -> impl Future<Output = Result<Arc<Self>>> {
        async {
            let once_lock = Self::get_static_once();

            match once_lock.get() {
                None => {
                    let instance = Self::new_arc(ctx).await?;

                    Ok(once_lock.get_or_init(|| instance).clone())
                }
                Some(instance) => Ok(instance.clone()),
            }
        }
    }

    fn get_extension(ctx: &AppContext) -> impl Future<Output = Result<Extension<Arc<Self>>>> {
        async { Ok(Extension(Self::get_arc(ctx).await?)) }
    }
}
