use crate::Engine;

use {
    anyhow::{Ok, Result},
    pollster::block_on,
    std::sync::Arc,
    tracing::info,
    window::{Window, WindowConfig},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct App {
    window: Option<Window>,
    engine: Option<Engine>,
    config: WindowConfig,
}

impl App {
    pub fn run(window_config: WindowConfig) -> Result<()> {
        let event_loop = EventLoop::new()?;
        let mut app = Self::new(window_config);

        info!(target: "App::Run()", "App started");

        event_loop.run_app(&mut app)?;

        Ok(())
    }

    fn new(config: WindowConfig) -> Self {
        Self {
            window: None,
            engine: None,
            config: config,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let raw_window = event_loop
            .create_window(self.config.to_attributes())
            .expect("Failed to create native window");

        let window = Window::new(Arc::new(raw_window));
        let engine = block_on(Engine::new(&window));

        window.request_redraw();

        self.window = Some(window);
        self.engine = Some(engine);

        info!(target: "App::Resumed()", "App resumed");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(engine) = &mut self.engine else {
            return;
        };

        let Some(window) = &self.window else {
            return;
        };

        match &event {
            WindowEvent::CloseRequested => {
                info!(target: "App::WindowEvent()", "App closed");

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                engine.frame(event_loop);

                window.request_redraw();
            }
            _ => {
                engine.handle_window_event(&event);
            }
        }
    }
}
