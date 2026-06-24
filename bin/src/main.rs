use {anyhow::Result, app::App};

use tracing_subscriber::{EnvFilter, fmt};

fn main() -> Result<()> {
    let wgpu_hal_filter = EnvFilter::new("info")
        .add_directive("wgpu_hal=error".parse().unwrap())
        .add_directive("calloop=off".parse().unwrap());

    fmt().with_env_filter(wgpu_hal_filter).init();

    let mut app = App::default();

    app.run()?;

    Ok(())
}
