use app::engine::REngine;

use window::config::NativeWindowConfig;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    REngine::run(NativeWindowConfig::default())?;

    Ok(())
}
