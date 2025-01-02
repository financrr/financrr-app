use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityName, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use entity::currency;
use utility::snowflake::entity::Snowflake;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::database::entity::{count, delete, find_all_paginated, find_one_or_error, insert, update};
use crate::wrapper::entity::currency::dto::CurrencyDTO;
use crate::wrapper::entity::user::User;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{Permission, Permissions};
use crate::wrapper::types::phantom::{Identifiable, Phantom};
use crate::{permission_impl, SNOWFLAKE_GENERATOR};

pub(crate) mod dto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub(crate) struct Currency {
    #[serde(rename = "id")]
    pub(crate) snowflake: Snowflake,
    pub(crate) name: String,
    pub(crate) symbol: String,
    pub(crate) iso_code: Option<String>,
    pub(crate) decimal_places: i32,
    pub(crate) user: Option<Phantom<User>>,
}

impl Currency {
    pub(crate) async fn new(creation: CurrencyDTO, user_id: Snowflake) -> Result<Self, ApiError> {
        if !User::exists(user_id).await? {
            return Err(ApiError::ResourceNotFound("User"));
        }
        let snowflake = SNOWFLAKE_GENERATOR.next()?;

        let currency = currency::ActiveModel {
            id: Set(snowflake.id),
            name: Set(creation.name),
            symbol: Set(creation.symbol),
            iso_code: Set(creation.iso_code),
            decimal_places: Set(creation.decimal_places),
            user: Set(Some(user_id.id)),
        };
        let model = insert(currency).await?;
        let currency = Self::from(model);
        currency.add_permission(user_id, Permissions::all()).await?;

        Ok(currency)
    }

    pub(crate) async fn delete(self) -> Result<(), ApiError> {
        delete(currency::Entity::delete_by_id(self.snowflake.id)).await?;

        Ok(())
    }

    pub(crate) async fn find_by_id_include_user(id: Snowflake, user_id: Snowflake) -> Result<Self, ApiError> {
        Ok(Self::from(find_one_or_error(currency::Entity::find_by_id_include_user_id(id, user_id)).await?))
    }

    pub(crate) async fn find_all_with_no_user_paginated(page_size: &PageSizeParam) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(currency::Entity::find_all_with_no_user(), page_size)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub(crate) async fn count_all_with_no_user() -> Result<u64, ApiError> {
        count(currency::Entity::find_all_with_no_user()).await
    }

    pub(crate) async fn find_all_with_no_user_and_user_paginated(
        user_id: Snowflake,
        page_size: &PageSizeParam,
    ) -> Result<Vec<Self>, ApiError> {
        Ok(find_all_paginated(currency::Entity::find_all_with_no_user_and_user_id(user_id), page_size)
            .await?
            .into_iter()
            .map(Self::from)
            .collect())
    }

    pub(crate) async fn count_all_with_no_user_and_user(user_id: Snowflake) -> Result<u64, ApiError> {
        count(currency::Entity::find_all_with_no_user_and_user_id(user_id)).await
    }

    pub(crate) async fn update(self, update_dto: CurrencyDTO) -> Result<Self, ApiError> {
        let user_option = self.user.map(|user| user.get_id());
        let active_model = currency::ActiveModel {
            id: Set(self.snowflake.id),
            name: Set(update_dto.name),
            symbol: Set(update_dto.symbol),
            iso_code: Set(update_dto.iso_code),
            decimal_places: Set(update_dto.decimal_places),
            user: Set(user_option.map(|user_id| user_id.id)),
        };
        let model = update(active_model).await?;

        Ok(Self::from(model))
    }
}

permission_impl!(Currency);

impl TableName for Currency {
    fn table_name() -> &'static str {
        currency::Entity.table_name()
    }
}

impl WrapperEntity for Currency {
    fn get_id(&self) -> Snowflake {
        self.snowflake
    }
}

impl Identifiable for Currency {
    async fn find_by_id(id: Snowflake) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        find_one_or_error(currency::Entity::find_by_id(id)).await.map(Self::from)
    }
}

impl FromRequest for Currency {
    type Error = ApiError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = Json::<Self>::from_request(req, payload);
        let _req = req.clone();
        Box::pin(async move { Ok(fut.await?.into_inner()) })
    }
}

impl From<currency::Model> for Currency {
    fn from(value: currency::Model) -> Self {
        Self {
            snowflake: Snowflake::from(value.id),
            name: value.name,
            symbol: value.symbol,
            iso_code: value.iso_code,
            decimal_places: value.decimal_places,
            user: value.user.map(Phantom::from),
        }
    }
}
