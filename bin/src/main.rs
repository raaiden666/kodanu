use engine::*;

use env_logger::init;

use anyhow::{Ok, Result};

use window::config::NativeWindowConfig;

fn main() -> Result<()> {
    init();

    let window_config = NativeWindowConfig::default();

    let mut app = App::new(window_config);

    let event_loop = create_event_loop()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
