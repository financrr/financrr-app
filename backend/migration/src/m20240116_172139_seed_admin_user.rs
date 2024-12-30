use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ModelTrait};
use sea_orm_migration::prelude::*;

use entity::prelude::User;
use entity::user;
use entity::utility::hashing::hash_string;
use entity::utility::time::get_now;
use utility::snowflake::generator::SnowflakeGenerator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let snowflake_generator = SnowflakeGenerator::new_from_env().expect("Could not create snowflake generator!");
        let snowflake = snowflake_generator.next_id().expect("Could not generate snowflake id");

        let hashed_password = hash_string("Financrr123").expect("Could not hash password Financrr123");
        let user = user::ActiveModel {
            id: Set(snowflake),
            username: Set("admin".to_string()),
            email: Set(None),
            display_name: Set(None),
            password: Set(hashed_password.to_string()),
            created_at: Set(get_now()),
            is_admin: Set(true),
        };
        match user.insert(manager.get_connection()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match User::find_by_username("admin").one(manager.get_connection()).await {
            Ok(Some(user)) => {
                let _ = user.delete(manager.get_connection()).await;
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
