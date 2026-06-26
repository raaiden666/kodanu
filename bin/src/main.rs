use {anyhow::Result, tracing_subscriber::EnvFilter, tracing_subscriber::fmt};

use {kodanu_app::App, kodanu_window::WindowConfig};

fn main() -> Result<()> {
    let wgpu_hal_filter = EnvFilter::new("info")
        .add_directive("wgpu_hal=error".parse().unwrap())
        .add_directive("calloop=off".parse().unwrap());

    fmt().with_env_filter(wgpu_hal_filter).init();

    let window_config = WindowConfig::default()
        .with_title("Kodanu")
        .with_decorations(false);

    App::default().with_window_config(window_config).run()?;

    Ok(())
}
