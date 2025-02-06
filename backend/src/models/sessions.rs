use super::_entities::sessions::{ActiveModel, Column, Entity};
use crate::controllers::session::CreateSessionParams;
use crate::error::app_error::{AppError, AppResult};
use crate::middlewares::authentication::Authenticate;
use crate::models::_entities::sessions;
use crate::models::users::users;
use crate::services::secret_generator::SecretGenerator;
use crate::services::snowflake_generator::SnowflakeGenerator;
use crate::workers::session_used::{SessionUsedWorker, SessionUsedWorkerArgs};
use loco_rs::app::AppContext;
use loco_rs::prelude::BackgroundWorker;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use sea_orm::ActiveValue::Set;
use ActiveValue::Unchanged;

pub type Sessions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Authenticate for sessions::Model {
    async fn find_by_api_key(ctx: &AppContext, api_key: &str) -> AppResult<Self> {
        let session = Self::find_by_token(&ctx.db, api_key)
            .await?
            .ok_or_else(AppError::InvalidBearerToken)?;

        SessionUsedWorker::perform_later(ctx, SessionUsedWorkerArgs { session_id: session.id }).await?;

        Ok(session)
    }
}

impl sessions::Model {
    pub async fn create_session(
        db: &DatabaseConnection,
        snowflake_generator: &SnowflakeGenerator,
        secret_generator: &SecretGenerator,
        user: &users::Model,
        params: &CreateSessionParams,
    ) -> AppResult<Self> {
        let session = sessions::ActiveModel {
            id: Set(snowflake_generator.next_id()?),
            user_id: Set(user.id),
            api_key: Set(secret_generator.generate_token()),
            name: Set(params.name.clone()),
            user_agent: Set(params.user_agent.clone()),
            last_accessed_at: Set(None),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };

        Ok(session.insert(db).await?)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> AppResult<Option<Self>> {
        Ok(Entity::find().filter(Column::Id.eq(id)).one(db).await?)
    }

    pub async fn find_by_token(db: &DatabaseConnection, token: &str) -> AppResult<Option<Self>> {
        Ok(Entity::find().filter(Column::ApiKey.eq(token)).one(db).await?)
    }

    pub async fn get_user(&self, db: &DatabaseConnection) -> AppResult<users::Model> {
        Ok(self.find_related(users::Entity).one(db).await?.unwrap())
    }
}

impl sessions::ActiveModel {
    pub async fn update_last_accessed_at(mut self, db: &DatabaseConnection) -> AppResult<sessions::Model> {
        let now = chrono::Utc::now();

        match self.last_accessed_at {
            Set(Some(last_accessed)) => {
                if last_accessed > now {
                    self.last_accessed_at = Set(Some(now.into()));
                }
            }
            Unchanged(Some(last_accessed)) => {
                if last_accessed > now {
                    self.last_accessed_at = Set(Some(now.into()));
                }
            }
            _ => self.last_accessed_at = Set(Some(now.into())),
        }

        Ok(self.update(db).await?)
    }
}
