use crate::res::{RENDER_ENCODER_LABEL, RENDER_PASS_LABEL};

use crate::{gpu::GraphicsDevice, gpu::SurfaceContext, render::RenderResult};

use crate::render::setup::{
    create_adapter, create_device, create_instance, create_surface_configuration,
};

use crate::gpu::SurfaceFrame;

use window::native::NativeWindow;

use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceTexture, TextureViewDescriptor,
};

use std::iter::once;

use winit::dpi::PhysicalSize;

pub struct Renderer {
    graphics_device: GraphicsDevice,
    surface_context: SurfaceContext,
}

impl Renderer {
    pub async fn new(window: &NativeWindow) -> Self {
        let instance = create_instance();

        let surface = window.create_surface(&instance);

        let adapter = create_adapter(&instance, &surface).await;

        let (device, queue) = create_device(&adapter).await;

        let graphics_device = GraphicsDevice::new(device, queue);

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities.formats[0];

        let size = window.size();

        let config = create_surface_configuration(size, format, capabilities.alpha_modes[0]);

        surface.configure(&graphics_device.device(), &config);

        let surface_context = SurfaceContext::new(surface, config, size);

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
            .configure(&self.graphics_device.device());
    }

    pub fn surface_size(&self) -> PhysicalSize<u32> {
        self.surface_context.size()
    }

    pub fn surface_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_context
            .resize(&self.graphics_device.device(), size);
    }
}
