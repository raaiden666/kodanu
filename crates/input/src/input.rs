use crate::{KeyCode, MouseButton, button_state::ButtonState};

use types::Vec2;

pub struct Input {
    keyboard: ButtonState<KeyCode>,
    mouse: ButtonState<MouseButton>,

    mouse_position: Vec2<f64>,
    mouse_wheel_delta: f32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keyboard: ButtonState::new(),
            mouse: ButtonState::new(),

            mouse_position: Vec2::new(0.0, 0.0),
            mouse_wheel_delta: 0.0,
        }
    }
}

impl Input {
    pub fn begin_frame(&mut self) {
        self.keyboard.begin_frame();
        self.mouse.begin_frame();

        self.mouse_wheel_delta = 0.0;
    }

    pub fn press_key(&mut self, key: KeyCode) {
        self.keyboard.press(key);
    }

    pub fn release_key(&mut self, key: KeyCode) {
        self.keyboard.release(key);
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_released(key)
    }

    pub fn press_mouse_button(&mut self, button: MouseButton) {
        self.mouse.press(button);
    }

    pub fn release_mouse_button(&mut self, button: MouseButton) {
        self.mouse.release(button);
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse.is_pressed(button)
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse.is_just_pressed(button)
    }

    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.mouse.is_just_released(button)
    }

    pub fn set_mouse_position(&mut self, position: Vec2<f64>) {
        self.mouse_position = position;
    }

    pub fn get_mouse_position(&self) -> Vec2<f64> {
        self.mouse_position
    }

    pub fn add_mouse_wheel_delta(&mut self, wheel: f32) {
        self.mouse_wheel_delta += wheel;
    }

    pub fn get_mouse_wheel_delta(&self) -> f32 {
        self.mouse_wheel_delta
    }
}
