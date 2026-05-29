use wgpu::{Device, Queue};

pub struct GraphicsDevice {
    device: Device,
    queue: Queue,
}

impl GraphicsDevice {
    pub fn new(device: Device, queue: Queue) -> Self {
        Self { device, queue }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
