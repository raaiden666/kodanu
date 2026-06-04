use crate::{Editor, Engine};

use {
    anyhow::{Ok, Result},
    pollster::block_on,
    std::sync::Arc,
    tracing::info,
    window::{Window, WindowConfig},
};

use input::KeyCode;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct App {
    window: Option<Window>,
    engine: Option<Engine>,
    editor: Editor,
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
            editor: Editor::default(),
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
        match &event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.frame(event_loop);

                if let Some(window) = &self.window {
                    window.request_redraw();
                };
            }
            _ => {
                if let Some(engine) = &mut self.engine {
                    engine.handle_window_event(&event);
                }
            }
        }
    }
}

impl App {
    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let Some(engine) = &mut self.engine else {
            return;
        };

        engine.time_update();

        self.editor.update(engine.input(), engine.time());

        if engine.input().is_key_just_pressed(KeyCode::Escape) {
            info!(target: "App::Frame()", "App closed");
            event_loop.exit();
        }

        engine.render();

        engine.begin_frame();
    }
}
