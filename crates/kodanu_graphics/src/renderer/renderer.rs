use crate::{
    CameraRenderer, MaterialCache, ModelSrorageBuffer, ModelUniform, RenderItem,
    gpu::{GraphicsDevice, RenderSurface, SurfaceFrame},
    material::MaterialLayout,
    mesh::MeshCache,
    pipeline::GraphicsPipeline,
    renderer::FrameStatus,
    setup::{create_device_and_surface, create_render_surface},
};

use wgpu::{
    Color, IndexFormat, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp, SurfaceTexture, TextureViewDescriptor,
};

use {
    kodanu_math::Mat4, kodanu_math::UVec2, kodanu_window::Window, pollster::block_on,
    std::iter::once,
};

pub struct Renderer {
    graphics_device: GraphicsDevice,
    render_surface: RenderSurface,
    graphics_pipeline: GraphicsPipeline,

    camera_renderer: CameraRenderer,
    model_storage: ModelSrorageBuffer,

    material_layout: MaterialLayout,

    mesh_cache: MeshCache,
    material_cache: MaterialCache,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let (graphics_device, surface) = block_on(create_device_and_surface(window));

        let render_surface = create_render_surface(window, &graphics_device, surface);

        let camera_renderer = CameraRenderer::new(graphics_device.device());

        let model_storage = ModelSrorageBuffer::new(graphics_device.device(), 10_000);

        let material_layout = MaterialLayout::new(graphics_device.device());

        let graphics_pipeline = GraphicsPipeline::new(
            graphics_device.device(),
            render_surface.config().format,
            camera_renderer.bind_group_layout(),
            model_storage.bind_group_layout(),
            material_layout.bind_group_layout(),
        );

        let mesh_cache = MeshCache::new();
        let material_cache = MaterialCache::new();

        Self {
            graphics_device,
            render_surface,
            graphics_pipeline,
            camera_renderer,
            model_storage,
            material_layout,
            mesh_cache,
            material_cache,
        }
    }
}

impl Renderer {
    pub fn render(&mut self, view_projection: Mat4, items: &[RenderItem]) -> FrameStatus {
        let (frame, result) = match self.render_surface.acquire_frame() {
            SurfaceFrame::Ready(frame) => (frame, FrameStatus::Success),
            SurfaceFrame::Suboptimal(frame) => (frame, FrameStatus::Suboptimal),
            SurfaceFrame::Timeout => return FrameStatus::Timeout,
            SurfaceFrame::Occluded => return FrameStatus::Occluded,
            SurfaceFrame::Outdated => return FrameStatus::Outdated,
            SurfaceFrame::Lost => return FrameStatus::Lost,
            SurfaceFrame::Validation => return FrameStatus::Validation,
        };

        let models: Vec<ModelUniform> = items
            .iter()
            .map(|item| ModelUniform::new(item.model()))
            .collect();

        self.camera_renderer
            .update(self.graphics_device.queue(), view_projection);

        self.model_storage
            .update(self.graphics_device.queue(), &models);

        self.draw_frame(frame, items);

        result
    }

    fn draw_frame(&mut self, frame: SurfaceTexture, items: &[RenderItem]) {
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.graphics_device.create_encoder("Command Encoder");

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: (0.01),
                            g: (0.01),
                            b: (0.01),
                            a: (1.0),
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(self.graphics_pipeline.pipeline());

            render_pass.set_bind_group(0, self.camera_renderer.bind_group(), &[]);

            render_pass.set_bind_group(1, self.model_storage.bind_group(), &[]);

            for (index, item) in items.iter().enumerate() {
                let gpu_mesh = self
                    .mesh_cache
                    .get_or_create(self.graphics_device.device(), &item.mesh_handle());

                let gpu_material = self.material_cache.get_or_create(
                    self.graphics_device.device(),
                    self.material_layout.bind_group_layout(),
                    &item.material_handle(),
                );

                render_pass.set_bind_group(2, gpu_material.bind_group(), &[]);

                render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer().slice(..));
                render_pass
                    .set_index_buffer(gpu_mesh.index_buffer().slice(..), IndexFormat::Uint32);
                render_pass.draw_indexed(
                    0..gpu_mesh.index_count(),
                    0,
                    index as u32..index as u32 + 1,
                );
            }
        }

        self.graphics_device.queue().submit(once(encoder.finish()));

        frame.present();
    }

    pub fn reconfigure_surface(&mut self) {
        self.render_surface.configure(self.graphics_device.device());
    }

    pub fn surface_size(&self) -> UVec2 {
        self.render_surface.size()
    }

    pub fn surface_resize(&mut self, size: UVec2) {
        self.render_surface
            .resize(self.graphics_device.device(), size);
    }
}
