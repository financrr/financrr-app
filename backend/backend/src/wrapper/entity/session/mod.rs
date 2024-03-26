use std::ops::Add;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::{join_all, LocalBoxFuture};
use sea_orm::{EntityName, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use time::Duration as TimeDuration;
use time::OffsetDateTime;
use tokio::spawn;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use utoipa::ToSchema;
use uuid::Uuid;

use entity::session;
use entity::utility::time::get_now;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::config::Config;
use crate::databases::entity::{count, delete, find_all_paginated, find_one_or_error, insert, update};
use crate::databases::redis::{del, get, set_ex, zadd};
use crate::util::auth::extract_bearer_token;
use crate::wrapper::entity::user::User;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{
    HasPermissionByIdOrError, HasPermissionOrError, Permission, PermissionByIds, Permissions,
};
use crate::wrapper::util::handle_async_result_vec;

pub(crate) mod dto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct Session {
    pub(crate) id: i32,
    pub(crate) token: String,
    pub(crate) name: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) expires_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
    pub(crate) user: User,
}

impl Session {
    pub(crate) async fn new(user: User, session_name: Option<String>) -> Result<Self, ApiError> {
        let session_token = Self::generate_session_key();

        if Self::reached_session_limit(user.id).await? {
            Self::delete_oldest_session(user.id).await?;
        }

        // insert into database
        let session = session::ActiveModel {
            id: Default::default(),
            token: Set(session_token.clone()),
            name: Set(session_name),
            user: Set(user.id),
            created_at: Set(get_now()),
        };
        let model = insert(session).await?;

        let session = Self::from_model(model).await?;

        // insert into redis
        session.insert_into_redis().await?;

        //grant permissions to user
        session.add_permission(user.id, Permissions::all()).await?;

        Ok(session)
    }

    async fn insert_into_redis(&self) -> Result<(), ApiError> {
        set_ex(self.token.to_owned(), self.user.id.to_string(), self.expires_at.unix_timestamp() as u64).await?;
        zadd("sessions".to_owned(), self.token.to_owned(), self.expires_at.unix_timestamp() as f64).await?;

        Ok(())
    }

    pub(crate) async fn renew(mut self) -> Result<Self, ApiError> {
        self.expires_at = get_now().add(TimeDuration::hours(Config::get_config().session.lifetime_hours as i64));
        self.insert_into_redis().await?;

        let active_model = session::ActiveModel {
            id: Set(self.id),
            token: Set(self.token.to_owned()),
            name: Set(self.name.clone()),
            user: Set(self.user.id),
            created_at: Set(get_now()),
        };
        let model = update(active_model).await?;

        Self::from_model(model).await
    }

    pub(crate) async fn get_user_id(token: String) -> Result<i32, ApiError> {
        let user_id = Self::get_user_id_from_redis(token.to_owned()).await?;
        if user_id.is_none() {
            return Err(ApiError::InvalidSession());
        }

        Ok(user_id.unwrap())
    }

    async fn get_user_id_from_redis(token: String) -> Result<Option<i32>, ApiError> {
        let user_id = get::<Option<i32>>(token.to_owned()).await?;

        match user_id {
            Some(id) => Ok(Some(id)),
            None => Err(ApiError::InvalidSession()),
        }
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        if let Err(e) = del(self.token.to_owned()).await {
            error!("Could not delete session {}: {}", self.token, e);
        }
        delete(session::Entity::delete_by_id(self.id)).await?;

        Ok(())
    }

    pub(crate) async fn delete_by_token(session_token: &String) -> Result<(), ApiError> {
        if let Err(e) = del(session_token.clone()).await {
            error!("Could not delete session {}: {}", session_token, e);
        }
        delete(session::Entity::delete_by_token(session_token.clone())).await?;

        Ok(())
    }

    pub(crate) async fn delete_all_with_user(user_id: i32) -> Result<(), ApiError> {
        let sessions = Self::find_all_by_user(user_id).await?;

        for session in sessions {
            session.delete().await?;
        }

        Ok(())
    }

    pub(crate) async fn find_all_paginated(page_size: &PageSizeParam) -> Result<Vec<Self>, ApiError> {
        let results =
            join_all(find_all_paginated(session::Entity::find(), page_size).await?.into_iter().map(Self::from_model))
                .await;

        handle_async_result_vec(results)
    }

    pub(crate) async fn find_all_by_user_paginated(
        user_id: i32,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        let results = join_all(
            find_all_paginated(session::Entity::find_by_user(user_id), page_size)
                .await?
                .into_iter()
                .map(Self::from_model),
        )
        .await;

        handle_async_result_vec(results)
    }

    pub(crate) async fn find_all_by_user(user_id: i32) -> Result<Vec<Self>, ApiError> {
        let results = join_all(
            find_all_paginated(session::Entity::find_by_user(user_id), &PageSizeParam::default())
                .await?
                .into_iter()
                .map(Self::from_model),
        )
        .await;

        handle_async_result_vec(results)
    }

    pub(crate) async fn find_by_id(id: i32) -> Result<Self, ApiError> {
        let model = find_one_or_error(session::Entity::find_by_id(id), "Session").await?;

        Self::from_model(model).await
    }

    pub(crate) async fn find_by_token(token: String) -> Result<Self, ApiError> {
        let model = find_one_or_error(session::Entity::find_by_token(token), "Session").await?;

        Self::from_model(model).await
    }

    pub(crate) async fn count_all() -> Result<u64, ApiError> {
        count(session::Entity::count()).await
    }

    pub(crate) async fn count_all_by_user(user_id: i32) -> Result<u64, ApiError> {
        count(session::Entity::count_by_user(user_id)).await
    }

    pub(crate) async fn reached_session_limit(user_id: i32) -> Result<bool, ApiError> {
        let sessions = Self::count_all_by_user(user_id).await?;

        Ok(sessions >= Config::get_config().session.limit)
    }

    pub(crate) async fn init() -> Result<(), ApiError> {
        // we have this set intentionally high because we have small datasets => we can afford to do this
        let limit: u64 = 1_000;
        let count = Self::count_all().await?;
        let pages = (count as f64 / limit as f64).ceil() as u64;

        info!("Loading {} pages with {} sessions per page...", pages, limit);
        for page in 1..=pages {
            info!("Loading page {}...", page);
            let page_size = PageSizeParam::new(page, limit);
            let sessions = Self::find_all_paginated(&page_size).await?;
            for session in sessions {
                session.insert_into_redis().await?;
                session.schedule_deletion().await?;
            }
        }

        Ok(())
    }

    async fn schedule_deletion(self) -> Result<(), ApiError> {
        let now = get_now().unix_timestamp();
        let expiration_timestamp = self.expires_at.unix_timestamp();
        let delay = expiration_timestamp - now;
        if delay > 0 {
            Self::schedule_deletion_task(self.token.to_owned(), delay as u64);
        } else {
            self.delete().await?;
        }

        Ok(())
    }

    fn schedule_deletion_task(session_key: String, delay: u64) {
        spawn(async move {
            sleep(Duration::from_secs(delay)).await;
            if let Err(e) = Self::delete_by_token(&session_key).await {
                error!("Could not delete session {}: {}", session_key, e);
            }
        });
    }

    async fn delete_oldest_session(user_id: i32) -> Result<(), ApiError> {
        let model = find_one_or_error(session::Entity::find_oldest_session_from_user(user_id), "Session").await?;
        let session = Self::from_model(model).await?;
        session.delete().await?;

        Ok(())
    }

    async fn from_model(model: session::Model) -> Result<Self, ApiError> {
        let user = User::find_by_id(model.user).await?;
        Ok(Self {
            id: model.id,
            name: model.name,
            token: model.token,
            user,
            expires_at: model.created_at.add(TimeDuration::hours(Config::get_config().session.lifetime_hours as i64)),
            created_at: model.created_at,
        })
    }

    fn generate_session_key() -> String {
        Uuid::new_v4().to_string()
    }
}

impl TableName for Session {
    fn table_name() -> &'static str {
        session::Entity.table_name()
    }
}

impl WrapperEntity for Session {
    fn get_id(&self) -> i32 {
        self.id
    }
}

impl PermissionByIds for Session {}

impl Permission for Session {}

impl HasPermissionOrError for Session {}

impl HasPermissionByIdOrError for Session {}

impl FromRequest for Session {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, ApiError>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = extract_bearer_token(&req)?;

            Self::find_by_token(token).await
        })
    }
}
