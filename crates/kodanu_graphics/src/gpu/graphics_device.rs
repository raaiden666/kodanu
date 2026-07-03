use wgpu::{Adapter, Device, Queue};

#[derive(Debug)]
pub(crate) struct GraphicsDevice {
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
    #[inline]
    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    #[inline]
    pub fn device(&self) -> &Device {
        &self.device
    }

    #[inline]
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
