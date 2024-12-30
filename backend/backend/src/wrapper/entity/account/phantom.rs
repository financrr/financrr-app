use sea_orm::EntityName;

use entity::account;
use utility::snowflake::entity::Snowflake;

use crate::wrapper::entity::account::Account;
use crate::wrapper::entity::{TableName, WrapperEntity};
use crate::wrapper::permission::{HasPermissionOrError, Permission, PermissionByIds};
use crate::wrapper::types::phantom::Phantom;

impl WrapperEntity for Phantom<Account> {
    fn get_id(&self) -> Snowflake {
        self.get_id()
    }
}

impl TableName for Phantom<Account> {
    fn table_name() -> &'static str {
        account::Entity.table_name()
    }
}

impl PermissionByIds for Phantom<Account> {}

impl Permission for Phantom<Account> {}

impl HasPermissionOrError for Phantom<Account> {}
