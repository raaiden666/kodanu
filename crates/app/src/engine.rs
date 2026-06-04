use {
    graphics::Renderer,
    input::{Input, KeyCode},
    math::DVec2,
    time::Time,
    window::Window,
};

use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

pub struct Engine {
    renderer: Renderer,
    input: Input,
    time: Time,
}

impl Engine {
    pub async fn new(window: &Window) -> Self {
        Self {
            renderer: Renderer::new(window).await,
            input: Input::default(),
            time: Time::default(),
        }
    }
}

impl Engine {
    pub fn frame(&mut self, event_loop: &ActiveEventLoop) {
        self.time.update();

        self.render();

        self.should_close(event_loop);

        self.input.begin_frame();
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.renderer.surface_resize((*size).into());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.input.handle_keyboard_input(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.input.handle_mouse_input(*state, *button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input
                    .handle_cursor_move(DVec2::new(position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.input.handle_mouse_wheel(*delta);
            }
            _ => {}
        }
    }
}

impl Engine {
    fn render(&mut self) {
        let result = self.renderer.render();
        let size = self.renderer.surface_size();

        if result.requires_surface_recovery() {
            self.renderer.surface_resize(size);
        }

        if result.is_fatal() {
            panic!("Wgpu validation error");
        }
    }

    fn should_close(&self, event_loop: &ActiveEventLoop) {
        if self.input.is_key_just_pressed(KeyCode::Escape) {
            event_loop.exit();
        }
    }
}
