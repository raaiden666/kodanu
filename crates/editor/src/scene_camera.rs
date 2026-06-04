use {
    components::{Camera, Transform},
    input::{Input, KeyCode},
    math::Vec3,
    time::Time,
};

pub struct SceneCamera {
    camera: Camera,
    transform: Transform,
    move_speed: f32,
}

impl SceneCamera {
    pub const DEFAULT_MOVE_SPEED: f32 = 5.0;
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self::new(
            Camera::default(),
            Transform::default(),
            Self::DEFAULT_MOVE_SPEED,
        )
    }
}

impl SceneCamera {
    pub fn new(camera: Camera, transform: Transform, move_speed: f32) -> Self {
        Self {
            camera,
            transform,
            move_speed,
        }
    }
}

impl SceneCamera {
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl SceneCamera {
    pub fn update(&mut self, input: &Input, time: &Time) {
        let mut direction = Vec3::ZERO;

        if input.is_key_pressed(KeyCode::W) {
            direction += self.transform.forward();
        }
        if input.is_key_pressed(KeyCode::S) {
            direction -= self.transform.forward();
        }

        if input.is_key_pressed(KeyCode::A) {
            direction -= self.transform.right();
        }
        if input.is_key_pressed(KeyCode::D) {
            direction += self.transform.right();
        }

        if input.is_key_pressed(KeyCode::Space) {
            direction += self.transform.up();
        }
        if input.is_key_pressed(KeyCode::LeftCtrl) {
            direction -= self.transform.up();
        }

        if direction.length_squared() > 0.0 {
            self.transform
                .translate(direction.normalize() * self.move_speed * time.delta_time());
        }
    }
}
