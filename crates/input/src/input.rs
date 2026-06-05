use crate::{KeyCode, MouseKey, button_state::ButtonState, map_key_code, map_mouse_button};

use math::{DVec2, Vec2};

use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

pub struct Input {
    keyboard: ButtonState<KeyCode>,
    mouse: ButtonState<MouseKey>,

    mouse_position: DVec2,
    mouse_wheel_delta: Vec2,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keyboard: ButtonState::default(),
            mouse: ButtonState::default(),

            mouse_position: DVec2::ZERO,
            mouse_wheel_delta: Vec2::ZERO,
        }
    }
}

impl Input {
    pub fn begin_frame(&mut self) {
        self.keyboard.begin_frame();
        self.mouse.begin_frame();

        self.mouse_wheel_delta = Vec2::ZERO;
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_pressed(key)
    }

    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_pressed(key)
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_just_released(key)
    }

    pub fn button_pressed(&self, button: MouseKey) -> bool {
        self.mouse.is_pressed(button)
    }

    pub fn button_just_pressed(&self, button: MouseKey) -> bool {
        self.mouse.is_just_pressed(button)
    }

    pub fn button_just_released(&self, button: MouseKey) -> bool {
        self.mouse.is_just_released(button)
    }
}

impl Input {
    pub fn set_mouse_position(&mut self, position: DVec2) {
        self.mouse_position = position;
    }

    pub fn mouse_position(&self) -> DVec2 {
        self.mouse_position
    }

    pub fn add_mouse_wheel_delta(&mut self, x: f32, y: f32) {
        self.mouse_wheel_delta += Vec2::new(x, y);
    }

    pub fn mouse_wheel_delta(&self) -> Vec2 {
        self.mouse_wheel_delta
    }
}

impl Input {
    fn press_key(&mut self, key: KeyCode) {
        self.keyboard.press(key);
    }

    fn release_key(&mut self, key: KeyCode) {
        self.keyboard.release(key);
    }

    fn press_mouse_button(&mut self, button: MouseKey) {
        self.mouse.press(button);
    }

    fn release_mouse_button(&mut self, button: MouseKey) {
        self.mouse.release(button);
    }
}

impl Input {
    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(key_code) = event.physical_key else {
            return;
        };

        let Some(key_code) = map_key_code(key_code) else {
            return;
        };

        match event.state {
            ElementState::Pressed => {
                self.press_key(key_code);
            }
            ElementState::Released => {
                self.release_key(key_code);
            }
        }
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let Some(button) = map_mouse_button(button) else {
            return;
        };

        match state {
            ElementState::Pressed => {
                self.press_mouse_button(button);
            }
            ElementState::Released => {
                self.release_mouse_button(button);
            }
        }
    }

    pub fn handle_cursor_move(&mut self, position: DVec2) {
        self.set_mouse_position(position);
    }

    pub fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.add_mouse_wheel_delta(x, y);
            }
            MouseScrollDelta::PixelDelta(delta) => {
                self.add_mouse_wheel_delta(delta.y as f32, delta.x as f32);
            }
        }
    }
}
