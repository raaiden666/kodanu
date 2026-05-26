use winit::{dpi::PhysicalSize, window::WindowAttributes};

pub struct NativeWindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for NativeWindowConfig {
    fn default() -> Self {
        Self {
            title: "Engine".into(),
            width: 1240,
            height: 720,
        }
    }
}

impl NativeWindowConfig {
    pub fn to_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(PhysicalSize::new(self.width, self.height))
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width.max(1);
        self.height = height.max(1);
        self
    }
}
