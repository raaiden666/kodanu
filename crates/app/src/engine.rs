use {
    graphics::{RenderItem, Renderer},
    input::Input,
    math::{DVec2, Mat4, UVec2},
    time::Time,
    window::Window,
};

use winit::event::WindowEvent;

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
    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                self.renderer
                    .surface_resize(UVec2::new(size.height, size.width));
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

    pub fn render(&mut self, view_projection: Mat4, items: &[RenderItem]) {
        let result = self.renderer.render(view_projection, items);
        let size = self.renderer.surface_size();

        if result.requires_surface_recovery() {
            self.renderer.surface_resize(size);
        }

        if result.is_fatal() {
            panic!("Wgpu validation error");
        }
    }

    pub fn time_update(&mut self) {
        self.time.update();
    }

    pub fn begin_frame(&mut self) {
        self.input.begin_frame();
    }
}

impl Engine {
    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn input(&self) -> &Input {
        &self.input
    }
}
