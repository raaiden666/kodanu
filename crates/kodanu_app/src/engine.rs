use {
    kodanu_editor::Scene,
    kodanu_graphics::{RenderQueue, Renderer, RendererConfig},
    kodanu_input::{ActionMap, Input},
    kodanu_math::Mat4,
    kodanu_time::Time,
    kodanu_window::Window,
};

pub(crate) struct Engine {
    renderer: Renderer,
    render_queue: RenderQueue,
    input: Input,
    action_map: ActionMap,
    time: Time,
}

impl Engine {
    pub fn new(window: &Window, config: &RendererConfig) -> Self {
        Self {
            renderer: Renderer::new(window, config),
            render_queue: RenderQueue::with_capacity(10_000),
            input: Input::default(),
            action_map: ActionMap::default(),
            time: Time::default(),
        }
    }
}

impl Engine {
    pub fn render(&mut self, view_projection: Mat4, scene: &Scene) {
        self.render_queue.collect_render_items(scene);

        let result = self
            .renderer
            .render(view_projection, self.render_queue.items());

        if result.requires_surface_recovery() {
            let size = self.renderer.surface_size();
            self.renderer.surface_resize(size);
        }

        if result.is_fatal() {
            panic!("Wgpu validation error");
        }
    }
}

impl Engine {
    #[inline]
    pub fn time_update(&mut self) {
        self.time.update();
    }

    #[inline]
    pub fn begin_frame(&mut self) {
        self.input.begin_frame();
    }

    #[inline]
    pub fn time(&self) -> &Time {
        &self.time
    }

    #[inline]
    pub fn input(&self) -> &Input {
        &self.input
    }

    #[inline]
    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    #[inline]
    pub fn action_map(&self) -> &ActionMap {
        &self.action_map
    }

    #[inline]
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
