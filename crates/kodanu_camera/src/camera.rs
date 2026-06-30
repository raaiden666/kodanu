use crate::Projection;

use kodanu_math::Mat4;

#[derive(Default)]
pub struct Camera {
    projection: Projection,
}

impl Camera {
    pub fn new(projection: Projection) -> Self {
        Self { projection }
    }
}

impl Camera {
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        match &mut self.projection {
            Projection::Perspective(projection) => projection.set_aspect_ratio(aspect_ratio),
        }
    }

    pub fn set_viewport_size(&mut self, width: u32, height: u32) {
        if height == 0 {
            return;
        }

        self.set_aspect_ratio(width as f32 / height as f32);
    }
}

impl Camera {
    pub fn projection(&self) -> &Projection {
        &self.projection
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }
}
