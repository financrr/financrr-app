use crate::error::app_error::AppResult;
use crate::mailers::auth::AuthMailer;
use crate::models::users;
use crate::services::Service;
use crate::utils::context::AdditionalAppContextMethods;
use loco_rs::prelude::AppContext;
use std::sync::{Arc, OnceLock};

pub type UserVerificationService = Arc<UserVerificationServiceInner>;

#[derive(Clone)]
pub struct UserVerificationServiceInner {
    ctx: AppContext,
}

impl Service for UserVerificationServiceInner {
    async fn new(ctx: &AppContext) -> Result<Self, loco_rs::Error> {
        Ok(Self { ctx: ctx.clone() })
    }

    fn get_static_once() -> &'static OnceLock<Arc<Self>> {
        static INSTANCE: OnceLock<Arc<UserVerificationServiceInner>> = OnceLock::new();

        &INSTANCE
    }
}

impl UserVerificationServiceInner {
    pub async fn send_verification_email_or_verify_user(&self, user: users::ActiveModel) -> AppResult<users::Model> {
        let model = match self.ctx.is_mailer_enabled() {
            true => {
                let model = user.set_email_verification_sent(&self.ctx.db).await?;
                AuthMailer::send_welcome(&self.ctx, &model).await?;

                model
            }
            false => user.verified(&self.ctx.db).await?,
        };

        Ok(model)
    }
}
