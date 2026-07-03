use {bytemuck::cast_slice, kodanu_assets::Mesh};

use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

#[derive(Debug)]
pub(crate) struct GpuMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl GpuMesh {
    pub fn new(device: &Device, mesh: &Mesh) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(mesh.vertices()),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(mesh.indices()),
            usage: BufferUsages::INDEX,
        });

        let index_count = mesh.indices().len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}

impl GpuMesh {
    #[inline]
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    #[inline]
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
