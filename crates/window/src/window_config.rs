use winit::{dpi::PhysicalSize, window::WindowAttributes};

#[derive(Debug)]
pub struct WindowConfig {
    title: String,
    width: u32,
    height: u32,
    min_width: u32,
    min_height: u32,
    maximized: bool,
    decorations: bool,
}

impl WindowConfig {
    pub const DEFAULT_TITLE_STR: &str = "REngine";
    pub const DEFAULT_WIDTH: u32 = 1440;
    pub const DEFAULT_HEIGHT: u32 = 720;
    pub const DEFAULT_MIN_WIDTH: u32 = 600;
    pub const DEFAULT_MIN_HEIGHT: u32 = 400;
    pub const DEFAULT_MAXIMIZED: bool = false;
    pub const DEFAULT_DECORATIONS: bool = true;
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: Self::DEFAULT_TITLE_STR.to_string(),
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            min_width: Self::DEFAULT_MIN_WIDTH,
            min_height: Self::DEFAULT_MIN_HEIGHT,
            maximized: Self::DEFAULT_MAXIMIZED,
            decorations: Self::DEFAULT_DECORATIONS,
        }
    }
}

impl WindowConfig {
    pub fn to_attributes(&self) -> WindowAttributes {
        WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(PhysicalSize::new(self.width, self.height))
            .with_min_inner_size(PhysicalSize::new(self.min_width, self.min_height))
            .with_maximized(self.maximized)
            .with_decorations(self.decorations)
    }
}

impl WindowConfig {
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

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }
}
