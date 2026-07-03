use {anyhow::Result, kodanu_app::App, kodanu_log::LogConfig, kodanu_window::WindowConfig};

fn main() -> Result<()> {
    let log_config = LogConfig::default()
        .with_directive("wgpu_hal=error")
        .with_directive("calloop=off");

    let window_config = WindowConfig::default()
        .with_title("Kodanu")
        .with_decorations(false);

    App::default()
        .with_window_config(window_config)
        .with_log_config(log_config)
        .run()?;

    Ok(())
}
