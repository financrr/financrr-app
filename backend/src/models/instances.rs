use super::_entities::instances::{ActiveModel, Column, Entity, Model};
use crate::services::instance_handler::INSTANCE_HEARTBEAT_TOLERANCE_SECONDS;
use loco_rs::model::ModelError;
use loco_rs::prelude::ModelResult;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, IntoActiveModel, QueryOrder, QuerySelect, TransactionTrait};

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
    pub async fn get_node_id_and_create_new_instance(db: &DatabaseConnection) -> ModelResult<Self> {
        db.transaction::<_, _, ModelError>(|txn| {
            Box::pin(async move {
                let all_instances = Entity::find()
                    .order_by_asc(Column::NodeId)
                    .lock_exclusive() // Lock the rows to prevent concurrent access
                    .all(txn)
                    .await?;

                let all_node_ids = all_instances.iter().map(|model| model.node_id).collect::<Vec<_>>();

                let active_node_ids = all_instances
                    .iter()
                    .filter(|model| {
                        let last_heartbeat = model.last_heartbeat.to_utc();
                        let now = chrono::Utc::now();
                        let diff = now - last_heartbeat;
                        diff.num_seconds() < INSTANCE_HEARTBEAT_TOLERANCE_SECONDS as i64
                    })
                    .map(|model| model.node_id)
                    .collect::<Vec<_>>();

                let next_node_id = find_smallest_available_number(&all_node_ids, &active_node_ids);

                let model = Self::create_or_update_instance(txn, next_node_id).await?;

                Ok(model)
            })
        })
        .await
        .map_err(|err| ModelError::Any(err.into()))
    }

    pub async fn find_by_node_id(db: &impl ConnectionTrait, node_id: i16) -> ModelResult<Self> {
        let result = Entity::find().filter(Column::NodeId.eq(node_id)).one(db).await?;

        result.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn count_instances(db: &impl ConnectionTrait) -> ModelResult<u64> {
        Ok(Entity::find().count(db).await?)
    }

    pub async fn create_new_instance(db: &impl ConnectionTrait, node_id: i16) -> ModelResult<Self> {
        let instance = ActiveModel {
            node_id: ActiveValue::set(node_id),
            last_heartbeat: ActiveValue::set(chrono::Utc::now().into()),
            created_at: ActiveValue::set(chrono::Utc::now().into()),
            updated_at: ActiveValue::set(chrono::Utc::now().into()),
        };

        Ok(instance.insert(db).await?)
    }

    async fn create_or_update_instance(db: &impl ConnectionTrait, node_id: i16) -> ModelResult<Self> {
        let instance = Self::find_by_node_id(db, node_id).await;

        match instance {
            Ok(instance) => instance.into_active_model().update_heartbeat(db).await,
            Err(ModelError::EntityNotFound) => Self::create_new_instance(db, node_id).await,
            Err(err) => Err(err),
        }
    }
}

impl ActiveModel {
    pub async fn update_heartbeat(mut self, db: &impl ConnectionTrait) -> ModelResult<Model> {
        self.last_heartbeat = ActiveValue::Set(chrono::Utc::now().into());

        Ok(self.update(db).await?)
    }
}

fn find_smallest_available_number(all_node_ids: &[i16], active_node_ids: &[i16]) -> i16 {
    let active_set: std::collections::HashSet<i16> = active_node_ids.iter().copied().collect();

    for i in 0.. {
        if !active_set.contains(&(i as i16)) {
            return i as i16;
        }
    }

    all_node_ids.len() as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_available_number() {
        assert_eq!(
            find_smallest_available_number(&[0, 1, 2, 4, 5, 6, 7], &[0, 1, 2, 4, 5, 6, 7]),
            3
        );
        assert_eq!(
            find_smallest_available_number(&[0, 1, 2, 3, 4, 5, 6, 7], &[0, 1, 2, 3, 4, 5, 6, 7]),
            8
        );
        assert_eq!(
            find_smallest_available_number(&[2, 3, 4, 5, 6, 7], &[2, 3, 4, 5, 6, 7]),
            0
        );
        assert_eq!(find_smallest_available_number(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &[9]), 0);
        assert_eq!(find_smallest_available_number(&[], &[]), 0);
    }
}
