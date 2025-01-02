pub use sea_orm_migration::prelude::*;

mod m20240110_084613_default_currencies;
mod m20240116_172139_seed_admin_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240110_084613_default_currencies::Migration),
            Box::new(m20240116_172139_seed_admin_user::Migration),
        ]
    }
}
