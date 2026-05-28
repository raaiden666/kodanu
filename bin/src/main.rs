use engine::*;

use env_logger::init;

use anyhow::{Ok, Result};

use window::NativeWindowConfig;

const WINDOW_TITLE: &str = "REngine";
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

fn main() -> Result<()> {
    init();

    let window_config = NativeWindowConfig::default()
        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_title(WINDOW_TITLE);

    let mut app = App::new(window_config);

    let event_loop = create_event_loop()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
