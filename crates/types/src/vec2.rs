use winit::dpi::PhysicalPosition;

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl From<PhysicalPosition<f64>> for Vec2<f64> {
    fn from(position: PhysicalPosition<f64>) -> Self {
        Self::new(position.x, position.y)
    }
}
