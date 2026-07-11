use crate::{
    CameraRenderer, ModelSrorageBuffer, ModelUniform, gpu::GraphicsDevice, material::MaterialLayout,
};

use {kodanu_math::Mat4, wgpu::Queue};

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
    pub fn update(&self, queue: &Queue, view_projection: Mat4, models: &[ModelUniform]) {
        self.camera_renderer.update(queue, view_projection);
        self.model_storage.update(queue, models);
    }

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
