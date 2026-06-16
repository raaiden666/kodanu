mod button_state;
mod input;
mod key_code;
mod mouse_key;
mod winit_handler;
mod winit_mapper;

pub use {button_state::ButtonState, input::Input, key_code::KeyCode, mouse_key::MouseKey};

pub use winit_handler::{
    handle_cursor_move, handle_keyboard_input, handle_mouse_input, handle_mouse_wheel,
};

pub(crate) use winit_mapper::{map_key_code, map_mouse_button};
