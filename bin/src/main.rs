use {anyhow::Result, app::App, window::WindowConfig};

fn main() -> Result<()> {
    let window_config = WindowConfig::default();

    App::run(window_config)?;

    Ok(())
}
