use crate::{CameraUniform, gpu::GraphicsDevice};

use {bytemuck::bytes_of, kodanu_math::Mat4};

use wgpu::util::{BufferInitDescriptor, DeviceExt};

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Queue,
    ShaderStages,
};

#[derive(Debug)]
pub(crate) struct CameraRenderer {
    buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

impl CameraRenderer {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        let uniform = CameraUniform::new(Mat4::IDENTITY);

        let buffer = graphics_device
            .device()
            .create_buffer_init(&BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytes_of(&uniform),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

        let bind_group_layout =
            graphics_device
                .device()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                });

        let bind_group = graphics_device
            .device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &bind_group_layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

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

    #[inline]
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}
