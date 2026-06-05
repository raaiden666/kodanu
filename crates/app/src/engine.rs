use {graphics::Renderer, input::Input, math::DVec2, scene::Scene, time::Time, window::Window};

use winit::event::WindowEvent;

pub struct Engine {
    renderer: Renderer,
    scene: Scene,
    input: Input,
    time: Time,
}

impl Engine {
    pub async fn new(window: &Window) -> Self {
        Self {
            renderer: Renderer::new(window).await,
            scene: Scene::default(),
            input: Input::default(),
            time: Time::default(),
        }
    }
}

impl Engine {
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
    pub fn render(&mut self) {
        let result = self.renderer.render();
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

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}
