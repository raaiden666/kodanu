use crate::{
    Mesh, Vertex,
    gpu::{GraphicsDevice, RenderSurface, SurfaceFrame},
    pipeline::GraphicsPipeline,
    renderer::FrameStatus,
    setup::{create_device_and_surface, create_render_surface},
};

use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceTexture, TextureViewDescriptor,
};

use {math::Size, std::iter::once, window::Window};

pub struct Renderer {
    graphics_device: GraphicsDevice,
    render_surface: RenderSurface,
    graphics_pipeline: GraphicsPipeline,

    test_mesh: Mesh,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let (graphics_device, surface) = create_device_and_surface(window).await;

        let render_surface = create_render_surface(window, &graphics_device, surface);

        let graphics_pipeline =
            GraphicsPipeline::new(graphics_device.device(), render_surface.config().format);

        let test_mesh = Mesh::new(
            graphics_device.device(),
            &[
                Vertex {
                    position: [0.0, 0.5, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                },
            ],
        );

        Self {
            graphics_device,
            render_surface,
            graphics_pipeline,
            test_mesh,
        }
    }
}

impl Renderer {
    pub fn render(&self) -> FrameStatus {
        let (frame, result) = match self.render_surface.acquire_frame() {
            SurfaceFrame::Ready(frame) => (frame, FrameStatus::Success),
            SurfaceFrame::Suboptimal(frame) => (frame, FrameStatus::Suboptimal),
            SurfaceFrame::Timeout => return FrameStatus::Timeout,
            SurfaceFrame::Occluded => return FrameStatus::Occluded,
            SurfaceFrame::Outdated => return FrameStatus::Outdated,
            SurfaceFrame::Lost => return FrameStatus::Lost,
            SurfaceFrame::Validation => return FrameStatus::Validation,
        };

        self.draw_frame(&frame);
        frame.present();

        result
    }

    fn draw_frame(&self, frame: &SurfaceTexture) {
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

            self.draw_mesh(&self.test_mesh, &mut render_pass);
        }

        self.graphics_device.queue().submit(once(encoder.finish()));
    }

    pub fn draw_mesh(&self, mesh: &Mesh, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        render_pass.draw(0..mesh.vertex_count(), 0..1);
    }

    pub fn reconfigure_surface(&mut self) {
        self.render_surface.configure(self.graphics_device.device());
    }

    pub fn surface_size(&self) -> Size<u32> {
        self.render_surface.size()
    }

    pub fn surface_resize(&mut self, size: Size<u32>) {
        self.render_surface
            .resize(self.graphics_device.device(), size);
    }
}
