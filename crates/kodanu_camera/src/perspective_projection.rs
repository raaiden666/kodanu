use kodanu_math::Mat4;

pub struct PerspectiveProjection {
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl PerspectiveProjection {
    pub const DEFAULT_FOV: f32 = 90.0;
    pub const DEFAULT_ASPECT_RATIO: f32 = 1.0;
    pub const DEFAULT_NEAR: f32 = 0.03;
    pub const DEFAULT_FAR: f32 = 1000.0;
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_FOV,
            Self::DEFAULT_ASPECT_RATIO,
            Self::DEFAULT_NEAR,
            Self::DEFAULT_FAR,
        )
    }
}

impl PerspectiveProjection {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            fov: fov.to_radians(),
            aspect_ratio,
            near,
            far,
        }
    }
}

impl PerspectiveProjection {
    pub fn fov(&self) -> f32 {
        self.fov.to_radians()
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }
}

impl PerspectiveProjection {
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }
}
