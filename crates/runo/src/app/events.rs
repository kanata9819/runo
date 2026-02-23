use winit::application::ApplicationHandler;
use winit::event::{Ime, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::Key;
use winit::window::WindowId;

use crate::app::{AppRunner, RunoApplication};

fn scroll_delta_to_pixels(delta: MouseScrollDelta) -> (f64, f64) {
    match delta {
        MouseScrollDelta::LineDelta(dx, dy) => (dx as f64 * 20.0, dy as f64 * 20.0),
        MouseScrollDelta::PixelDelta(delta) => (delta.x, delta.y),
    }
}

fn clipboard_shortcut(logical_key: &Key) -> Option<bool> {
    if let Key::Character(text) = logical_key {
        if text.eq_ignore_ascii_case("c") {
            return Some(true);
        }
        if text.eq_ignore_ascii_case("v") {
            return Some(false);
        }
    }
    None
}

impl<A: RunoApplication + 'static> ApplicationHandler for AppRunner<A> {
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
                let (dx, dy) = scroll_delta_to_pixels(delta);
                self.input.on_mouse_wheel(dx, dy);
                self.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    let ctrl = self.input.ctrl_pressed();
                    if !self.input.ime_active()
                        && !ctrl
                        && let Some(text) = event.text.as_ref()
                    {
                        self.input.push_text_input(text);
                    }
                    if let Key::Named(named) = &event.logical_key {
                        self.input.on_named_key_pressed(*named);
                    }
                    if ctrl && let Some(copy) = clipboard_shortcut(&event.logical_key) {
                        if copy {
                            self.input.on_copy_pressed();
                        } else {
                            self.input.on_paste_pressed();
                        }
                    }
                    self.request_redraw();
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.input.set_ctrl_pressed(modifiers.state().control_key());
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
            WindowEvent::RedrawRequested => {
                if self.render() {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::dpi::PhysicalPosition;

    #[test]
    fn scroll_delta_to_pixels_handles_line_and_pixel_modes() {
        let (dx, dy) = scroll_delta_to_pixels(MouseScrollDelta::LineDelta(1.5, -2.0));
        assert_eq!(dx, 30.0);
        assert_eq!(dy, -40.0);

        let (dx, dy) =
            scroll_delta_to_pixels(MouseScrollDelta::PixelDelta(PhysicalPosition::new(3.0, -4.0)));
        assert_eq!(dx, 3.0);
        assert_eq!(dy, -4.0);
    }

    #[test]
    fn clipboard_shortcut_detects_copy_and_paste_keys() {
        assert_eq!(clipboard_shortcut(&Key::Character("c".into())), Some(true));
        assert_eq!(clipboard_shortcut(&Key::Character("C".into())), Some(true));
        assert_eq!(clipboard_shortcut(&Key::Character("v".into())), Some(false));
        assert_eq!(clipboard_shortcut(&Key::Character("V".into())), Some(false));
        assert_eq!(clipboard_shortcut(&Key::Character("x".into())), None);
        assert_eq!(clipboard_shortcut(&Key::Named(winit::keyboard::NamedKey::Enter)), None);
    }
}
