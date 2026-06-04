use {anyhow::Result, app::App, window::WindowConfig};

use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::new("info").add_directive("wgpu_hal=error".parse().unwrap()))
        .init();

    let window_config = WindowConfig::default();

    App::run(window_config)?;

    Ok(())
}
