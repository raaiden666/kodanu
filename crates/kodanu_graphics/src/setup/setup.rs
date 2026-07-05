use crate::gpu::{GraphicsDevice, RenderSurface};

use {kodanu_math::UVec2, kodanu_window::Window};

use wgpu::{
    Adapter, BackendOptions, Backends, CompositeAlphaMode, Device, DeviceDescriptor,
    ExperimentalFeatures, Features, Instance, InstanceDescriptor, InstanceFlags, Limits,
    MemoryBudgetThresholds, MemoryHints, PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceColorSpace, SurfaceConfiguration, TextureFormat,
    TextureUsages, Trace,
};

pub(crate) fn create_instance() -> Instance {
    Instance::new({
        InstanceDescriptor {
            backends: Backends::VULKAN | Backends::METAL | Backends::DX12,
            flags: InstanceFlags::default(),
            memory_budget_thresholds: MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
            display: None,
        }
    })
}

pub(crate) async fn create_adapter(instance: &Instance, surface: &Surface<'_>) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(surface),
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        })
        .await
        .expect("Failed to create adapter")
}

pub(crate) async fn create_device(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(&DeviceDescriptor {
            label: Some("Device"),
            memory_hints: MemoryHints::Performance,
            required_features: Features::empty(),
            experimental_features: ExperimentalFeatures::disabled(),
            required_limits: Limits::defaults(),
            trace: Trace::Off,
        })
        .await
        .expect("Failed to create device")
}

pub(crate) fn create_surface_configuration(
    size: UVec2,
    texture_format: TextureFormat,
    composite_alpha_mode: CompositeAlphaMode,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: texture_format,
        width: size.x,
        height: size.y,
        present_mode: PresentMode::Fifo,
        alpha_mode: composite_alpha_mode,
        view_formats: vec![texture_format],
        desired_maximum_frame_latency: 2,
        color_space: SurfaceColorSpace::Srgb,
    }
}

pub(crate) async fn create_device_and_surface(
    window: &Window,
) -> (GraphicsDevice, Surface<'static>) {
    let instance = create_instance();

    let surface = window.create_surface(&instance);

    let adapter = create_adapter(&instance, &surface).await;

    let (device, queue) = create_device(&adapter).await;

    let graphics_device = GraphicsDevice::new(adapter, device, queue);

    (graphics_device, surface)
}

pub fn create_render_surface(
    window: &Window,
    graphics_device: &GraphicsDevice,
    surface: Surface<'static>,
) -> RenderSurface {
    let capabilities = surface.get_capabilities(graphics_device.adapter());

    let format = capabilities
        .formats
        .iter()
        .copied()
        .find(TextureFormat::is_srgb)
        .unwrap_or(capabilities.formats[0]);

    let size = window.size();

    let config = create_surface_configuration(size, format, capabilities.alpha_modes[0]);

    surface.configure(graphics_device.device(), &config);

    RenderSurface::new(surface, config, size)
}
