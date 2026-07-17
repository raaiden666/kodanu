#![allow(dead_code)]

use crate::{SampleCount, gpu::RenderTextureDescriptor};

use kodanu_math::UVec2;

use wgpu::{
    CompareFunction, DepthBiasState, DepthStencilState, Device, Extent3d, StencilState, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

pub(crate) struct RenderTexture {
    texture: Texture,
    view: TextureView,
    descriptor: RenderTextureDescriptor,
}

impl RenderTexture {
    pub fn new(device: &Device, size: UVec2, descriptor: RenderTextureDescriptor) -> Self {
        let (texture, view) = Self::create_texture(&descriptor, device, size);

        Self {
            texture,
            view,
            descriptor,
        }
    }

    pub fn new_depth(device: &Device, sample_count: SampleCount, size: UVec2) -> Self {
        let descriptor = RenderTextureDescriptor::depth_texture(sample_count);
        let (texture, view) = Self::create_texture(&descriptor, device, size);

        Self {
            texture,
            view,
            descriptor,
        }
    }

    pub fn new_color(
        format: TextureFormat,
        sample_count: SampleCount,
        device: &Device,
        size: UVec2,
    ) -> Self {
        let descriptor = RenderTextureDescriptor::color_texture(format, sample_count);
        let (texture, view) = Self::create_texture(&descriptor, device, size);

        Self {
            texture,
            view,
            descriptor,
        }
    }
}

impl RenderTexture {
    #[inline]
    pub fn view(&self) -> &TextureView {
        &self.view
    }

    #[inline]
    pub fn format(&self) -> TextureFormat {
        self.descriptor.format
    }

    #[inline]
    pub fn sample_count(&self) -> u32 {
        self.descriptor.sample_count.as_u32()
    }

    #[inline]
    pub fn usage(&self) -> TextureUsages {
        self.descriptor.usage
    }

    pub fn resize(&mut self, device: &Device, size: UVec2) {
        (self.texture, self.view) = Self::create_texture(&self.descriptor, device, size);
    }

    pub fn depth_stencil_state() -> DepthStencilState {
        DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(CompareFunction::Less),
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }
    }

    fn create_texture(
        descriptor: &RenderTextureDescriptor,
        device: &Device,
        size: UVec2,
    ) -> (Texture, TextureView) {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Render Texture"),
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            sample_count: descriptor.sample_count.as_u32(),
            mip_level_count: 1,
            dimension: TextureDimension::D2,
            format: descriptor.format,
            usage: descriptor.usage,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        (texture, view)
    }
}
