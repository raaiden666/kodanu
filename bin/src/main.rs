use {anyhow::Result, app::App};

use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::new("info").add_directive("wgpu_hal=error".parse().unwrap()))
        .init();

    App::run()?;

    Ok(())
}
