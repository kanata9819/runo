use winit::application::ApplicationHandler;
use winit::event::{Ime, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowId;

use crate::app::{AppRunner, RunoApplication};

#[cfg(test)]
#[path = "../../tests/unit/app/events.rs"]
mod tests;

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

fn apply_keyboard_actions<A: RunoApplication + 'static>(
    runner: &mut AppRunner<A>,
    actions: KeyboardActions,
) {
    if let Some(text) = actions.push_text.as_deref() {
        runner.input.push_text_input(text);
    }

    if let Some(named) = actions.named_key {
        runner.input.on_named_key_pressed(named);
    }

    if let Some(shortcut) = actions.clipboard {
        match shortcut {
            ClipboardShortcut::Copy => runner.input.on_copy_pressed(),
            ClipboardShortcut::Paste => runner.input.on_paste_pressed(),
        }
    }

    if actions.request_redraw {
        runner.request_redraw();
    }
}

impl<A: RunoApplication + 'static> AppRunner<A> {
    fn update_input_and_request_redraw(
        &mut self,
        update: impl FnOnce(&mut crate::input::InputState),
    ) {
        update(&mut self.input);
        self.request_redraw();
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
                self.update_input_and_request_redraw(|input| {
                    input.set_cursor_pos(position.x, position.y);
                });
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.update_input_and_request_redraw(|input| {
                    input.on_mouse_input(state);
                });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (dx, dy) = scroll_delta_to_pixels(delta);
                self.update_input_and_request_redraw(|input| {
                    input.on_mouse_wheel(dx, dy);
                });
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let actions = keyboard_actions(
                    &event.logical_key,
                    event.text.as_deref(),
                    event.state.is_pressed(),
                    self.input.ctrl_pressed(),
                    self.input.ime_active(),
                );
                apply_keyboard_actions(self, actions);
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
                self.update_input_and_request_redraw(|input| {
                    input.push_text_input(&text);
                });
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
