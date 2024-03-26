use backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::{backtrace, panic};

use tracing::error;

pub(crate) fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::capture();
        let message = panic_info.payload().downcast_ref::<&str>().copied().unwrap_or_else(|| {
            panic_info.payload().downcast_ref::<String>().map(|s| s.as_str()).unwrap_or("<unknown>")
        });

        error!("Unrecoverable error occurred: {}", message);

        if backtrace.status().eq(&BacktraceStatus::Captured) {
            error!("Backtrace: {:?}", backtrace);
        }
    }));
}
