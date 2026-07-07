#![allow(dead_code)]

use crate::{AppConfig, engine::Engine};

use {
    anyhow::{Ok, Result},
    kodanu_window::Window,
    std::sync::Arc,
};

use winit::event_loop::ActiveEventLoop;

pub(crate) struct AppRuntime {
    window: Window,
    engine: Engine,
}

impl AppRuntime {
    pub fn new(event_loop: &ActiveEventLoop, config: &AppConfig) -> Result<Self> {
        let window = event_loop
            .create_window(config.window_config().to_attributes())
            .expect("Failed to create window");

        let window = Window::new(Arc::new(window));
        let engine = Engine::new(&window, config.renderer_config());

        window.request_redraw();

        Ok(Self { window, engine })
    }
}

impl AppRuntime {
    #[inline]
    pub fn window(&self) -> &Window {
        &self.window
    }

    #[inline]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    #[inline]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    #[inline]
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}
