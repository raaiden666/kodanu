use kodanu_color::Color;

#[derive(Debug)]
pub struct Material {
    color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material {
    pub fn color(&self) -> Color {
        self.color
    }
}
