use std::iter::once;

use wgpu::{Adapter, CommandEncoder, CommandEncoderDescriptor, Device, Queue, SurfaceTexture};

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
    pub fn create_encoder(&self) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            })
    }

    pub fn submit(&self, encoder: CommandEncoder) {
        self.queue.submit(once(encoder.finish()));
    }

    pub fn present(&self, surface_texture: SurfaceTexture) {
        self.queue.present(surface_texture);
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
