use super::_entities::sessions::{ActiveModel, Entity};
use crate::controllers::session::CreateSessionParams;
use crate::error::app_error::AppResult;
use crate::models::_entities::sessions;
use crate::models::users;
use crate::services::secret_generator::SecretGenerator;
use crate::services::snowflake_generator::SnowflakeGenerator;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;

pub type Sessions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)

    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
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
}
