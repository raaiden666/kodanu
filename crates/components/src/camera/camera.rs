use crate::Projection;

use math::Mat4;

pub struct Camera {
    projection: Projection,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Projection::default())
    }
}

impl Camera {
    pub fn new(projection: Projection) -> Self {
        Self { projection }
    }
}

impl Camera {
    pub fn projection(&self) -> &Projection {
        &self.projection
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        match &mut self.projection {
            Projection::Perspective(projection) => projection.set_aspect_ratio(aspect_ratio),
        }
    }
}
