use winit::application::ApplicationHandler;
use winit::event::{MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use crate::app::{AppRunner, Application};

impl<A: Application + 'static> ApplicationHandler for AppRunner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.init_window_and_gpu(event_loop);
            self.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
                self.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.set_cursor_pos(position.x, position.y);
                self.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.input.on_mouse_input(state);
                self.request_redraw();
            }
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }
}
