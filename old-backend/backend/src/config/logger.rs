use std::io::stdout;

use fmt::Subscriber;
use time::{format_description, UtcOffset};
use tracing::{subscriber, Level};
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_log::LogTracer;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub(crate) fn configure() -> WorkerGuard {
    LogTracer::init().expect("Failed to set  tracing-log adapter!");

    // Time format: 2021-01-01 00:00:00
    let timer = format_description::parse("[year]-[month padding:zero]-[day padding:zero] [hour]:[minute]:[second]")
        .expect("Failed to parse format description");
    let time_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let timer = OffsetTime::new(time_offset, timer);

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("financrr")
        .filename_suffix("log")
        .max_log_files(30)
        .build("logs")
        .expect("Failed to create rolling file appender");
    let (non_blocking_file_appender, file_appender_guard) =
        NonBlockingBuilder::default().lossy(false).finish(file_appender);

    let stdout = stdout.with_max_level(Level::INFO);

    let subscriber = Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(stdout.and(non_blocking_file_appender))
        .with_timer(timer)
        .finish();

    subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    file_appender_guard
}
