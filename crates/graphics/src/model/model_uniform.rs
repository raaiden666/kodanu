use {
    bytemuck::{Pod, Zeroable},
    math::Mat4,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ModelUniform {
    model: [[f32; 4]; 4],
}

impl ModelUniform {
    pub fn new(model: Mat4) -> Self {
        Self {
            model: model.to_cols_array_2d(),
        }
    }
}
