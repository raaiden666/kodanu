use wgpu::{CurrentSurfaceTexture, Device, Surface, SurfaceConfiguration};

use winit::dpi::PhysicalSize;

pub struct SurfaceContext {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl SurfaceContext {
    pub fn new(
        surface: Surface<'static>,
        config: SurfaceConfiguration,
        size: PhysicalSize<u32>,
    ) -> Self {
        Self {
            surface,
            config,
            size,
        }
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn configure(&self, device: &Device) {
        self.surface.configure(device, &self.config);
    }

    pub fn resize(&mut self, device: &Device, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;

        self.surface.configure(device, &self.config);
    }

    pub fn acquire_frame(&self) -> CurrentSurfaceTexture {
        self.surface.get_current_texture()
    }
}
