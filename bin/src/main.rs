use {anyhow::Result, app::App};

use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<()> {
    let wgpu_hal_filter = EnvFilter::new("info").add_directive("wgpu_hal=error".parse().unwrap());

    let callop_filter =
        EnvFilter::new("warn").add_directive("calloop::loop_logic=error".parse().unwrap());

    fmt()
        .with_env_filter(wgpu_hal_filter)
        .with_env_filter(callop_filter)
        .init();

    App::run()?;

    Ok(())
}
