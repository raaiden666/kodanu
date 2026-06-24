use crate::CameraUniform;

use {bytemuck::bytes_of, math::Mat4};

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

pub(crate) struct CameraRenderer {
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl CameraRenderer {
    pub fn new(device: &Device) -> Self {
        let uniform = CameraUniform::new(Mat4::IDENTITY);

        let buffer = Self::create_buffer(device, uniform);

        let bind_group_layout = Self::create_bind_group_layout(device);

        let bind_group = Self::create_bind_group(device, &buffer, &bind_group_layout);

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}
impl CameraRenderer {
    pub fn update(&self, queue: &Queue, view_projection: Mat4) {
        let uniform = CameraUniform::new(view_projection);
        queue.write_buffer(&self.buffer, 0, bytes_of(&uniform));
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

impl CameraRenderer {
    fn create_buffer(device: &Device, uniform: CameraUniform) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytes_of(&uniform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        })
    }

    fn create_bind_group(
        device: &Device,
        buffer: &Buffer,
        bind_group_layout: &BindGroupLayout,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        })
    }

    fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
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
