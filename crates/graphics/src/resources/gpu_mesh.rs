use crate::Vertex;

use bytemuck::cast_slice;

use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

pub struct GpuMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vertex_count: u32,
    index_count: u32,
}

impl GpuMesh {
    pub fn new(device: &Device, verticies: &[Vertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(verticies),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(verticies),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            vertex_count: verticies.len() as u32,
            index_count: verticies.len() as u32,
        }
    }
}

impl GpuMesh {
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
