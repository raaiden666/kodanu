use {kodanu_math::Vec3, kodanu_transform::Transform};

#[derive(Debug, Clone, Copy)]
pub struct BoundingSphere {
    center: Vec3,
    radius: f32,
}

impl BoundingSphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl BoundingSphere {
    pub fn transformed(&self, transform: &Transform) -> Self {
        let max_scale = transform.scale().max_element();

        Self {
            center: transform.point(self.center),
            radius: self.radius * max_scale,
        }
    }

    #[inline]
    pub fn center(&self) -> Vec3 {
        self.center
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }
}
