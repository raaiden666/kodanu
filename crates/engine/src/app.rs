use std::sync::Arc;

use window::{NativeWindow, NativeWindowConfig};

use anyhow::{Ok, Result};

use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    event_loop::EventLoop, window::WindowId,
};

pub struct App {
    window: Option<NativeWindow>,
    config: NativeWindowConfig,
}

pub fn create_event_loop() -> Result<EventLoop<()>> {
    Ok(EventLoop::new()?)
}

impl App {
    pub fn new(config: NativeWindowConfig) -> Self {
        Self {
            window: None,
            config,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_config = self.config.to_attributes();

        let raw_window = event_loop.create_window(window_config).unwrap();

        let native_window = NativeWindow::new(Arc::new(raw_window));

        self.window = Some(native_window);
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
            _ => {}
        }
    }
}
