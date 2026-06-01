use crate::camera::PerspectiveProjection;

use glam::Mat4;

pub enum Projection {
    Perspective(PerspectiveProjection),
}

impl Projection {
    pub fn matrix(&self) -> Mat4 {
        match self {
            Projection::Perspective(projection) => projection.matrix(),
        }
    }
}
