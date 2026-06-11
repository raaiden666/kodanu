use {
    components::{Camera, Transform},
    input::{Input, KeyCode},
    math::{Mat4, Quat, Vec3},
    time::Time,
};

pub struct SceneCamera {
    camera: Camera,
    transform: Transform,
    move_speed: f32,
    look_speed: f32,
}

impl SceneCamera {
    pub const DEFAULT_MOVE_SPEED: f32 = 3.0;
    pub const DEFAULT_LOOK_SPEED: f32 = 125.0;
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self::new(
            Camera::default(),
            Transform::default(),
            Self::DEFAULT_MOVE_SPEED,
            Self::DEFAULT_LOOK_SPEED,
        )
    }
}

impl SceneCamera {
    pub fn new(camera: Camera, transform: Transform, move_speed: f32, look_speed: f32) -> Self {
        Self {
            camera,
            transform,
            move_speed,
            look_speed: look_speed.to_radians(),
        }
    }
}

impl SceneCamera {
    pub fn update(&mut self, input: &Input, time: &Time) {
        let mut direction = Vec3::ZERO;

        {
            if input.key_pressed(KeyCode::W) {
                direction += self.transform.forward();
            }
            if input.key_pressed(KeyCode::S) {
                direction -= self.transform.forward();
            }

            if input.key_pressed(KeyCode::A) {
                direction -= self.transform.right();
            }
            if input.key_pressed(KeyCode::D) {
                direction += self.transform.right();
            }

            if input.key_pressed(KeyCode::J) {
                direction += self.transform.up()
            }
            if input.key_pressed(KeyCode::K) {
                direction -= self.transform.up()
            }
        }

        {
            if input.key_pressed(KeyCode::H) {
                self.transform.set_rotation(
                    Quat::from_rotation_y(self.look_speed * time.delta_time())
                        * self.transform.rotation(),
                );
            }
            if input.key_pressed(KeyCode::L) {
                self.transform.set_rotation(
                    Quat::from_rotation_y(-self.look_speed * time.delta_time())
                        * self.transform.rotation(),
                );
            }

            if input.key_pressed(KeyCode::Q) {
                self.transform
                    .rotate(Quat::from_rotation_x(self.look_speed * time.delta_time()));
            }
            if input.key_pressed(KeyCode::E) {
                self.transform
                    .rotate(Quat::from_rotation_x(-self.look_speed * time.delta_time()));
            }
        }

        if direction.length_squared() > 0.0 {
            self.transform
                .translate(direction.normalize() * self.move_speed * time.delta_time());
        }
    }

    pub fn view_projection(&self) -> Mat4 {
        self.camera.projection_matrix() * self.transform().view_matrix()
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
