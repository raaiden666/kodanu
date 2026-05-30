use app::App;

use window::WindowConfig;

use input::Input;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let window_config = WindowConfig::default();
    let input = Input::default();

    App::run(window_config, input)?;

    Ok(())
}
