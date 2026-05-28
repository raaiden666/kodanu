use std::{iter::once, sync::Arc};

use wgpu::{
    BackendOptions, Backends, Color, CommandEncoderDescriptor, CurrentSurfaceTexture,
    DeviceDescriptor, Instance, InstanceDescriptor, InstanceFlags, LoadOp, MemoryBudgetThresholds,
    Operations, PowerPreference, PresentMode, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterOptions, StoreOp, SurfaceConfiguration, SurfaceTexture, TextureUsages,
    TextureViewDescriptor,
};

use window::native::NativeWindow;

use winit::dpi::PhysicalSize;

use crate::{rendering::GraphicsDevice, rendering::RenderResult, rendering::SurfaceContext};

use crate::rendering::render_errors::{
    FAILED_TO_CREATE_ADAPTER, FAILED_TO_CREATE_DEVICE, RENDER_ENCODER_LABEL, RENDER_PASS_LABEL,
};

pub struct Renderer {
    graphics_device: Arc<GraphicsDevice>,
    surface_context: SurfaceContext,
}

impl Renderer {
    pub async fn new(window: &NativeWindow) -> Self {
        let instance = Instance::new({
            InstanceDescriptor {
                backends: Backends::VULKAN | Backends::METAL,
                flags: if cfg!(debug_assertions) {
                    InstanceFlags::default()
                } else {
                    InstanceFlags::empty()
                },
                memory_budget_thresholds: MemoryBudgetThresholds::default(),
                backend_options: BackendOptions::default(),
                display: None,
            }
        });

        let surface = window.create_surface(&instance);

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect(FAILED_TO_CREATE_ADAPTER);

        let adapter_info = adapter.get_info();

        println!("GPU: {:?}", adapter_info.name);
        println!("Backend: {:?}", adapter_info.backend);
        println!("Driver: {:?}", adapter_info.driver_info);

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .expect(FAILED_TO_CREATE_DEVICE);

        let graphics_device = Arc::new(GraphicsDevice::new(device, queue));

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities.formats[0];

        let size = window.size();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![format],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&graphics_device.device(), &config);

        let surface_context = SurfaceContext::new(surface, config, size);

        Self {
            graphics_device,
            surface_context,
        }
    }

    pub fn render(&self) -> RenderResult {
        let frame = match self.surface_context.acquire_frame() {
            CurrentSurfaceTexture::Success(frame) => frame,
            CurrentSurfaceTexture::Suboptimal(frame) => {
                self.render_frame(&frame);
                frame.present();
                return RenderResult::Suboptimal;
            }
            CurrentSurfaceTexture::Timeout => {
                return RenderResult::Timeout;
            }
            CurrentSurfaceTexture::Occluded => {
                return RenderResult::Occluded;
            }
            CurrentSurfaceTexture::Outdated => {
                return RenderResult::Outdated;
            }
            CurrentSurfaceTexture::Lost => {
                return RenderResult::Lost;
            }
            CurrentSurfaceTexture::Validation => {
                return RenderResult::Validation;
            }
        };

        self.render_frame(&frame);
        frame.present();

        return RenderResult::Success;
    }

    pub fn render_frame(&self, frame: &SurfaceTexture) {
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.graphics_device
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some(RENDER_ENCODER_LABEL),
                });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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

    pub fn resize(&mut self) {
        self.surface_context
            .configure(&self.graphics_device.device());
    }

    pub fn surface_size(&mut self) -> PhysicalSize<u32> {
        self.surface_context.size()
    }

    pub fn surface_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_context
            .resize(&self.graphics_device.device(), size);
    }
}
