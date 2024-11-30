use super::_entities::instances::{ActiveModel, Column, Entity, Model};
use crate::services::snowflake_generator::SNOWFLAKE_HEARTBEAT_INTERVAL_SECONDS;
use loco_rs::model::ModelError;
use loco_rs::prelude::ModelResult;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, QueryOrder, QuerySelect};

pub type Instances = Entity;

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

impl Model {
    pub async fn find_next_node_id(db: &DatabaseConnection) -> ModelResult<i16> {
        let result = Entity::find()
            .select_only()
            .column(Column::NodeId)
            .filter(Column::NodeId.gt(0))
            .order_by_asc(Column::NodeId)
            .into_tuple::<i16>()
            .all(db)
            .await?;

        Ok(find_smallest_missing_number(&result))
    }

    pub async fn find_by_node_id(db: &DatabaseConnection, node_id: i16) -> ModelResult<Self> {
        let result = Entity::find().filter(Column::NodeId.eq(node_id)).one(db).await?;

        result.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_all_inactive_instances(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let result = Entity::find()
            .filter(
                Column::LastHeartbeat
                    .lt(chrono::Utc::now()
                        - chrono::Duration::seconds((SNOWFLAKE_HEARTBEAT_INTERVAL_SECONDS + 1) as i64)),
            )
            .all(db)
            .await?;

        Ok(result)
    }

    pub async fn create_new_instance(db: &DatabaseConnection, node_id: i16) -> ModelResult<Self> {
        let instance = ActiveModel {
            node_id: ActiveValue::set(node_id),
            last_heartbeat: ActiveValue::set(chrono::Utc::now().into()),
            created_at: ActiveValue::set(chrono::Utc::now().into()),
            updated_at: ActiveValue::set(chrono::Utc::now().into()),
        };

        Ok(instance.insert(db).await?)
    }
}

impl ActiveModel {
    pub async fn update_heartbeat(mut self, db: &DatabaseConnection) -> ModelResult<Model> {
        self.last_heartbeat = ActiveValue::Set(chrono::Utc::now().into());

        Ok(self.update(db).await?)
    }
}

fn find_smallest_missing_number(numbers: &[i16]) -> i16 {
    for (i, &number) in numbers.iter().enumerate() {
        if number != (i as i16 + 1) {
            return i as i16 + 1;
        }
    }

    numbers.len() as i16 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_missing_number() {
        assert_eq!(find_smallest_missing_number(&[1, 2, 4, 5, 6, 7]), 3);
        assert_eq!(find_smallest_missing_number(&[1, 2, 3, 4, 5, 6, 7]), 8);
        assert_eq!(find_smallest_missing_number(&[2, 3, 4, 5, 6, 7]), 1);
        assert_eq!(find_smallest_missing_number(&[]), 1);
    }
}
