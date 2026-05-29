use app::engine::REngine;

use window::config::NativeWindowConfig;

use anyhow::{Ok, Result};

use env_logger::init;

fn main() -> Result<()> {
    init();

    REngine::run(NativeWindowConfig::default())?;

    Ok(())
}
