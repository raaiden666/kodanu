use winit::dpi::PhysicalPosition;

pub type Vec2f32 = Vec2<f32>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec2i32 = Vec2<i32>;
pub type Vec2u32 = Vec2<u32>;

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
