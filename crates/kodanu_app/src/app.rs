use crate::{AppConfig, AppRuntime, Editor};

use {
    kodanu_input::KeyCode,
    kodanu_log::LogConfig,
    kodanu_math::{DVec2, UVec2},
    kodanu_window::WindowConfig,
    tracing_subscriber::fmt,
};

use kodanu_input::{
    handle_cursor_move, handle_keyboard_input, handle_mouse_input, handle_mouse_wheel,
};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

#[derive(Default)]
pub struct App {
    runtime: Option<AppRuntime>,
    config: AppConfig,
    editor: Editor,
}

impl App {
    pub fn run(&mut self) {
        let log_config = self.config.log_config();
        let event_loop = EventLoop::new().expect("Failed to create event loop");

        fmt().with_env_filter(log_config.env_filter()).init();

        self.editor.init_test_mesh();

        event_loop.run_app(self).expect("Failed to run app");
    }

    pub fn with_window_config(mut self, config: WindowConfig) -> Self {
        self.config.set_window_config(config);
        self
    }

    pub fn with_log_config(mut self, config: LogConfig) -> Self {
        self.config.set_log_config(config);
        self
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.runtime.is_some() {
            return;
        }

        self.runtime =
            Some(AppRuntime::new(event_loop, &self.config).expect("Failed to create app"));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.handle_redraw(event_loop),
            WindowEvent::Resized(size) => self.handle_resize(size),
            _ => self.handle_input(event),
        }
    }
}

impl App {
    fn handle_redraw(&mut self, event_loop: &ActiveEventLoop) {
        let Some(runtime) = &mut self.runtime else {
            return;
        };

        runtime.window_mut().request_redraw();

        let engine = runtime.engine_mut();

        engine.time_update();

        self.editor
            .update(engine.input(), engine.action_map(), engine.time());

        if engine.input().key_just_pressed(KeyCode::Escape) {
            event_loop.exit();
        }

        engine.render(
            self.editor.scene_camera().view_projection(),
            &self.editor.collect_render_items(),
        );

        engine.begin_frame();
    }

    fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        let Some(runtime) = &mut self.runtime else {
            return;
        };

        let engine = runtime.engine_mut();

        engine
            .renderer_mut()
            .surface_resize(UVec2::new(size.width, size.height));

        self.editor
            .scene_camera_mut()
            .camera_mut()
            .set_viewport_size(size.width, size.height);
    }

    fn handle_input(&mut self, event: WindowEvent) {
        let Some(runtime) = &mut self.runtime else {
            return;
        };

        let engine = runtime.engine_mut();

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                handle_keyboard_input(engine.input_mut(), &event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                handle_mouse_input(engine.input_mut(), state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                handle_cursor_move(engine.input_mut(), DVec2::new(position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                handle_mouse_wheel(engine.input_mut(), delta);
            }
            _ => {}
        }
    }
}
