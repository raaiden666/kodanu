use crate::{Editor, Engine};

use {
    anyhow::{Ok, Result},
    input::KeyCode,
    math::DVec2,
    math::UVec2,
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
    editor: Editor,
    config: WindowConfig,
}

impl App {
    pub fn run() -> Result<()> {
        let event_loop = EventLoop::new()?;
        let mut app = Self::new();

        info!(target: "App::Run()", "App started");

        event_loop.run_app(&mut app)?;

        Ok(())
    }

    fn new() -> Self {
        Self {
            window: None,
            engine: None,
            editor: Editor::default(),
            config: WindowConfig::default(),
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
                };
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
                engine.input_mut().handle_keyboard_input(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                engine.input_mut().handle_mouse_input(*state, *button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                engine
                    .input_mut()
                    .handle_cursor_move(DVec2::new(position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                engine.input_mut().handle_mouse_wheel(*delta);
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

        self.editor.update(engine.input(), engine.time());

        if engine.input().key_just_pressed(KeyCode::Escape) {
            info!(target: "App::Frame()", "App closed");
            event_loop.exit();
        }

        engine.render(
            self.editor.scene_camera().view_projection(),
            &self.editor.collect_render_items(),
        );

        engine.begin_frame();
    }
}
