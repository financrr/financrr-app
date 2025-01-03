use std::sync::Arc;
use std::time::Duration;

use tokio::time::interval;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;
use crate::scheduling::schedule_task_with_interval;
use crate::wrapper::permission::PermissionsEntity;
use crate::wrapper::processor::db_iterator::{process_entity, CountAllFn, FindAllPaginatedFn, JobFn};

pub(crate) const CLEAN_UP_INTERVAL_SECONDS: u64 = 60 * 60 * 24;

pub(crate) fn schedule_clean_up_task() {
    let interval = interval(Duration::from_secs(CLEAN_UP_INTERVAL_SECONDS));
    schedule_task_with_interval(interval, clean_up);
}

async fn clean_up() {
    let count_all: CountAllFn = Arc::new(|| Box::pin(PermissionsEntity::count_all()));
    let find_all_paginated: FindAllPaginatedFn<PermissionsEntity> = Arc::new(|page_size: PageSizeParam| {
        let page_size = page_size.clone();
        Box::pin(PermissionsEntity::find_all_paginated(page_size))
    });
    let job: JobFn<PermissionsEntity> = Arc::new(|permissions: PermissionsEntity| {
        let permissions = permissions.clone();
        Box::pin(async move { clean_up_entity(permissions).await })
    });

    process_entity(count_all, find_all_paginated, job).await;
}

async fn clean_up_entity(entity: PermissionsEntity) -> Result<(), ApiError> {
    if entity.should_be_cleaned_up().await? {
        entity.delete().await
    } else {
        Ok(())
    }
}
