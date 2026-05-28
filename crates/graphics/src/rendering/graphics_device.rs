use std::sync::Arc;

use wgpu::{Device, Queue};

pub struct GraphicsDevice {
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl GraphicsDevice {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
