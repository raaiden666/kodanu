use crate::{
    CameraRenderer, ModelSrorageBuffer, fragment_shader, material::MaterialLayout,
    pipeline::vertex_layout::vertex_layout, vertex_shader,
};

use wgpu::{
    Device, FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState,
};

pub(crate) struct GraphicsPipeline {
    pipeline: RenderPipeline,
}

impl GraphicsPipeline {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        camera_renderer: &CameraRenderer,
        model_storage_buffer: &ModelSrorageBuffer,
        material_layout: &MaterialLayout,
    ) -> Self {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                Some(camera_renderer.bind_group_layout()),
                Some(model_storage_buffer.bind_group_layout()),
                Some(material_layout.bind_group_layout()),
            ],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &vertex_shader(device),
                entry_point: Some("vs_main"),
                buffers: &[Some(vertex_layout())],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &fragment_shader(device),
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
