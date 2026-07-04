use {kodanu_log::LogConfig, kodanu_window::WindowConfig};

#[derive(Debug, Default)]
pub struct AppConfig {
    window_config: WindowConfig,
    log_config: LogConfig,
}

impl AppConfig {
    pub fn set_window_config(&mut self, config: WindowConfig) {
        self.window_config = config;
    }

    pub fn set_log_config(&mut self, config: LogConfig) {
        self.log_config = config;
    }

    pub fn window_config(&self) -> &WindowConfig {
        &self.window_config
    }

    pub fn log_config(&self) -> &LogConfig {
        &self.log_config
    }
}
