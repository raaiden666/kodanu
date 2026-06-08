use {assets::Mesh, math::Mat4, std::sync::Arc};

pub struct RenderItem {
    mesh: Arc<Mesh>,
    model: Mat4,
}

impl RenderItem {
    pub fn new(mesh: Arc<Mesh>, model: Mat4) -> Self {
        Self { mesh, model }
    }
}

impl RenderItem {
    pub fn mesh(&self) -> &Mesh {
        &self.mesh.as_ref()
    }

    pub fn mesh_handle(&self) -> Arc<Mesh> {
        Arc::clone(&self.mesh)
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }
}
