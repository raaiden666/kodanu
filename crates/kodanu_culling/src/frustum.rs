use crate::{BoundingSphere, Plane};

use kodanu_math::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    left: Plane,
    right: Plane,
    top: Plane,
    bottom: Plane,
    near: Plane,
    far: Plane,
}

impl Frustum {
    pub fn from_view_projection(matrix: Mat4) -> Self {
        let x = matrix.x_axis;
        let y = matrix.y_axis;
        let z = matrix.z_axis;
        let w = matrix.w_axis;

        let mut left = Plane::from_vec4(w + x);
        let mut right = Plane::from_vec4(w - x);

        let mut bottom = Plane::from_vec4(w + y);
        let mut top = Plane::from_vec4(w - y);

        let mut near = Plane::from_vec4(w + z);
        let mut far = Plane::from_vec4(w - z);

        left.normilize();
        right.normilize();

        bottom.normilize();
        top.normilize();

        near.normilize();
        far.normilize();

        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }
}

impl Frustum {
    pub fn contains_sphere(&self, sphere: &BoundingSphere) -> bool {
        self.left.contains_sphere(sphere)
            && self.right.contains_sphere(sphere)
            && self.bottom.contains_sphere(sphere)
            && self.top.contains_sphere(sphere)
            && self.near.contains_sphere(sphere)
            && self.far.contains_sphere(sphere)
    }
}
