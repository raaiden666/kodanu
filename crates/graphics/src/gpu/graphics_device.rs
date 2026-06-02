use wgpu::{Adapter, Device, Queue};

pub struct GraphicsDevice {
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl GraphicsDevice {
    pub fn new(adapter: Adapter, device: Device, queue: Queue) -> Self {
        Self {
            adapter,
            device,
            queue,
        }
    }
}

impl GraphicsDevice {
    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
