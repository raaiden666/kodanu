use crate::res::{DEVICE_LABEL, FAILED_TO_CREATE_ADAPTER, FAILED_TO_CREATE_DEVICE};

use primitives::winit::SizeU32;

use wgpu::{
    Adapter, BackendOptions, Backends, CompositeAlphaMode, Device, DeviceDescriptor,
    ExperimentalFeatures, Features, Instance, InstanceDescriptor, InstanceFlags, Limits,
    MemoryBudgetThresholds, MemoryHints, PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat, TextureUsages, Trace,
};

pub(crate) fn create_instance() -> Instance {
    Instance::new({
        InstanceDescriptor {
            backends: Backends::VULKAN | Backends::METAL,
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
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect(FAILED_TO_CREATE_ADAPTER)
}

pub(crate) async fn create_device(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(&DeviceDescriptor {
            label: Some(DEVICE_LABEL),
            memory_hints: MemoryHints::Performance,
            required_features: Features::empty(),
            experimental_features: ExperimentalFeatures::disabled(),
            required_limits: Limits::defaults(),
            trace: Trace::Off,
        })
        .await
        .expect(FAILED_TO_CREATE_DEVICE)
}

pub(crate) fn create_surface_configuration(
    size: SizeU32,
    format: TextureFormat,
    alpha_mode: CompositeAlphaMode,
) -> SurfaceConfiguration {
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: alpha_mode,
        view_formats: vec![format],
        desired_maximum_frame_latency: 2,
    }
}
