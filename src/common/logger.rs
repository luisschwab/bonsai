use bitcoin::Network;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::node::log_capture::LogCapture;
use crate::node::log_capture::LogCaptureLayer;
use crate::settings::bonsai_settings::BonsaiSettings;

pub(crate) fn setup_logger(network: Network) -> LogCapture {
    // Create the data directory, if needed.
    let data_dir = BonsaiSettings::base_dir().join(network.to_string());
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        eprintln!(
            "Failed to create log directory at {}: {}",
            data_dir.to_string_lossy(),
            e
        );
    }

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

    tracing::info!("Tracing subscriber setup");

    log_capture
}
