use types::Size;

use winit::window::WindowAttributes;

pub struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    min_width: u32,
    min_height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        const DEFAULT_TITLE_STR: &str = "REngine";

        const DEFAULT_WIDTH: u32 = 1280;
        const DEFAULT_HEIGHT: u32 = 720;

        const DEFAULT_MIN_WIDTH: u32 = 800;
        const DEFAULT_MIN_HEIGHT: u32 = 600;

        Self {
            title: DEFAULT_TITLE_STR.into(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            min_width: DEFAULT_MIN_WIDTH,
            min_height: DEFAULT_MIN_HEIGHT,
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
