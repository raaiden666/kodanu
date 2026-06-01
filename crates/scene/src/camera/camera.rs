use crate::{camera::Projection, transform::Transform};

use glam::Mat4;

pub struct Camera {
    transform: Transform,
    projection: Projection,
}

impl Camera {
    pub fn new(transform: Transform, projection: Projection) -> Self {
        Self {
            transform,
            projection,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn projection(&self) -> &Projection {
        &self.projection
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.transform.matrix().inverse()
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}
