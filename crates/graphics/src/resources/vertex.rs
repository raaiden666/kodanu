use wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode, vertex_attr_array};

use bytemuck::{Pod, Zeroable};

use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
}

impl Vertex {
    pub fn layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
           0 => Float32x3,
           1 => Float32x3
        ];

        VertexBufferLayout {
            attributes: &ATTRIBUTES,
            array_stride: size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
        }
    }
}
