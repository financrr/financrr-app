use std::future::Future;

use tokio::task::JoinHandle;
use tokio::time::Interval;

pub(crate) fn schedule_task_with_interval<F, Fut>(mut interval: Interval, task: F) -> JoinHandle<()>
where
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            task().await;
        }
    })
}
