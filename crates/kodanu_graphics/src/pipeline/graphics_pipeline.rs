use crate::{
    RendererConfig, fragment_shader,
    gpu::{GraphicsDevice, RenderSurface, RenderTexture},
    pipeline::vertex_layout::vertex_layout,
    resources::FrameResources,
    vertex_shader,
};

use wgpu::{
    FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, VertexState,
};

pub(crate) struct GraphicsPipeline {
    pipeline: RenderPipeline,
}

impl GraphicsPipeline {
    pub fn new(
        config: &RendererConfig,
        graphics_device: &GraphicsDevice,
        render_surface: &RenderSurface,
        frame_resources: &FrameResources,
    ) -> Self {
        let layout = graphics_device
            .device()
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    Some(frame_resources.camera_renderer().bind_group_layout()),
                    Some(frame_resources.model_storage().bind_group_layout()),
                    Some(frame_resources.material_layout().bind_group_layout()),
                ],
                immediate_size: 0,
            });

        let pipeline = graphics_device
            .device()
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &vertex_shader(graphics_device.device()),
                    entry_point: Some("vs_main"),
                    buffers: &[Some(vertex_layout())],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(FragmentState {
                    module: &fragment_shader(graphics_device.device()),
                    entry_point: Some("fs_main"),
                    targets: &[Some(render_surface.config().format.into())],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                depth_stencil: Some(RenderTexture::create_depth_stencil_state()),
                primitive: PrimitiveState::default(),
                multisample: MultisampleState {
                    count: config.sample_count().as_u32(),
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
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
