use crate::gpu::GraphicsDevice;

use kodanu_math::UVec2;

use wgpu::{
    CompareFunction, DepthBiasState, DepthStencilState, Device, Extent3d, StencilState, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

pub(crate) struct DepthTexture {
    texture: Texture,
    view: TextureView,
}

impl DepthTexture {
    pub fn new(graphics_device: &GraphicsDevice, size: UVec2) -> Self {
        let (texture, view) = Self::create(graphics_device.device(), size);

        Self { texture, view }
    }
}

impl DepthTexture {
    pub fn view(&self) -> &TextureView {
        &self.view
    }

    pub fn resize(&mut self, device: &Device, size: UVec2) {
        (self.texture, self.view) = Self::create(device, size)
    }

    pub fn create_depth_stencil_state() -> DepthStencilState {
        DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(CompareFunction::Less),
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }
    }

    fn create(device: &Device, size: UVec2) -> (Texture, TextureView) {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        (texture, view)
    }
}
