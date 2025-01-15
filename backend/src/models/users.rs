pub use super::_entities::users::{self, ActiveModel, Entity, Model};
use crate::controllers::user::RegisterParams;
use crate::error::app_error::{AppError, AppResult};
use crate::models::_entities::sessions;
use crate::services::secret_generator::SecretGenerator;
use crate::services::snowflake_generator::SnowflakeGenerator;
use chrono::offset::Local;
use enumflags2::_internal::RawBitFlags;
use enumflags2::bitflags;
use loco_rs::{hash, prelude::*};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{JoinType, PaginatorTrait, QuerySelect, RelationTrait};

#[bitflags(default = User)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UserFlags {
    Admin = 0b0001,
    User = 0b0010,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !self.updated_at.is_set() {
            self.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        }

        if insert {
            self.created_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        }

        Ok(self)
    }
}

impl super::_entities::users::Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> AppResult<Option<Self>> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::Email, email).build())
            .one(db)
            .await?;

        Ok(user)
    }

    pub async fn is_email_unique(db: &DatabaseConnection, email: &str) -> AppResult<bool> {
        let user = users::Entity::find()
            .select_only()
            .column(users::Column::Email)
            .filter(users::Column::Email.eq(email))
            .count(db)
            .await?;

        Ok(user == 0)
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        email: &str,
        token: &str,
    ) -> Result<Option<Self>, AppError> {
        Ok(users::Entity::find()
            .filter(
                query::condition()
                    .eq(users::Column::Email, email)
                    .eq(users::Column::EmailVerificationToken, token)
                    .build(),
            )
            .one(db)
            .await?)
    }

    /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, email: &str, token: &str) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(
                query::condition()
                    .eq(users::Column::Email, email)
                    .eq(users::Column::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> ModelResult<Self> {
        let user = users::Entity::find()
            .filter(query::condition().eq(users::Column::Id, id).build())
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let cloned_api_key = api_key.to_string();
        let user = users::Entity::find()
            .join(
                JoinType::LeftJoin,
                sessions::Relation::Users.def().rev().on_condition(move |_left, right| {
                    Expr::col((right, sessions::Column::ApiKey))
                        .eq(cloned_api_key.as_str())
                        .into_condition()
                }),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password(
        db: &DatabaseConnection,
        snowflake_generator: &SnowflakeGenerator,
        params: &RegisterParams,
    ) -> Result<Self, AppError> {
        let txn = db.begin().await?;

        let password_hash = hash::hash_password(&params.password)?;
        let user = users::ActiveModel {
            id: ActiveValue::set(snowflake_generator.next_id().map_err(|e| ModelError::Any(e.into()))?),
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            reset_token: Default::default(),
            reset_sent_at: Default::default(),
            email_verification_token: Default::default(),
            email_verification_sent_at: Default::default(),
            email_verified_at: Default::default(),
            name: ActiveValue::set(params.name.to_string()),
            flags: ActiveValue::set(UserFlags::DEFAULT as i32),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(user)
    }
}

impl super::_entities::users::ActiveModel {
    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
        secret_generator: &SecretGenerator,
    ) -> ModelResult<Model> {
        const EMAIL_VERIFICATION_TOKEN_LENGTH: usize = 8;
        self.email_verification_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.email_verification_token = ActiveValue::Set(Some(
            secret_generator.generate_token_with_length(EMAIL_VERIFICATION_TOKEN_LENGTH),
        ));
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(
        mut self,
        db: &DatabaseConnection,
        secret_generator: &SecretGenerator,
    ) -> ModelResult<Model> {
        const RESET_TOKEN_LENGTH: usize = 8;
        self.reset_sent_at = ActiveValue::set(Some(Local::now().into()));
        self.reset_token = ActiveValue::Set(Some(secret_generator.generate_token_with_length(RESET_TOKEN_LENGTH)));
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.email_verified_at = ActiveValue::set(Some(Local::now().into()));
        self.email_verification_token = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(mut self, db: &DatabaseConnection, password: &str) -> ModelResult<Model> {
        self.password = ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.reset_token = ActiveValue::Set(None);
        self.reset_sent_at = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }
}
