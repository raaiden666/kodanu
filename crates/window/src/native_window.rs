use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

pub struct NativeWindow {
    window: Arc<Window>,
}

impl NativeWindow {
    pub fn new(window: Arc<Window>) -> Self {
        Self { window }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn raw(&self) -> &Window {
        &self.window
    }

    pub fn arc(&self) -> Arc<Window> {
        Arc::clone(&self.window)
    }
}
