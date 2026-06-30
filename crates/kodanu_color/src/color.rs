#[derive(Debug, Clone, Copy)]
pub struct Color {
    value: [f32; 4],
}

impl Color {
    pub const WHITE: Self = Self {
        value: [1.0, 1.0, 1.0, 1.0],
    };
    pub const BLACK: Self = Self {
        value: [0.0, 0.0, 0.0, 1.0],
    };
    pub const RED: Self = Self {
        value: [1.0, 0.0, 0.0, 1.0],
    };
    pub const GREEN: Self = Self {
        value: [0.0, 1.0, 0.0, 1.0],
    };
    pub const BLUE: Self = Self {
        value: [0.0, 0.0, 1.0, 1.0],
    };
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            value: [r, g, b, 1.0],
        }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            value: [r, g, b, a],
        }
    }
}

impl Color {
    pub const fn value(&self) -> [f32; 4] {
        self.value
    }
}
