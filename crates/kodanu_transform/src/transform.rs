use kodanu_math::{Mat4, Quat, Vec3};

#[derive(Debug, Clone)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl Transform {
    #[inline]
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        self.matrix().inverse()
    }

    #[inline]
    pub fn point(&self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (point * self.scale)
    }

    #[inline]
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    #[inline]
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    #[inline]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    #[inline]
    pub fn scale(&self) -> Vec3 {
        self.scale
    }
}

impl Transform {
    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    #[inline]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}

impl Transform {
    #[inline]
    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }

    #[inline]
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation *= rotation;
    }
}

impl Transform {
    #[inline]
    pub fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    #[inline]
    pub fn from_rotation(rotation: Quat) -> Self {
        Self::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    #[inline]
    pub fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self::new(position, rotation, Vec3::ONE)
    }
}
