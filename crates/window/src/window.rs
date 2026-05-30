use types::Size;

use std::sync::Arc;

use wgpu::{Instance, Surface};

use winit::window::Window as WinitWindow;

pub struct Window {
    raw_window: Arc<WinitWindow>,
}

impl Window {
    pub fn new(raw_window: Arc<WinitWindow>) -> Self {
        Self { raw_window }
    }

    pub fn size(&self) -> Size<u32> {
        self.raw_window.inner_size().into()
    }

    pub fn request_redraw(&self) {
        self.raw_window.request_redraw();
    }

    pub fn create_surface(&self, instance: &Instance) -> Surface<'static> {
        const FAILED_TO_CREATE_SURFACE: &str = "Failed to create surface";

        instance
            .create_surface(Arc::clone(&self.raw_window))
            .expect(FAILED_TO_CREATE_SURFACE)
    }
}
