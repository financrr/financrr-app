use crate::models::users::users;
use crate::validation::ValidationResult;
use loco_rs::app::AppContext;
use tokio::runtime::Handle;
use tokio::task::block_in_place;
use validator::ValidationError;

pub fn validate_email_uniqueness(email: &str, ctx: &AppContext) -> ValidationResult {
    block_in_place(|| {
        Handle::current().block_on(async {
            if !users::Model::is_email_unique(&ctx.db, email).await? {
                return Err(ValidationError::new("Email is already taken"));
            }

            Ok(())
        })
    })
}
