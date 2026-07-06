use crate::{CameraRenderer, ModelSrorageBuffer, material::MaterialLayout};

use wgpu::Device;

pub(crate) struct FrameResources {
    camera_renderer: CameraRenderer,
    model_storage: ModelSrorageBuffer,
    material_layout: MaterialLayout,
}

impl FrameResources {
    pub fn new(device: &Device) -> Self {
        let camera_renderer = CameraRenderer::new(device);

        let model_storage = ModelSrorageBuffer::new(device, 10_000);

        let material_layout = MaterialLayout::new(device);

        Self {
            camera_renderer,
            model_storage,
            material_layout,
        }
    }
}

impl FrameResources {
    #[inline]
    pub fn camera_renderer(&self) -> &CameraRenderer {
        &self.camera_renderer
    }

    #[inline]
    pub fn model_storage(&self) -> &ModelSrorageBuffer {
        &self.model_storage
    }

    #[inline]
    pub fn material_layout(&self) -> &MaterialLayout {
        &self.material_layout
    }
}
