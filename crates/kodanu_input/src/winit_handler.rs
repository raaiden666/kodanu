use crate::{Input, map_key_code, map_mouse_button};

use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

use kodanu_math::DVec2;

#[inline]
pub fn handle_keyboard_input(input: &mut Input, event: &KeyEvent) {
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

#[inline]
pub fn handle_mouse_input(input: &mut Input, state: ElementState, button: MouseButton) {
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

#[inline]
pub fn handle_cursor_move(input: &mut Input, position: DVec2) {
    input.set_mouse_position(position);
}

#[inline]
pub fn handle_mouse_wheel(input: &mut Input, delta: MouseScrollDelta) {
    match delta {
        MouseScrollDelta::LineDelta(x, y) => {
            input.add_mouse_wheel_delta(x, y);
        }
        MouseScrollDelta::PixelDelta(delta) => {
            input.add_mouse_wheel_delta(delta.x as f32, delta.y as f32);
        }
    }
}
