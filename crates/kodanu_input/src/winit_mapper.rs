use crate::{KeyCode, MouseKey};

use winit::{event::MouseButton as WinitMouseButton, keyboard::KeyCode as WinitKeyCode};

pub(crate) fn map_key_code(key: WinitKeyCode) -> Option<KeyCode> {
    match key {
        WinitKeyCode::KeyW => Some(KeyCode::W),
        WinitKeyCode::KeyA => Some(KeyCode::A),
        WinitKeyCode::KeyS => Some(KeyCode::S),
        WinitKeyCode::KeyD => Some(KeyCode::D),
        WinitKeyCode::KeyZ => Some(KeyCode::Z),
        WinitKeyCode::KeyX => Some(KeyCode::X),
        WinitKeyCode::KeyC => Some(KeyCode::C),
        WinitKeyCode::KeyH => Some(KeyCode::H),
        WinitKeyCode::KeyJ => Some(KeyCode::J),
        WinitKeyCode::KeyK => Some(KeyCode::K),
        WinitKeyCode::KeyL => Some(KeyCode::L),
        WinitKeyCode::KeyQ => Some(KeyCode::Q),
        WinitKeyCode::KeyE => Some(KeyCode::E),
        WinitKeyCode::Space => Some(KeyCode::Space),
        WinitKeyCode::Escape => Some(KeyCode::Escape),
        WinitKeyCode::ShiftLeft => Some(KeyCode::LeftShift),
        WinitKeyCode::ControlRight => Some(KeyCode::LeftCtrl),
        WinitKeyCode::Enter => Some(KeyCode::Enter),
        _ => None,
    }
}

pub(crate) fn map_mouse_button(button: WinitMouseButton) -> Option<MouseKey> {
    match button {
        WinitMouseButton::Left => Some(MouseKey::Left),
        WinitMouseButton::Right => Some(MouseKey::Right),
        WinitMouseButton::Middle => Some(MouseKey::Middle),
        _ => None,
    }
}
