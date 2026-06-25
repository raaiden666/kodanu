use {
    codanu_transform::Transform,
    components::Camera,
    input::{ActionMap, Axis, Input},
    math::{EulerRot, Mat4, Quat},
    time::Time,
};

pub struct SceneCamera {
    camera: Camera,
    transform: Transform,
    move_speed: f32,
    look_speed: f32,
    yaw: f32,
    pitch: f32,
}

impl SceneCamera {
    pub const DEFAULT_MOVE_SPEED: f32 = 6.0;
    pub const DEFAULT_LOOK_SPEED: f32 = 110.0;
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
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl SceneCamera {
    pub fn update(&mut self, input: &Input, action_map: &ActionMap, time: &Time) {
        let direction = self.transform.forward() * action_map.axis(Axis::MoveY, input)
            + -self.transform.right() * action_map.axis(Axis::MoveX, input)
            + self.transform.up() * action_map.axis(Axis::MoveZ, input);

        self.transform
            .translate(direction * self.move_speed * time.delta());

        self.yaw += action_map.axis(Axis::LookX, input) * self.look_speed * time.delta();
        self.pitch += action_map.axis(Axis::LookY, input) * self.look_speed * time.delta();

        self.transform
            .set_rotation(Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0));
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
