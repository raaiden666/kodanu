use std::{panic, sync::Arc};

use pollster::block_on;

use window::{config::NativeWindowConfig, native::NativeWindow};

use anyhow::{Ok, Result};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use graphics::{rendering::RenderResult, rendering::Renderer};

use crate::app_errors::{FAILED_TO_CREATE_WINDOW, WGPU_VALIDATION_ERROR};

pub struct App {
    window: Option<NativeWindow>,
    renderer: Option<Renderer>,
    config: NativeWindowConfig,
}

pub fn create_event_loop() -> Result<EventLoop<()>> {
    Ok(EventLoop::new()?)
}

impl App {
    pub fn new(config: NativeWindowConfig) -> Self {
        Self {
            window: None,
            renderer: None,
            config: config,
        }
    }

    pub fn render(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        match renderer.render() {
            RenderResult::Success => {}
            RenderResult::Suboptimal => {
                let size = renderer.surface_size();
                renderer.surface_resize(size);
            }
            RenderResult::Timeout => {}
            RenderResult::Occluded => {}
            RenderResult::Outdated => {
                let size = renderer.surface_size();
                renderer.surface_resize(size);
            }
            RenderResult::Lost => {
                let size = renderer.surface_size();
                renderer.surface_resize(size);
            }
            RenderResult::Validation => {
                panic!("{}", WGPU_VALIDATION_ERROR)
            }
        }
    }
}

impl ApplicationHandler for App {
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
