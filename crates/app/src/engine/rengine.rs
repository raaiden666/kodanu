use crate::res::{FAILED_TO_CREATE_WINDOW, WGPU_VALIDATION_ERROR};

use graphics::render::Renderer;

use window::{config::NativeWindowConfig, native::NativeWindow};

use std::{panic, sync::Arc};

use pollster::block_on;

use anyhow::{Ok, Result};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct REngine {
    window: Option<NativeWindow>,
    renderer: Option<Renderer>,
    config: NativeWindowConfig,
}

impl REngine {
    fn new(config: NativeWindowConfig) -> Self {
        Self {
            window: None,
            renderer: None,
            config: config,
        }
    }

    pub fn run(config: NativeWindowConfig) -> Result<()> {
        let mut rengine = REngine::new(config);
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut rengine)?;
        Ok(())
    }

    fn recover_surface(renderer: &mut Renderer) {
        let size = renderer.surface_size();
        renderer.surface_resize(size);
    }

    fn render(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let result = renderer.render();

        if result.requires_surface_recovery() {
            Self::recover_surface(renderer);
        }

        if result.is_fatal() {
            panic!("{}", WGPU_VALIDATION_ERROR)
        }
    }
}

impl ApplicationHandler for REngine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let raw_window = event_loop
            .create_window(self.config.to_attributes())
            .expect(FAILED_TO_CREATE_WINDOW);

        let window = NativeWindow::new(Arc::new(raw_window));
        let renderer = block_on(Renderer::new(&window));

        window.request_redraw();

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.surface_resize(size);
                }
            }
            _ => {}
        }
    }
}
