use crate::gpu::SurfaceFrame;

use wgpu::{CurrentSurfaceTexture, Device, Surface, SurfaceConfiguration, TextureFormat};

use types::Size;

pub struct RenderSurface {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    size: Size<u32>,
}

impl RenderSurface {
    pub fn new(surface: Surface<'static>, config: SurfaceConfiguration, size: Size<u32>) -> Self {
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

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn configure(&self, device: &Device) {
        self.surface.configure(device, &self.config);
    }

    pub fn resize(&mut self, device: &Device, size: Size<u32>) {
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

    pub fn format(&self) -> TextureFormat {
        self.config.format
    }
}
