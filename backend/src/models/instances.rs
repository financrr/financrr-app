use super::_entities::instances::{ActiveModel, Column, Entity, Model};
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

    pub async fn create_new_instance(db: &DatabaseConnection, node_id: i16) -> ModelResult<Self> {
        let instance = ActiveModel {
            node_id: ActiveValue::set(node_id),
            ..Default::default()
        };

        Ok(instance.insert(db).await?)
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
