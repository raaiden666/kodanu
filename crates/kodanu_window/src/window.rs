use {
    kodanu_math::UVec2,
    std::sync::Arc,
    wgpu::{Instance, Surface},
};

use winit::window::Window as WinitWindow;

pub struct Window {
    raw_window: Arc<WinitWindow>,
}

impl Window {
    pub fn new(raw_window: Arc<WinitWindow>) -> Self {
        Self { raw_window }
    }
}

impl Window {
    pub fn size(&self) -> UVec2 {
        let size = self.raw_window.inner_size();

        UVec2::new(size.height, size.width)
    }

    pub fn request_redraw(&self) {
        self.raw_window.request_redraw();
    }

    pub fn create_surface(&self, instance: &Instance) -> Surface<'static> {
        instance
            .create_surface(Arc::clone(&self.raw_window))
            .expect("Failed to create surface")
    }
}
