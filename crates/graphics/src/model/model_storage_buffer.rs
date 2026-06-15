use crate::ModelUniform;

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    Device, Queue, ShaderStages,
};

use bytemuck::cast_slice;

pub struct ModelSrorageBuffer {
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
    capacity: usize,
}

impl ModelSrorageBuffer {
    pub fn new(device: &Device, capacity: usize) -> Self {
        let buffer = Self::create_buffer(device, capacity);

        let bind_group_layout = Self::create_bind_group_layout(device);

        let bind_group = Self::create_bind_group(device, &bind_group_layout, &buffer);

        Self {
            buffer,
            bind_group,
            bind_group_layout,
            capacity,
        }
    }
}

impl ModelSrorageBuffer {
    pub fn update(&self, queue: &Queue, models: &[ModelUniform]) {
        assert!(models.len() <= self.capacity);

        queue.write_buffer(&self.buffer, 0, cast_slice(models));
    }
}

impl ModelSrorageBuffer {
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

impl ModelSrorageBuffer {
    pub fn create_buffer(device: &Device, capacity: usize) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: Some("Model Sorage Buffer"),
            size: (size_of::<ModelUniform>() * capacity) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Model Storage Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    pub fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model Storage Bind Group Layout"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }
}
