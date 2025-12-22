use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::node::control::DATA_DIR;
use crate::node::control::NETWORK;
use crate::node::log_capture::LogCapture;
use crate::node::log_capture::LogCaptureLayer;

pub(crate) fn setup_logger() -> LogCapture {
    // Create the data directory, if needed.
    let data_dir = format!("{}{}", DATA_DIR, NETWORK);
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let file_appender = tracing_appender::rolling::never(&data_dir, "bonsai.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    std::mem::forget(_guard);

    let log_capture = LogCapture::new(1_000_000);
    let capture_layer = LogCaptureLayer::new(log_capture.clone());

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(
                "info,iced=error,bonsai=debug,bdk_floresta=info,floresta_chain=info,floresta_wire=info",
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
        .with(capture_layer)
        .init();

    tracing::info!("Setup tracing subscriber");

    log_capture
}
