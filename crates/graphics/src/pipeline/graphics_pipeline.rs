use crate::VertexLayout;

use wgpu::{
    BindGroupLayout, Device, FragmentState, PipelineLayoutDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState,
};

pub struct GraphicsPipeline {
    pipeline: RenderPipeline,
}

impl GraphicsPipeline {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        camera_bind_group_layout: &BindGroupLayout,
        model_bind_group_layout: &BindGroupLayout,
        material_bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let vs = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Mesh VS"),
            source: ShaderSource::Wgsl(
                include_str!("../../../../resources/shaders/wgsl/mesh.vert.wgsl").into(),
            ),
        });

        let fs = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Mesh FS"),
            source: ShaderSource::Wgsl(
                include_str!("../../../../resources/shaders/wgsl/mesh.frag.wgsl").into(),
            ),
        });

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                Some(camera_bind_group_layout),
                Some(model_bind_group_layout),
                Some(material_bind_group_layout),
            ],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &vs,
                entry_point: Some("vs_main"),
                buffers: &[VertexLayout::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &fs,
                entry_point: Some("fs_main"),
                targets: &[Some(format.into())],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });

        Self { pipeline }
    }
}

impl GraphicsPipeline {
    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
