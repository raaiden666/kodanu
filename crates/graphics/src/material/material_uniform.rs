#![allow(dead_code)]

use {
    assets::Material,
    bytemuck::{Pod, Zeroable},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct MaterialUniform {
    base_color: [f32; 4],
}

impl MaterialUniform {
    pub fn new(base_color: [f32; 4]) -> Self {
        Self { base_color }
    }
}

impl MaterialUniform {
    pub fn base_color(&self) -> [f32; 4] {
        self.base_color
    }
}

impl From<&Material> for MaterialUniform {
    fn from(maerial: &Material) -> Self {
        Self {
            base_color: maerial.color().value(),
        }
    }
}
