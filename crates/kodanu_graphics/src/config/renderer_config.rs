use crate::Backend;

#[derive(Debug, Clone)]
pub struct RendererConfig {
    backends: Backend,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            backends: Backend::AUTO,
        }
    }
}

impl RendererConfig {
    pub fn with_backends(mut self, backends: Backend) -> Self {
        self.backends = backends;
        self
    }

    pub fn backends(&self) -> Backend {
        self.backends
    }
}
