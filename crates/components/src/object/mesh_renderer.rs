use graphics::Mesh;

pub struct MeshRenderer {
    pub mesh: Mesh,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> Self {
        Self { mesh }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}
