use crate::res::{
    DEFAULT_HEIGHT, DEFAULT_MIN_HEIGHT, DEFAULT_MIN_WIDTH, DEFAULT_TITLE_STR, DEFAULT_WIDTH,
};

use primitives::winit::SizeU32;

use winit::window::WindowAttributes;

pub struct NativeWindowConfig {
    title: String,
    width: u32,
    height: u32,
    min_width: u32,
    min_height: u32,
}

impl Default for NativeWindowConfig {
    fn default() -> Self {
        Self {
            title: DEFAULT_TITLE_STR.into(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            min_width: DEFAULT_MIN_WIDTH,
            min_height: DEFAULT_MIN_HEIGHT,
        }
    }
}

impl NativeWindowConfig {
    pub fn to_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(SizeU32::clamped(self.width, self.height))
            .with_min_inner_size(SizeU32::clamped(self.min_width, self.min_height))
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

    pub fn with_min_size(mut self, min_width: u32, min_height: u32) -> Self {
        self.min_width = min_width.max(1);
        self.min_height = min_height.max(1);
        self
    }
}
