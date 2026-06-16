#![allow(dead_code)]

use crate::gpu::SurfaceFrame;

use wgpu::{CurrentSurfaceTexture, Device, Surface, SurfaceConfiguration, TextureFormat};

use math::UVec2;

pub(crate) struct RenderSurface {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    size: UVec2,
}

impl RenderSurface {
    pub fn new(surface: Surface<'static>, config: SurfaceConfiguration, size: UVec2) -> Self {
        Self {
            surface,
            config,
            size,
        }
    }
}

impl RenderSurface {
    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn configure(&self, device: &Device) {
        self.surface.configure(device, &self.config);
    }

    pub fn resize(&mut self, device: &Device, size: UVec2) {
        if size.x == 0 || size.y == 0 {
            return;
        }

        self.size = size;
        self.config.width = size.x;
        self.config.height = size.y;

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
