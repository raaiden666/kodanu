use bytemuck::{Pod, Zeroable};
use math::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position }
    }
}

impl Vertex {
    pub fn position(&self) -> [f32; 3] {
        self.position
    }
}

impl From<Vec3> for Vertex {
    fn from(position: Vec3) -> Self {
        Self {
            position: position.to_array(),
        }
    }
}
