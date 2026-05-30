use crate::res::{FAILED_TO_CREATE_NATIVE_WINDOW, WGPU_VALIDATION_ERROR};

use graphics::render::Renderer;

use input::winit::{
    InputState, handle_cursor_move, handle_keyboard_input, handle_mouse_input, handle_mouse_wheel,
};

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
    input: InputState,
}

impl REngine {
    pub fn run(config: NativeWindowConfig, input: InputState) -> Result<()> {
        let event_loop = EventLoop::new()?;
        let mut rengine = REngine::new(config, input);

        event_loop.run_app(&mut rengine)?;
        Ok(())
    }

    fn new(config: NativeWindowConfig, input: InputState) -> Self {
        Self {
            window: None,
            renderer: None,
            config: config,
            input: input,
        }
    }

    fn render(&mut self) {
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
}

impl ApplicationHandler for REngine {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let raw_window = event_loop
            .create_window(self.config.to_attributes())
            .expect(FAILED_TO_CREATE_NATIVE_WINDOW);

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
