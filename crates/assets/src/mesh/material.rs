use math::Vec4;

pub struct Material {
    base_color: Vec4,
}

impl Material {
    pub fn new(base_color: Vec4) -> Self {
        Self { base_color }
    }
}

impl Material {
    pub fn base_color(&self) -> Vec4 {
        self.base_color
    }
}

impl Material {
    pub fn red_color() -> Self {
        Self {
            base_color: Vec4::new(1.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn green_color() -> Self {
        Self {
            base_color: Vec4::new(0.0, 1.0, 0.0, 1.0),
        }
    }
}
