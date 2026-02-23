use winit::application::ApplicationHandler;
use winit::event::{Ime, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowId;

use crate::app::{AppRunner, RunoApplication};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClipboardShortcut {
    Copy,
    Paste,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct KeyboardActions {
    push_text: Option<String>,
    named_key: Option<NamedKey>,
    clipboard: Option<ClipboardShortcut>,
    request_redraw: bool,
}

fn scroll_delta_to_pixels(delta: MouseScrollDelta) -> (f64, f64) {
    match delta {
        MouseScrollDelta::LineDelta(dx, dy) => (dx as f64 * 20.0, dy as f64 * 20.0),
        MouseScrollDelta::PixelDelta(delta) => (delta.x, delta.y),
    }
}

fn clipboard_shortcut(logical_key: &Key) -> Option<ClipboardShortcut> {
    if let Key::Character(text) = logical_key {
        if text.eq_ignore_ascii_case("c") {
            return Some(ClipboardShortcut::Copy);
        }
        if text.eq_ignore_ascii_case("v") {
            return Some(ClipboardShortcut::Paste);
        }
    }
    None
}

fn keyboard_actions(
    logical_key: &Key,
    text: Option<&str>,
    state_is_pressed: bool,
    ctrl_pressed: bool,
    ime_active: bool,
) -> KeyboardActions {
    if !state_is_pressed {
        return KeyboardActions::default();
    }

    let push_text = if !ime_active && !ctrl_pressed {
        text.map(str::to_string)
    } else {
        None
    };

    let named_key = if let Key::Named(named) = logical_key {
        Some(*named)
    } else {
        None
    };

    let clipboard = if ctrl_pressed {
        clipboard_shortcut(logical_key)
    } else {
        None
    };

    KeyboardActions {
        push_text,
        named_key,
        clipboard,
        request_redraw: true,
    }
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
                let actions = keyboard_actions(
                    &event.logical_key,
                    event.text.as_deref(),
                    event.state.is_pressed(),
                    self.input.ctrl_pressed(),
                    self.input.ime_active(),
                );

                if let Some(text) = actions.push_text.as_deref() {
                    self.input.push_text_input(text);
                }
                if let Some(named) = actions.named_key {
                    self.input.on_named_key_pressed(named);
                }
                if let Some(shortcut) = actions.clipboard {
                    match shortcut {
                        ClipboardShortcut::Copy => self.input.on_copy_pressed(),
                        ClipboardShortcut::Paste => self.input.on_paste_pressed(),
                    }
                }
                if actions.request_redraw {
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

        let (dx, dy) = scroll_delta_to_pixels(MouseScrollDelta::PixelDelta(PhysicalPosition::new(
            3.0, -4.0,
        )));
        assert_eq!(dx, 3.0);
        assert_eq!(dy, -4.0);
    }

    #[test]
    fn clipboard_shortcut_detects_copy_and_paste_keys() {
        assert_eq!(
            clipboard_shortcut(&Key::Character("c".into())),
            Some(ClipboardShortcut::Copy)
        );
        assert_eq!(
            clipboard_shortcut(&Key::Character("C".into())),
            Some(ClipboardShortcut::Copy)
        );
        assert_eq!(
            clipboard_shortcut(&Key::Character("v".into())),
            Some(ClipboardShortcut::Paste)
        );
        assert_eq!(
            clipboard_shortcut(&Key::Character("V".into())),
            Some(ClipboardShortcut::Paste)
        );
        assert_eq!(clipboard_shortcut(&Key::Character("x".into())), None);
        assert_eq!(
            clipboard_shortcut(&Key::Named(winit::keyboard::NamedKey::Enter)),
            None
        );
    }

    #[test]
    fn keyboard_actions_for_character_input_without_modifiers() {
        let actions = keyboard_actions(&Key::Character("a".into()), Some("a"), true, false, false);
        assert_eq!(actions.push_text.as_deref(), Some("a"));
        assert_eq!(actions.named_key, None);
        assert_eq!(actions.clipboard, None);
        assert!(actions.request_redraw);
    }

    #[test]
    fn keyboard_actions_for_named_key_and_clipboard_shortcut() {
        let enter_actions =
            keyboard_actions(&Key::Named(NamedKey::Enter), None, true, false, false);
        assert_eq!(enter_actions.named_key, Some(NamedKey::Enter));
        assert_eq!(enter_actions.push_text, None);
        assert_eq!(enter_actions.clipboard, None);
        assert!(enter_actions.request_redraw);

        let copy_actions =
            keyboard_actions(&Key::Character("c".into()), Some("c"), true, true, false);
        assert_eq!(copy_actions.push_text, None);
        assert_eq!(copy_actions.clipboard, Some(ClipboardShortcut::Copy));
    }

    #[test]
    fn keyboard_actions_ignore_text_when_ime_active_or_key_released() {
        let ime_actions =
            keyboard_actions(&Key::Character("x".into()), Some("x"), true, false, true);
        assert_eq!(ime_actions.push_text, None);
        assert!(ime_actions.request_redraw);

        let released_actions =
            keyboard_actions(&Key::Character("x".into()), Some("x"), false, false, false);
        assert_eq!(released_actions, KeyboardActions::default());
    }
}
