use crate::{
    gpu::GraphicsDevice,
    gpu::SurfaceContext,
    gpu::SurfaceFrame,
    render::RenderResult,
    render::setup::{create_graphics_device, create_surface_context},
    res::{RENDER_ENCODER_LABEL, RENDER_PASS_LABEL},
};

use window::native::NativeWindow;

use primitives::winit::SizeU32;

use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceTexture, TextureViewDescriptor,
};

use std::iter::once;

pub struct Renderer {
    graphics_device: GraphicsDevice,
    surface_context: SurfaceContext,
}

impl Renderer {
    pub async fn new(window: &NativeWindow) -> Self {
        let (graphics_device, surface) = create_graphics_device(window).await;

        let surface_context = create_surface_context(window, &graphics_device, surface);

        Self {
            graphics_device,
            surface_context,
        }
    }

    pub fn render(&self) -> RenderResult {
        let (frame, result) = match self.surface_context.acquire_frame() {
            SurfaceFrame::Ready(frame) => (frame, RenderResult::Success),
            SurfaceFrame::Suboptimal(frame) => (frame, RenderResult::Suboptimal),
            SurfaceFrame::Timeout => return RenderResult::Timeout,
            SurfaceFrame::Occluded => return RenderResult::Occluded,
            SurfaceFrame::Outdated => return RenderResult::Outdated,
            SurfaceFrame::Lost => return RenderResult::Lost,
            SurfaceFrame::Validation => return RenderResult::Validation,
        };

        self.render_frame(&frame);
        frame.present();

        result
    }

    fn render_frame(&self, frame: &SurfaceTexture) {
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.graphics_device
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some(RENDER_ENCODER_LABEL),
                });

        {
            let mut _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some(RENDER_PASS_LABEL),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::RED),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }

        self.graphics_device.queue().submit(once(encoder.finish()));
    }

    pub fn reconfigure_surface(&mut self) {
        self.surface_context
            .configure(self.graphics_device.device());
    }

    pub fn surface_size(&self) -> SizeU32 {
        self.surface_context.size()
    }

    pub fn surface_resize(&mut self, size: SizeU32) {
        self.surface_context
            .resize(self.graphics_device.device(), size);
    }
}
