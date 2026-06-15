use crate::Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

impl Mesh {
    pub fn triangle_2d() -> Self {
        Mesh::new(
            vec![
                Vertex::new([-0.5, -0.5, 0.0]),
                Vertex::new([0.5, -0.5, 0.0]),
                Vertex::new([0.0, 0.5, 0.0]),
            ],
            vec![0, 1, 2],
        )
    }

    pub fn cube_2d() -> Self {
        Mesh::new(
            vec![
                Vertex::new([-0.5, -0.5, 0.0]),
                Vertex::new([0.5, -0.5, 0.0]),
                Vertex::new([0.5, 0.5, 0.0]),
                Vertex::new([-0.5, 0.5, 0.0]),
            ],
            vec![0, 1, 2, 2, 3, 0],
        )
    }
}
