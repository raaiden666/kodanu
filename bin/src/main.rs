use app::engine::REngine;

use window::config::NativeWindowConfig;

use input::winit::InputState;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let window_config = NativeWindowConfig::default();
    let window_input = InputState::default();

    REngine::run(window_config, window_input)?;

    Ok(())
}
