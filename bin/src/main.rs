use {
    anyhow::{Ok, Result},
    app::{App, Time},
    input::Input,
    window::WindowConfig,
};

fn main() -> Result<()> {
    let window_config = WindowConfig::default();
    let input = Input::default();
    let time = Time::default();

    App::run(window_config, input, time)?;

    Ok(())
}
