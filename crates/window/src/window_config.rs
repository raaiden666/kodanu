use math::Size;

use winit::window::WindowAttributes;

pub struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    min_width: u32,
    min_height: u32,
}

impl WindowConfig {
    pub const DEFAULT_TITLE_STR: &str = "REngine";

    pub const DEFAULT_WIDTH: u32 = 1280;
    pub const DEFAULT_HEIGHT: u32 = 720;

    pub const DEFAULT_MIN_WIDTH: u32 = 800;
    pub const DEFAULT_MIN_HEIGHT: u32 = 600;
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: Self::DEFAULT_TITLE_STR.into(),
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            min_width: Self::DEFAULT_MIN_WIDTH,
            min_height: Self::DEFAULT_MIN_HEIGHT,
        }
    }
}

impl WindowConfig {
    pub fn to_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(Size::clamped(self.width, self.height))
            .with_min_inner_size(Size::clamped(self.min_width, self.min_height))
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
