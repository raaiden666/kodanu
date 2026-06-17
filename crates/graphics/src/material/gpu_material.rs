#![allow(dead_code)]
use crate::MaterialUniform;

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use {assets::Material, bytemuck::bytes_of};

pub(crate) struct GpuMaterial {
    buffer: Buffer,
    bind_group: BindGroup,
}

impl GpuMaterial {
    pub fn new(device: &Device, bind_group_layout: &BindGroupLayout, material: &Material) -> Self {
        let uniform = MaterialUniform::from(material);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytes_of(&uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl GpuMaterial {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
