use wgpu::{Adapter, CommandEncoder, CommandEncoderDescriptor, Device, Queue};

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
    pub fn create_encoder(&self, label: &str) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor { label: Some(label) })
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
