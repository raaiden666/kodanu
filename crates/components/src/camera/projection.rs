use crate::PerspectiveProjection;

use math::Mat4;

pub enum Projection {
    Perspective(PerspectiveProjection),
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(PerspectiveProjection::default())
    }
}

impl Projection {
    pub fn matrix(&self) -> Mat4 {
        match self {
            Projection::Perspective(projection) => projection.matrix(),
        }
    }
}
