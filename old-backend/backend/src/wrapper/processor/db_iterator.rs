use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::task;
use tokio::task::JoinHandle;
use tracing::error;

use crate::api::error::api::ApiError;
use crate::api::pagination::PageSizeParam;

pub(crate) type CountAllFn =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<u64, ApiError>> + Send + 'static>> + Send + Sync>;
pub(crate) type FindAllPaginatedFn<T> = Arc<
    dyn Fn(PageSizeParam) -> Pin<Box<dyn Future<Output = Result<Vec<T>, ApiError>> + Send + 'static>> + Send + Sync,
>;
pub(crate) type JobFn<T> =
    Arc<dyn Fn(T) -> Pin<Box<dyn Future<Output = Result<(), ApiError>> + Send + 'static>> + Send + Sync>;

pub(crate) async fn process_entity<T>(count_all: CountAllFn, find_all_paginated: FindAllPaginatedFn<T>, job: JobFn<T>)
where
    T: Send + 'static,
{
    let limit: u64 = 500;
    let count = count_all().await.expect("Failed to count all");
    let pages = (count as f64 / limit as f64).ceil() as u64;

    for page in 1..=pages {
        let page_size = PageSizeParam::new(page, limit);
        let data = find_all_paginated(page_size).await.expect("Failed to find all paginated");

        let tasks: Vec<JoinHandle<Result<(), ApiError>>> = data
            .into_iter()
            .map(|entry| {
                let job = job.clone();
                task::spawn(async move { job(entry).await })
            })
            .collect();

        for task in tasks {
            match task.await {
                Ok(result) => {
                    if let Err(e) = result {
                        error!("Task failed with ApiError: {}", e);
                    }
                }
                Err(e) => error!("Task failed with JoinError: {}", e),
            }
        }
    }
}
