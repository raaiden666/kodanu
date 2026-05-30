use crate::winit::{InputState, map_key_code, map_mouse_button};

use primitives::winit::Vec2f64;

use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

pub fn handle_keyboard_input(event: KeyEvent, input: &mut InputState) {
    let PhysicalKey::Code(key_code) = event.physical_key else {
        return;
    };

    let Some(key_code) = map_key_code(key_code) else {
        return;
    };

    match event.state {
        ElementState::Pressed => {
            input.press_key(key_code);
        }
        ElementState::Released => {
            input.release_key(key_code);
        }
    }
}

pub fn handle_mouse_input(state: ElementState, button: MouseButton, input: &mut InputState) {
    let Some(button) = map_mouse_button(button) else {
        return;
    };

    match state {
        ElementState::Pressed => {
            input.press_mouse_button(button);
        }
        ElementState::Released => {
            input.release_mouse_button(button);
        }
    }
}

pub fn handle_cursor_move(position: Vec2f64, input: &mut InputState) {
    input.set_mouse_position(position);
}

pub fn handle_mouse_wheel(delta: MouseScrollDelta, input: &mut InputState) {
    match delta {
        MouseScrollDelta::LineDelta(_, y) => {
            input.add_mouse_wheel_delta(y);
        }
        MouseScrollDelta::PixelDelta(position) => {
            input.add_mouse_wheel_delta(position.y as f32);
        }
    }
}
