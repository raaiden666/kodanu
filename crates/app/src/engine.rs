use {
    graphics::{RenderItem, Renderer},
    input::Input,
    math::Mat4,
    time::Time,
    window::Window,
};

pub(crate) struct Engine {
    renderer: Renderer,
    input: Input,
    time: Time,
}

impl Engine {
    pub fn new(window: &Window) -> Self {
        Self {
            renderer: Renderer::new(window),
            input: Input::default(),
            time: Time::default(),
        }
    }
}

impl Engine {
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

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
