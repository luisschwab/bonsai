use std::collections::VecDeque;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;

use tracing::Subscriber;
use tracing_subscriber::Layer;

const DEFAULT_LOG_VERSION: usize = 0;

#[derive(Clone, Default)]
pub struct LogCapture {
    logs: Arc<Mutex<VecDeque<String>>>,
    version: Arc<Mutex<usize>>,
    max_logs: usize,
}

impl LogCapture {
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Arc::new(Mutex::new(VecDeque::new())),
            version: Arc::new(Mutex::new(0)),
            max_logs,
        }
    }

    pub fn version(&self) -> usize {
        self.version
            .lock()
            .map(|version| *version)
            .unwrap_or(DEFAULT_LOG_VERSION)
    }

    pub fn add_log(&self, log: String) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push_back(log);
            if logs.len() > self.max_logs {
                logs.pop_front();
            }
        }

        self.bump_version();
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs
            .lock()
            .map(|logs| logs.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn clear(&self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }

        self.bump_version();
    }

    fn bump_version(&self) {
        if let Ok(mut version) = self.version.lock() {
            *version = version.wrapping_add(1);
        }
    }
}

pub struct LogCaptureLayer {
    capture: LogCapture,
}

impl LogCaptureLayer {
    pub fn new(capture: LogCapture) -> Self {
        Self { capture }
    }
}

impl<S: Subscriber> Layer<S> for LogCaptureLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut message = String::new();

        let now = chrono::Local::now();
        write!(&mut message, "[{}] ", now.format("%H:%M:%S")).ok();
        write!(&mut message, "{} ", event.metadata().level()).ok();

        let mut visitor = MessageVisitor(String::new());
        event.record(&mut visitor);

        if visitor.0.is_empty() {
            write!(&mut message, "{:?}", event).ok();
        } else {
            message.push_str(&visitor.0);
        }

        self.capture.add_log(message);
    }
}

struct MessageVisitor(String);

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            let debug_str = format!("{:?}", value);
            self.0 = debug_str.trim_matches('"').to_string();
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.0 = value.to_string();
        }
    }
}
