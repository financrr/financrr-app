use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use sea_orm::{EntityName, EntityTrait};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use dto::Credentials;
use entity::prelude::User as DbUser;
use entity::user;
use entity::user::Model;
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::database::entity::{count, find_one, find_one_or_error, insert};
use crate::permission_impl;
use crate::util::auth::extract_bearer_token;
use crate::wrapper::entity::session::Session;
use crate::wrapper::entity::user::dto::UserRegistration;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{Permission, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};

pub(crate) mod dto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct User {
    #[serde(rename = "id")]
    pub(crate) snowflake: Snowflake,
    pub(crate) username: String,
    pub(crate) email: Option<String>,
    pub(crate) display_name: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub(crate) created_at: OffsetDateTime,
    pub(crate) is_admin: bool,
}

impl User {
    pub(crate) async fn exists(id: Snowflake) -> Result<bool, ApiError> {
        Ok(count(user::Entity::find_by_id(id)).await? > 0)
    }

    pub(crate) async fn authenticate(credentials: &Credentials) -> Result<Self, ApiError> {
        let user = find_one(DbUser::find_by_username(credentials.username.as_str())).await;
        match user {
            Ok(Some(user)) => {
                if user.verify_password(credentials.password.as_bytes()).unwrap_or(false) {
                    Ok(Self::from(user))
                } else {
                    Err(ApiError::InvalidCredentials())
                }
            }
            _ => Err(ApiError::InvalidCredentials()),
        }
    }

    pub(crate) async fn register(registration: UserRegistration) -> Result<Self, ApiError> {
        match user::ActiveModel::register(
            registration.username,
            registration.email,
            registration.display_name,
            registration.password,
        ) {
            Ok(user) => {
                let model = insert(user).await?;
                let user = Self::from(model);
                user.add_permission(user.snowflake, Permissions::all()).await?;

                Ok(user)
            }
            Err(e) => Err(ApiError::from(e)),
        }
    }
}

permission_impl!(User);

impl Identifiable for User {
    async fn find_by_id(id: Snowflake) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        find_one_or_error(user::Entity::find_by_id(id)).await.map(Self::from)
    }
}

impl TableName for User {
    fn table_name() -> &'static str {
        user::Entity.table_name()
    }
}

impl WrapperEntity for User {
    fn get_id(&self) -> Snowflake {
        self.snowflake
    }
}

impl FromRequest for User {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            // get bearer token from request
            let token = extract_bearer_token(&req)?;
            let user_id = Session::find_user_id(token).await?;

            Self::find_by_id(user_id).await
        })
    }
}

impl FromRequest for Phantom<User> {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = extract_bearer_token(&req)?;
            let user_id = Session::find_user_id(token).await?;
            User::exists(user_id).await?;

            Ok(Self::new(user_id))
        })
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            snowflake: Snowflake::from(value.id),
            username: value.username,
            email: value.email,
            display_name: value.display_name,
            created_at: value.created_at,
            is_admin: value.is_admin,
        }
    }
}
