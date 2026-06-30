use crate::{Editor, Engine};

use {
    kodanu_input::{
        handle_cursor_move, handle_keyboard_input, handle_mouse_input, handle_mouse_wheel,
    },
    tracing::info,
};

use {
    anyhow::{Ok, Result},
    kodanu_input::KeyCode,
    kodanu_log::LogConfig,
    kodanu_math::{DVec2, UVec2},
    kodanu_window::{Window, WindowConfig},
    std::sync::Arc,
    tracing_subscriber::fmt,
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    engine: Option<Engine>,
    editor: Editor,
    window_config: WindowConfig,
    log_config: LogConfig,
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;

        info!(target: "App::Run()", "App started");

        event_loop.run_app(self)?;

        Ok(())
    }
}

impl App {
    pub fn with_window_config(mut self, config: WindowConfig) -> Self {
        self.window_config = config;
        self
    }

    pub fn with_log_config(mut self, config: LogConfig) -> Self {
        self.log_config = config;
        self
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        fmt().with_env_filter(self.log_config.env_filter()).init();

        let raw_window = event_loop
            .create_window(self.window_config.to_attributes())
            .expect("Failed to create native window");

        let window = Window::new(Arc::new(raw_window));
        let engine = Engine::new(&window);

        window.request_redraw();

        self.window = Some(window);
        self.engine = Some(engine);

        self.editor.init_test_mesh();

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

        match &event {
            WindowEvent::CloseRequested => {
                info!("App::Window_Event(), CloseRequested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.frame(event_loop);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                engine
                    .renderer_mut()
                    .surface_resize(UVec2::new(size.width, size.height));

                self.editor
                    .scene_camera_mut()
                    .camera_mut()
                    .set_viewport_size(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                handle_keyboard_input(engine.input_mut(), event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                handle_mouse_input(engine.input_mut(), *state, *button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                handle_cursor_move(engine.input_mut(), DVec2::new(position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                handle_mouse_wheel(engine.input_mut(), *delta);
            }
            _ => {}
        }
    }
}

impl App {
    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let Some(engine) = &mut self.engine else {
            return;
        };

        engine.time_update();

        self.editor
            .update(engine.input(), engine.action_map(), engine.time());

        if engine.input().key_just_pressed(KeyCode::Escape) {
            info!(target: "App::Frame()", "App closed");
            event_loop.exit();
        }

        if engine.input().key_just_pressed(KeyCode::Enter) {
            info!(target: "App::Frame()", "{:#?}", self.window_config)
        }

        engine.render(
            self.editor.scene_camera().view_projection(),
            &self.editor.collect_render_items(),
        );

        engine.begin_frame();
    }
}
