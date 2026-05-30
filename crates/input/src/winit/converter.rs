use crate::{winit::KeyCode, winit::MouseButton};

use winit::{event::MouseButton as WinitMouseButton, keyboard::KeyCode as WinitKeyCode};

pub fn map_key_code(key: WinitKeyCode) -> Option<KeyCode> {
    match key {
        WinitKeyCode::KeyW => Some(KeyCode::W),
        WinitKeyCode::KeyA => Some(KeyCode::A),
        WinitKeyCode::KeyS => Some(KeyCode::S),
        WinitKeyCode::KeyD => Some(KeyCode::D),
        WinitKeyCode::KeyZ => Some(KeyCode::Z),
        WinitKeyCode::KeyX => Some(KeyCode::X),
        WinitKeyCode::KeyC => Some(KeyCode::C),
        WinitKeyCode::Space => Some(KeyCode::Space),
        WinitKeyCode::Escape => Some(KeyCode::Escape),
        WinitKeyCode::ShiftLeft => Some(KeyCode::LeftShift),
        WinitKeyCode::ControlRight => Some(KeyCode::LeftCtrl),
        WinitKeyCode::Enter => Some(KeyCode::Enter),
        _ => None,
    }
}

pub fn map_mouse_button(button: WinitMouseButton) -> Option<MouseButton> {
    match button {
        WinitMouseButton::Left => Some(MouseButton::Left),
        WinitMouseButton::Right => Some(MouseButton::Right),
        WinitMouseButton::Middle => Some(MouseButton::Middle),
        _ => None,
    }
}
