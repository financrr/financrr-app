use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm_migration::prelude::*;
use tracing::info;

use entity::currency;
use entity::currency::ActiveModel as Currency;
use entity::utility::loading::load_schema;
use utility::snowflake::generator::SnowflakeGenerator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let data = include_str!("currencies.csv");
        let lines: Vec<&str> = data.lines().collect();
        let snowflake_generator = SnowflakeGenerator::new_from_env().expect("Could not create snowflake generator!");

        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                continue;
            } // Skip the header row
            let snowflake = snowflake_generator.next_id().expect("Could not generate snowflake id");

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 4 {
                let currency = Currency {
                    id: Set(snowflake),
                    name: Set(parts[0].to_string()),
                    symbol: Set(parts[1].to_string()),
                    iso_code: Set(Some(parts[2].to_string())),
                    decimal_places: Set(parts[3].parse().expect("Could not parse decimal places")),
                    user: Set(None),
                };
                currency.insert(manager.get_connection()).await.expect("Could not insert currency");
            } else {
                info!("Skipping line: \"{}\" due to invalid data.", line);
            }
        }
        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(currency::Entity).to_owned()).await.expect("Could not drop table");
        load_schema(manager.get_connection()).await;
        Ok(())
    }
}
