use crate::gpu::SurfaceFrame;

use primitives::winit::SizeU32;

use wgpu::{CurrentSurfaceTexture, Device, Surface, SurfaceConfiguration};

pub struct SurfaceContext {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    size: SizeU32,
}

impl SurfaceContext {
    pub fn new(surface: Surface<'static>, config: SurfaceConfiguration, size: SizeU32) -> Self {
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

    pub fn size(&self) -> SizeU32 {
        self.size
    }

    pub fn configure(&self, device: &Device) {
        self.surface.configure(device, &self.config);
    }

    pub fn resize(&mut self, device: &Device, size: SizeU32) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.size = size;
        self.config.width = size.width;
        self.config.height = size.height;

        self.surface.configure(device, &self.config);
    }

    pub fn acquire_frame(&self) -> SurfaceFrame {
        match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(frame) => SurfaceFrame::Ready(frame),
            CurrentSurfaceTexture::Suboptimal(frame) => SurfaceFrame::Suboptimal(frame),
            CurrentSurfaceTexture::Timeout => SurfaceFrame::Timeout,
            CurrentSurfaceTexture::Occluded => SurfaceFrame::Occluded,
            CurrentSurfaceTexture::Outdated => SurfaceFrame::Outdated,
            CurrentSurfaceTexture::Lost => SurfaceFrame::Lost,
            CurrentSurfaceTexture::Validation => SurfaceFrame::Validation,
        }
    }
}
