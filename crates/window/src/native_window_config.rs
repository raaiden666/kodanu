use winit::{dpi::PhysicalSize, window::WindowAttributes};

const DEFAULT_TITLE_STR: &str = "REngine";
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

pub struct NativeWindowConfig {
    title: String,
    width: u32,
    height: u32,
}

impl Default for NativeWindowConfig {
    fn default() -> Self {
        Self {
            title: DEFAULT_TITLE_STR.into(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
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

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
