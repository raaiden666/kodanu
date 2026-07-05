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

        actions.insert(Action::MoveForward, vec![KeyCode::W]);
        actions.insert(Action::MoveBackward, vec![KeyCode::S]);

        actions.insert(Action::MoveLeft, vec![KeyCode::A]);
        actions.insert(Action::MoveRight, vec![KeyCode::D]);

        actions.insert(Action::MoveUp, vec![KeyCode::Space]);
        actions.insert(Action::MoveDown, vec![KeyCode::LeftCtrl]);

        axis.insert(Axis::MoveX, AxisBinding::new(KeyCode::A, KeyCode::D));
        axis.insert(Axis::MoveY, AxisBinding::new(KeyCode::W, KeyCode::S));
        axis.insert(Axis::MoveZ, AxisBinding::new(KeyCode::J, KeyCode::K));

        axis.insert(Axis::LookX, AxisBinding::new(KeyCode::H, KeyCode::L));
        axis.insert(Axis::LookY, AxisBinding::new(KeyCode::Q, KeyCode::E));

        Self { actions, axis }
    }
}

impl ActionMap {
    #[inline]
    pub fn pressed(&self, action: &Action, input: &Input) -> bool {
        self.actions
            .get(action)
            .is_some_and(|keys| keys.iter().copied().any(|key| input.key_pressed(key)))
    }

    #[inline]
    pub fn just_pressed(&self, action: &Action, input: &Input) -> bool {
        self.actions
            .get(action)
            .is_some_and(|keys| keys.iter().copied().any(|key| input.key_just_pressed(key)))
    }

    #[inline]
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
