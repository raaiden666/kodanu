use crate::ModelUniform;

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use {bytemuck::bytes_of, math::Mat4};

pub struct ModelRenderer {
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl ModelRenderer {
    pub fn new(device: &Device) -> Self {
        let uniform = ModelUniform::new(Mat4::IDENTITY);

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

impl ModelRenderer {
    pub fn update(&self, queue: &Queue, uniform: ModelUniform) {
        queue.write_buffer(&self.buffer, 0, bytes_of(&uniform));
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

impl ModelRenderer {
    pub fn create_buffer(device: &Device, uniform: ModelUniform) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Model Buffer"),
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
            label: Some("Model Bind Group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Model Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
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
