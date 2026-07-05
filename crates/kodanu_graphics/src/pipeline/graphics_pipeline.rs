use crate::VertexLayout;

use wgpu::{
    BindGroupLayout, Device, FragmentState, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState,
};

#[derive(Debug)]
pub(crate) struct GraphicsPipeline {
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
                buffers: &[Some(VertexLayout::layout())],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &fs,
                entry_point: Some("fs_main"),
                targets: &[Some(format.into())],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self { pipeline }
    }
}

impl GraphicsPipeline {
    #[inline]
    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
