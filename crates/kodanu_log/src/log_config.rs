use crate::Level;

use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
pub struct LogConfig {
    filter: EnvFilter,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            filter: EnvFilter::new("info"),
        }
    }
}

impl LogConfig {
    pub fn with_level(self, level: Level) -> Self {
        let level = match level {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
        };

        Self {
            filter: EnvFilter::new(level),
        }
    }

    pub fn with_directive(mut self, directive: impl AsRef<str>) -> Self {
        self.filter = self
            .filter
            .add_directive(directive.as_ref().parse().expect("Invalid log directive"));
        self
    }

    pub fn env_filter(&self) -> EnvFilter {
        self.filter.clone()
    }
}
