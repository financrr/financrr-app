use std::fmt::Debug;
use std::future::Future;

use tokio::spawn;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::time::sleep;
use tokio::time::Duration;
use tracing::error;

use crate::api::error::api::ApiError;
use crate::wrapper::entity::account::event_listener::account_listener;
use crate::wrapper::entity::budget::event_listener::budget_listener;

pub(crate) mod lifecycle;
pub(crate) mod macros;

const CHANNEL_SIZE: usize = 10240;

pub(crate) fn init() {
    account_listener();
    budget_listener();
}

pub(crate) trait GenericEvent
where
    Self: Debug + Clone + Send + 'static,
{
    fn get_event_bus() -> &'static EventBus<Self>;

    fn fire(self) {
        Self::get_event_bus().fire(self);
    }

    fn fire_scheduled(self, delay: Duration) {
        Self::get_event_bus().fire_scheduled(self, delay);
    }

    fn get_receiver() -> Receiver<Self>
    where
        Self: Sized,
    {
        Self::get_event_bus().subscribe()
    }

    fn subscribe<F, Fut>(function: F)
    where
        F: Fn(Self) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), ApiError>> + Send + 'static,
    {
        spawn(async move {
            loop {
                let event_result = Self::get_receiver().recv().await;
                match event_result {
                    Ok(event) => {
                        let future = function(event);
                        if let Err(e) = future.await {
                            error!("Error executing event callback: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error receiving event: {}", e);
                    }
                }
            }
        });
    }
}

pub(crate) struct EventBus<T: Clone> {
    sender: Sender<T>,
}

impl<T: Debug + Clone + Send + 'static> EventBus<T> {
    pub(crate) fn new() -> Self {
        let (sender, _) = channel(CHANNEL_SIZE);
        Self {
            sender,
        }
    }

    pub(crate) fn subscribe(&self) -> Receiver<T> {
        self.sender.subscribe()
    }

    pub(crate) fn fire(&self, event: T) {
        let _ = self.sender.send(event);
    }

    pub(crate) fn fire_scheduled(&self, event: T, delay: Duration) {
        let sender = self.sender.clone();
        spawn(async move {
            sleep(delay).await;
            let _ = sender.send(event);
        });
    }
}

impl<T: Debug + Clone + Send + 'static> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}
