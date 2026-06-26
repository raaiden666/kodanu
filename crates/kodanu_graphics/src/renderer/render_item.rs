use {
    kodanu_assets::{Material, Mesh},
    kodanu_math::Mat4,
    std::sync::Arc,
};

pub struct RenderItem {
    mesh: Arc<Mesh>,
    material: Arc<Material>,
    model: Mat4,
}

impl RenderItem {
    pub fn new(mesh: Arc<Mesh>, material: Arc<Material>, model: Mat4) -> Self {
        Self {
            mesh,
            material,
            model,
        }
    }
}

impl RenderItem {
    pub fn mesh(&self) -> &Mesh {
        self.mesh.as_ref()
    }

    pub fn mesh_handle(&self) -> Arc<Mesh> {
        Arc::clone(&self.mesh)
    }

    pub fn model(&self) -> Mat4 {
        self.model
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_handle(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}
