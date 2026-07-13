use crate::{Backend, SampleCount};

#[derive(Debug, Clone)]
pub struct RendererConfig {
    backends: Backend,
    sample_count: SampleCount,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            backends: Backend::AUTO,
            sample_count: SampleCount::Quad,
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

    pub fn with_sample_count(mut self, sample_count: SampleCount) -> Self {
        self.sample_count = sample_count;
        self
    }

    pub fn sample_count(&self) -> SampleCount {
        self.sample_count
    }
}
