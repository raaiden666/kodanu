use {
    assets::{Material, Mesh},
    std::sync::Arc,
};

pub struct MeshRenderer {
    mesh: Arc<Mesh>,
    material: Arc<Material>,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, material: Material) -> Self {
        Self {
            mesh: Arc::new(mesh),
            material: Arc::new(material),
        }
    }
}

impl MeshRenderer {
    pub fn mesh(&self) -> &Mesh {
        &self.mesh.as_ref()
    }

    pub fn mesh_handle(&self) -> Arc<Mesh> {
        Arc::clone(&self.mesh)
    }

    pub fn material(&self) -> &Material {
        &self.material.as_ref()
    }

    pub fn material_hanlde(&self) -> Arc<Material> {
        Arc::clone(&self.material)
    }
}
