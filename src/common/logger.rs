use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::node::control::DATA_DIR;

pub(crate) fn setup_logger() {
    // Create the data directory, if needed.
    let data_dir = format!("{}signet", DATA_DIR);
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let file_appender = tracing_appender::rolling::daily(&data_dir, "bonsai.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    std::mem::forget(_guard);

    // Build the subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(
                "info,bonsai=debug,bdk_floresta=info,floresta_chain=info,floresta_wire=info",
            )
        }))
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .with_target(true),
        )
        .with(
            fmt::layer()
                .with_writer(non_blocking_file)
                .with_ansi(false)
                .with_target(true),
        )
        .init();

    tracing::info!("Bonsai logging initialized");
}
