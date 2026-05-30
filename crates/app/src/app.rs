use input::{
    Input, handle_cursor_move, handle_keyboard_input, handle_mouse_input, handle_mouse_wheel,
};

use graphics::Renderer;

use window::{Window, WindowConfig};

use std::{panic, sync::Arc};

use pollster::block_on;

use anyhow::{Ok, Result};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

pub struct App {
    window: Option<Window>,
    renderer: Option<Renderer>,
    config: WindowConfig,
    input: Input,
}

impl App {
    pub fn run(config: WindowConfig, input: Input) -> Result<()> {
        let event_loop = EventLoop::new()?;
        let mut app = App::new(config, input);

        event_loop.run_app(&mut app)?;
        Ok(())
    }

    fn new(config: WindowConfig, input: Input) -> Self {
        Self {
            window: None,
            renderer: None,
            config: config,
            input: input,
        }
    }

    fn render(&mut self) {
        const WGPU_VALIDATION_ERROR: &str = "Wgpu validation error";

        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let result = renderer.render();
        let size = renderer.surface_size();

        if result.requires_surface_recovery() {
            renderer.surface_resize(size);
        }

        if result.is_fatal() {
            panic!("{}", WGPU_VALIDATION_ERROR)
        }
    }

    fn frame(&mut self) {
        self.render();
        self.input.begin_frame();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        const FAILED_TO_CREATE_NATIVE_WINDOW: &str = "Failed to create native window";

        let raw_window = event_loop
            .create_window(self.config.to_attributes())
            .expect(FAILED_TO_CREATE_NATIVE_WINDOW);

        let window = Window::new(Arc::new(raw_window));
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
                self.frame();

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.surface_resize(size.into());
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                handle_keyboard_input(event, &mut self.input);
            }

            WindowEvent::MouseInput { state, button, .. } => {
                handle_mouse_input(state, button, &mut self.input);
            }
            WindowEvent::CursorMoved { position, .. } => {
                handle_cursor_move(position.into(), &mut self.input);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                handle_mouse_wheel(delta, &mut self.input);
            }
            _ => {}
        }
    }
}
