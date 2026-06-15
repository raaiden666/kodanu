use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

use assets::Vertex;

pub struct VertexLayout;

impl VertexLayout {
    pub const ATTRIBUTES: [VertexAttribute; 1] = [VertexAttribute {
        format: VertexFormat::Float32x3,
        offset: 0,
        shader_location: 0,
    }];

    pub const fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
