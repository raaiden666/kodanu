use app::engine::{REngine, create_event_loop};

use window::config::NativeWindowConfig;

use anyhow::{Ok, Result};

use env_logger::init;

fn main() -> Result<()> {
    init();

    let window_config = NativeWindowConfig::default();

    let mut rengine = REngine::new(window_config);

    let event_loop = create_event_loop()?;

    event_loop.run_app(&mut rengine)?;

    Ok(())
}
