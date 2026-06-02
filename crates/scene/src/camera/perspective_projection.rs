use glam::Mat4;

pub struct PerspectiveProjection {
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl PerspectiveProjection {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            aspect_ratio,
            near,
            far,
        }
    }
}

impl PerspectiveProjection {
    pub fn matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }
}
