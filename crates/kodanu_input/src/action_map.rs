use crate::{Input, KeyCode, action::Action, axis::Axis, axis_binding::AxisBinding};

use std::collections::HashMap;

pub struct ActionMap {
    actions: HashMap<Action, Vec<KeyCode>>,
    axis: HashMap<Axis, AxisBinding>,
}

impl Default for ActionMap {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(128);
        let mut axis = HashMap::with_capacity(128);

        actions.insert(Action::MoveForward, [KeyCode::W].to_vec());
        actions.insert(Action::MoveBackward, [KeyCode::S].to_vec());

        actions.insert(Action::MoveLeft, [KeyCode::A].to_vec());
        actions.insert(Action::MoveRight, [KeyCode::D].to_vec());

        actions.insert(Action::MoveUp, [KeyCode::Space].to_vec());
        actions.insert(Action::MoveDown, [KeyCode::LeftCtrl].to_vec());

        axis.insert(Axis::MoveX, AxisBinding::new(KeyCode::A, KeyCode::D));
        axis.insert(Axis::MoveY, AxisBinding::new(KeyCode::W, KeyCode::S));
        axis.insert(Axis::MoveZ, AxisBinding::new(KeyCode::J, KeyCode::K));

        axis.insert(Axis::LookX, AxisBinding::new(KeyCode::H, KeyCode::L));
        axis.insert(Axis::LookY, AxisBinding::new(KeyCode::Q, KeyCode::E));

        Self { actions, axis }
    }
}
impl ActionMap {
    pub fn pressed(&self, action: &Action, input: &Input) -> bool {
        self.actions
            .get(action)
            .is_some_and(|keys| keys.iter().copied().any(|key| input.key_pressed(key)))
    }

    pub fn just_pressed(&self, action: &Action, input: &Input) -> bool {
        self.actions
            .get(action)
            .is_some_and(|keys| keys.iter().copied().any(|key| input.key_just_pressed(key)))
    }

    pub fn axis(&self, axis: Axis, input: &Input) -> f32 {
        let Some(binding) = self.axis.get(&axis) else {
            return 0.0;
        };

        let mut value = 0.0;

        if input.key_pressed(binding.positive()) {
            value += 1.0;
        }

        if input.key_pressed(binding.negative()) {
            value -= 1.0;
        }

        value
    }
}
