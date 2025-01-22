use crate::services::bank_linking_data::BankLinkingDataInner;
use crate::services::custom_configs::base::CustomConfigInner;
use crate::services::instance_handler::InstanceHandlerInner;
use crate::services::opensearch::client::OpensearchClientInner;
use crate::services::snowflake_generator::SnowflakeGeneratorInner;
use crate::services::status_service::StatusServiceInner;
use crate::services::user_verification::UserVerificationServiceInner;
use axum::{Extension, Router as AxumRouter};
use loco_rs::app::AppContext;
use loco_rs::prelude::Result;
use secret_generator::SecretGeneratorInner;
use std::any::type_name;
use std::future::Future;
use std::sync::{Arc, OnceLock};
use tracing::debug;

pub mod bank_linking_data;
pub mod custom_configs;
pub mod instance_handler;
pub mod opensearch;
pub mod secret_generator;
pub mod snowflake_generator;
pub mod status_service;
pub mod user_verification;

pub async fn configure_services(router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
    Ok(router
        .layer(CustomConfigInner::get_extension(ctx).await?)
        .layer(InstanceHandlerInner::get_extension(ctx).await?)
        .layer(SecretGeneratorInner::get_extension(ctx).await?)
        .layer(UserVerificationServiceInner::get_extension(ctx).await?)
        .layer(SnowflakeGeneratorInner::get_extension(ctx).await?)
        .layer(StatusServiceInner::get_extension(ctx).await?)
        .layer(BankLinkingDataInner::get_extension(ctx).await?)
        .layer(OpensearchClientInner::get_extension(ctx).await?))
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
        debug!("Adding extension for {}", type_name::<Self>());
        async { Ok(Extension(Self::get_arc(ctx).await?)) }
    }
}
