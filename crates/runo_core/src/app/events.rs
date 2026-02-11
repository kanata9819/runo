use winit::application::ApplicationHandler;
use winit::event::{Ime, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
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
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(dx, dy) => (dx as f64 * 20.0, dy as f64 * 20.0),
                    MouseScrollDelta::PixelDelta(delta) => (delta.x, delta.y),
                };
                self.input.on_mouse_wheel(dx, dy);
                self.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    if !self.input.ime_active()
                        && let Some(text) = event.text.as_ref()
                    {
                        self.input.push_text_input(text);
                    }
                    if matches!(event.logical_key, Key::Named(NamedKey::Backspace)) {
                        self.input.on_backspace_pressed();
                    }
                    self.request_redraw();
                }
            }
            WindowEvent::Ime(Ime::Enabled) => {
                self.input.set_ime_active(true);
            }
            WindowEvent::Ime(Ime::Disabled) => {
                self.input.set_ime_active(false);
            }
            WindowEvent::Ime(Ime::Commit(text)) => {
                self.input.push_text_input(&text);
                self.request_redraw();
            }
            WindowEvent::RedrawRequested => self.render(),
            _ => {}
        }
    }
}
