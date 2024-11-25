use crate::api::error::api::ApiError;
use crate::entity::db_model::user::ActiveModel;
use crate::entity::user_entity::UserPermissions;
use crate::util::hashing::hashing::hash_string;
use sea_orm::{NotSet, Set};

pub(crate) struct UserFactory;

impl UserFactory {
    pub(crate) fn new(
        username: String,
        email: Option<String>,
        display_name: Option<String>,
        password: String,
    ) -> Result<ActiveModel, ApiError> {
        let hashed_password = hash_string(&password, None)?;

        Ok(ActiveModel {
            id: NotSet,
            username: Set(username),
            email: Set(email),
            display_name: Set(display_name),
            password: Set(hashed_password),
            permissions: Set(UserPermissions::USER.bits()),
        })
    }
}
