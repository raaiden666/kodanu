#![allow(dead_code)]

use crate::SampleCount;

use wgpu::{TextureFormat, TextureUsages};

#[derive(Clone, Copy)]
pub struct RenderTextureDescriptor {
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub sample_count: SampleCount,
}

impl RenderTextureDescriptor {
    pub fn new(format: TextureFormat, usage: TextureUsages, sample_count: SampleCount) -> Self {
        Self {
            format,
            usage,
            sample_count,
        }
    }
}

impl RenderTextureDescriptor {
    pub fn depth_texture(sample_count: SampleCount) -> Self {
        Self {
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            sample_count,
        }
    }

    pub fn color_texture(format: TextureFormat, sample_count: SampleCount) -> Self {
        Self {
            format,
            usage: TextureUsages::RENDER_ATTACHMENT,
            sample_count,
        }
    }
}
