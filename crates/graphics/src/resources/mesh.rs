use crate::resources::Vertex;

use bytemuck::checked::cast_slice;

use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

pub struct Mesh {
    vertex_buffer: Buffer,
    vertex_count: u32,
}

impl Mesh {
    pub fn new(device: &Device, verticies: &[Vertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: cast_slice(verticies),
            usage: BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            vertex_count: verticies.len() as u32,
        }
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
}
