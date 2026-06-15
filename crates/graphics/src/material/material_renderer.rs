use crate::MaterialUniform;

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use {bytemuck::bytes_of, math::Vec4};

pub struct MaterialRenderer {
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl MaterialRenderer {
    pub fn new(device: &Device) -> Self {
        let uniform = MaterialUniform::new(Vec4::ZERO);

        let buffer = Self::create_buffer(device, uniform);

        let bind_group_layout = Self::create_bind_group_layout(device);

        let bind_group = Self::create_bind_group(device, &bind_group_layout, &buffer);

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}

impl MaterialRenderer {
    pub fn update(&self, queue: &Queue, color: Vec4) {
        let uniform = MaterialUniform::new(color);
        queue.write_buffer(&self.buffer, 0, bytes_of(&uniform));
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

impl MaterialRenderer {
    pub fn create_buffer(device: &Device, uniform: MaterialUniform) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Materal Buffer"),
            contents: bytes_of(&uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        })
    }

    pub fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Material Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}
