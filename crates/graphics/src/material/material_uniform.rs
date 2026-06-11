use {
    bytemuck::{Pod, Zeroable},
    math::Vec4,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct MaterialUniform {
    base_color: [f32; 4],
}

impl MaterialUniform {
    pub fn new(base_color: Vec4) -> Self {
        Self {
            base_color: base_color.to_array(),
        }
    }
}

impl MaterialUniform {
    pub fn base_color(&self) -> [f32; 4] {
        self.base_color
    }
}
