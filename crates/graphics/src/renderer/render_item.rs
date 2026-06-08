use crate::resources::GpuMesh;

use {math::Mat4, std::sync::Arc};

pub struct RenderItem {
    mesh: Arc<GpuMesh>,
    model: Mat4,
}

impl RenderItem {
    pub fn new(mesh: Arc<GpuMesh>, model: Mat4) -> Self {
        Self { mesh, model }
    }
}

impl RenderItem {
    pub fn mesh(&self) -> &GpuMesh {
        &self.mesh
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }
}
