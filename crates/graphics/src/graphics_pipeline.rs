use wgpu::{
    Device, FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource,
    TextureFormat, VertexState,
};

pub struct GraphicsPipeline {
    pipeline: RenderPipeline,
}

impl GraphicsPipeline {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Double Shdaer"),
            source: ShaderSource::Wgsl(
                include_str!("../../../resources/shaders/wgsl/double.wgsl").into(),
            ),
        });

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
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

    pub fn raw(&self) -> &RenderPipeline {
        &self.pipeline
    }
}
