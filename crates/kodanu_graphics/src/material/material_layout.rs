use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, ShaderStages,
};

use crate::gpu::GraphicsDevice;

#[derive(Debug)]
pub(crate) struct MaterialLayout {
    bind_group_layout: BindGroupLayout,
}

impl MaterialLayout {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        let bind_group_layout =
            graphics_device
                .device()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                });

        Self { bind_group_layout }
    }
}

impl MaterialLayout {
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}
