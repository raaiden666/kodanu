use crate::{
    CameraRenderer, MeshCache, ModelBuffer, RenderItem,
    gpu::{GraphicsDevice, RenderSurface, SurfaceFrame},
    pipeline::GraphicsPipeline,
    renderer::FrameStatus,
    setup::{create_device_and_surface, create_render_surface},
};

use math::Mat4;
use wgpu::{
    Color, CommandEncoderDescriptor, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceTexture, TextureViewDescriptor,
};

use {math::UVec2, std::iter::once, window::Window};

pub struct Renderer {
    graphics_device: GraphicsDevice,
    render_surface: RenderSurface,
    graphics_pipeline: GraphicsPipeline,
    model_buffer: ModelBuffer,
    camera_renderer: CameraRenderer,
    mesh_cache: MeshCache,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let (graphics_device, surface) = create_device_and_surface(window).await;

        let render_surface = create_render_surface(window, &graphics_device, surface);

        let model_buffer = ModelBuffer::new(graphics_device.device());

        let camera_renderer = CameraRenderer::new(graphics_device.device());

        let graphics_pipeline = GraphicsPipeline::new(
            graphics_device.device(),
            render_surface.config().format,
            camera_renderer.bind_group_layout(),
            model_buffer.bind_group_layout(),
        );

        let mesh_cache = MeshCache::new();

        Self {
            graphics_device,
            render_surface,
            graphics_pipeline,
            model_buffer,
            camera_renderer,
            mesh_cache,
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

        self.camera_renderer
            .update(self.graphics_device.queue(), view_projection);

        self.draw_frame(frame, items);

        result
    }

    fn draw_frame(&mut self, frame: SurfaceTexture, items: &[RenderItem]) {
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.graphics_device
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(self.graphics_pipeline.raw());

            render_pass.set_bind_group(0, self.camera_renderer.bind_group(), &[]);

            for item in items {
                let gpu_mesh = self
                    .mesh_cache
                    .get_or_create(self.graphics_device.device(), &item.mesh_handle());

                self.model_buffer
                    .update(self.graphics_device.queue(), item.model());

                render_pass.set_bind_group(1, self.model_buffer.bind_group(), &[]);

                render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer().slice(..));
                render_pass
                    .set_index_buffer(gpu_mesh.index_buffer().slice(..), IndexFormat::Uint32);
                render_pass.draw_indexed(0..gpu_mesh.index_count(), 0, 0..1);
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
