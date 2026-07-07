use crate::{CameraRenderer, ModelSrorageBuffer, gpu::GraphicsDevice, material::MaterialLayout};

pub(crate) struct FrameResources {
    camera_renderer: CameraRenderer,
    model_storage: ModelSrorageBuffer,
    material_layout: MaterialLayout,
}

impl FrameResources {
    pub fn new(graphics_device: &GraphicsDevice) -> Self {
        let camera_renderer = CameraRenderer::new(graphics_device);

        let model_storage = ModelSrorageBuffer::new(graphics_device, 10_000);

        let material_layout = MaterialLayout::new(graphics_device);

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
