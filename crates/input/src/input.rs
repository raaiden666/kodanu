use crate::{KeyCode, MouseButton, button_state::ButtonState, map_key_code, map_mouse_button};

use math::DVec2;

use winit::{
    event::{ElementState, KeyEvent, MouseButton as WinitMouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

pub struct Input {
    keyboard: ButtonState<KeyCode>,
    mouse: ButtonState<MouseButton>,

    mouse_position: DVec2,
    mouse_wheel_delta: f32,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keyboard: ButtonState::new(),
            mouse: ButtonState::new(),

            mouse_position: DVec2::new(0.0, 0.0),
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

    pub fn set_mouse_position(&mut self, position: DVec2) {
        self.mouse_position = position;
    }

    pub fn get_mouse_position(&self) -> DVec2 {
        self.mouse_position
    }

    pub fn add_mouse_wheel_delta(&mut self, wheel: f32) {
        self.mouse_wheel_delta += wheel;
    }

    pub fn get_mouse_wheel_delta(&self) -> f32 {
        self.mouse_wheel_delta
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

    pub fn handle_mouse_input(&mut self, state: ElementState, button: WinitMouseButton) {
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
            MouseScrollDelta::LineDelta(_, y) => {
                self.add_mouse_wheel_delta(y);
            }
            MouseScrollDelta::PixelDelta(position) => {
                self.add_mouse_wheel_delta(position.y as f32);
            }
        }
    }
}
