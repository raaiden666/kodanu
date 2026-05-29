use crate::res::FAILED_TO_CREATE_SURFACE;

use std::sync::Arc;

use wgpu::{Instance, Surface};

use winit::{dpi::PhysicalSize, window::Window};

pub struct NativeWindow {
    raw_window: Arc<Window>,
}

impl NativeWindow {
    pub fn new(raw_window: Arc<Window>) -> Self {
        Self { raw_window }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.raw_window.inner_size()
    }

    pub fn request_redraw(&self) {
        self.raw_window.request_redraw();
    }

    pub fn create_surface(&self, instance: &Instance) -> Surface<'static> {
        instance
            .create_surface(Arc::clone(&self.raw_window))
            .expect(FAILED_TO_CREATE_SURFACE)
    }
}
