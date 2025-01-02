use bitflags::bitflags;
use serde::Serialize;
use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, Type};
use utoipa::openapi::schema::SchemaType;
use utoipa::ToSchema;
use crate::entity::db_model::permissions::Model;
use crate::snowflake::snowflake_type::Snowflake;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, ToSchema, Serialize)]
pub(crate) struct Permissions {
    pub(crate) user_id: Snowflake,
    pub(crate) entity_type: String,
    pub(crate) entity_id: i64,
    pub(crate) permissions: PermissionBits
}

impl From<Model> for Permissions {
    fn from(value: Model) -> Self {
        Self {
            user_id: Snowflake::new(value.user_id),
            entity_type: value.entity_type,
            entity_id: value.entity_id,
            permissions: PermissionBits::from_bits_truncate(value.permissions as u32)
        }
    }
}

bitflags! {
    #[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize)]
    pub(crate) struct PermissionBits: u32 {
        const READ = 0b001;
        const WRITE = 0b010;
        const DELETE = 0b100;
    }
}

impl ToSchema<'static> for PermissionBits {
    fn schema() -> (&'static str, RefOr<Schema>) {
        (
            "Permissions",
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(Type::Integer))
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::UInt32)))
                .build()
                .into(),
        )
    }
}
