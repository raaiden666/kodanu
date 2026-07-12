use crate::{
    AssetResources, RenderItem, RendererConfig,
    gpu::{DepthTexture, GraphicsDevice, RenderSurface, SurfaceFrame},
    pipeline::GraphicsPipeline,
    renderer::FrameStatus,
    resources::FrameResources,
    setup::{create_device_and_surface, create_render_surface},
};

use wgpu::{
    Color, CommandEncoder, IndexFormat, LoadOp, Operations, RenderPass, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp, SurfaceTexture, TextureView,
    TextureViewDescriptor,
};

use {kodanu_math::Mat4, kodanu_math::UVec2, kodanu_window::Window, pollster::block_on};

pub struct Renderer {
    graphics_device: GraphicsDevice,
    render_surface: RenderSurface,
    graphics_pipeline: GraphicsPipeline,
    depth_texture: DepthTexture,

    asset_resources: AssetResources,
    frame_resources: FrameResources,
}

impl Renderer {
    pub fn new(window: &Window, config: &RendererConfig) -> Self {
        let (graphics_device, surface) = block_on(create_device_and_surface(window, config));

        let render_surface = create_render_surface(window, &graphics_device, surface);

        let frame_resources = FrameResources::new(&graphics_device);

        let graphics_pipeline =
            GraphicsPipeline::new(&graphics_device, &render_surface, &frame_resources);

        let depth_texture = DepthTexture::new(&graphics_device, window.size());

        let asset_resources = AssetResources::default();

        Self {
            graphics_device,
            render_surface,
            graphics_pipeline,
            depth_texture,
            frame_resources,
            asset_resources,
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

        self.frame_resources
            .update(self.graphics_device.queue(), view_projection, items);

        self.draw_frame(frame, items);

        result
    }

    fn draw_frame(&mut self, frame: SurfaceTexture, items: &[RenderItem]) {
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.graphics_device.create_encoder();

        {
            let mut render_pass = self.create_render_pass(&mut encoder, &view);

            render_pass.set_pipeline(self.graphics_pipeline.pipeline());

            render_pass.set_bind_group(0, self.frame_resources.camera_renderer().bind_group(), &[]);
            render_pass.set_bind_group(1, self.frame_resources.model_storage().bind_group(), &[]);

            for (instance, item) in items.iter().enumerate() {
                self.draw_item(&mut render_pass, instance as u32, item);
            }
        }

        self.graphics_device.submit(encoder);
        self.graphics_device.present(frame);
    }

    fn draw_item(&mut self, render_pass: &mut RenderPass, instance: u32, item: &RenderItem) {
        let gpu_mesh = self
            .asset_resources
            .gpu_mesh(self.graphics_device.device(), item);

        let gpu_material = self.asset_resources.gpu_material(
            self.graphics_device.device(),
            &self.frame_resources,
            item,
        );

        render_pass.set_bind_group(2, gpu_material.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer().slice(..));
        render_pass.set_index_buffer(gpu_mesh.index_buffer().slice(..), IndexFormat::Uint32);
        render_pass.draw_indexed(0..gpu_mesh.index_count(), 0, instance..instance + 1);
    }

    fn create_render_pass<'r>(
        &self,
        encoder: &'r mut CommandEncoder,
        view: &'r TextureView,
    ) -> RenderPass<'r> {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
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
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: self.depth_texture.view(),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        })
    }
}

impl Renderer {
    pub fn reconfigure_surface(&mut self) {
        self.render_surface.configure(self.graphics_device.device());
    }

    pub fn surface_size(&self) -> UVec2 {
        self.render_surface.size()
    }

    pub fn surface_resize(&mut self, size: UVec2) {
        self.render_surface
            .resize(self.graphics_device.device(), size);

        self.depth_texture
            .resize(self.graphics_device.device(), size);
    }
}
