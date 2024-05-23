//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.4

use sea_orm::entity::prelude::*;
use sea_orm::{DeleteMany, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "public", table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub token: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub platform_details: Option<String>,
    pub user: i32,
    pub created_at: TimeDateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::User",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_token(token: String) -> Select<Self> {
        Self::find().filter(Column::Token.eq(token))
    }

    pub fn find_by_user_id(user_id: i32) -> Select<Self> {
        Self::find().filter(Column::User.eq(user_id))
    }

    pub fn find_oldest_session_from_user_id(user_id: i32) -> Select<Self> {
        Self::find().filter(Column::User.eq(user_id)).order_by_asc(Column::CreatedAt).limit(1)
    }

    pub fn delete_by_token(session_token: String) -> DeleteMany<Self> {
        Self::delete_many().filter(Column::Token.contains(session_token))
    }
}
