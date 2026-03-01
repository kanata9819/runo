use winit::event::ElementState;
use winit::keyboard::NamedKey;

#[cfg(test)]
#[path = "../tests/unit/input.rs"]
mod tests;

#[derive(Clone)]
pub(crate) struct InputFrame {
    pub(crate) cursor_pos: (f64, f64),
    pub(crate) mouse_down: bool,
    pub(crate) mouse_pressed: bool,
    pub(crate) mouse_released: bool,
    pub(crate) text_input: String,
    pub(crate) backspace_pressed: bool,
    pub(crate) delete_pressed: bool,
    pub(crate) enter_pressed: bool,
    pub(crate) arrow_left_pressed: bool,
    pub(crate) arrow_right_pressed: bool,
    pub(crate) arrow_up_pressed: bool,
    pub(crate) arrow_down_pressed: bool,
    pub(crate) copy_pressed: bool,
    pub(crate) paste_pressed: bool,
    pub(crate) scroll_x: f64,
    pub(crate) scroll_y: f64,
}

#[derive(Default)]
pub(crate) struct InputState {
    cursor_pos: (f64, f64),
    mouse_down: bool,
    mouse_pressed: bool,
    mouse_released: bool,
    text_input: String,
    backspace_pressed: bool,
    delete_pressed: bool,
    enter_pressed: bool,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
    arrow_up_pressed: bool,
    arrow_down_pressed: bool,
    copy_pressed: bool,
    paste_pressed: bool,
    ctrl_pressed: bool,
    scroll_x: f64,
    scroll_y: f64,
    ime_active: bool,
}

impl InputState {
    pub(crate) fn snapshot(&self) -> InputFrame {
        InputFrame {
            cursor_pos: self.cursor_pos,
            mouse_down: self.mouse_down,
            mouse_pressed: self.mouse_pressed,
            mouse_released: self.mouse_released,
            text_input: self.text_input.clone(),
            backspace_pressed: self.backspace_pressed,
            delete_pressed: self.delete_pressed,
            enter_pressed: self.enter_pressed,
            arrow_left_pressed: self.arrow_left_pressed,
            arrow_right_pressed: self.arrow_right_pressed,
            arrow_up_pressed: self.arrow_up_pressed,
            arrow_down_pressed: self.arrow_down_pressed,
            copy_pressed: self.copy_pressed,
            paste_pressed: self.paste_pressed,
            scroll_x: self.scroll_x,
            scroll_y: self.scroll_y,
        }
    }

    pub(crate) fn end_frame(&mut self) {
        self.mouse_pressed = false;
        self.mouse_released = false;
        self.text_input.clear();
        self.backspace_pressed = false;
        self.delete_pressed = false;
        self.enter_pressed = false;
        self.arrow_left_pressed = false;
        self.arrow_right_pressed = false;
        self.arrow_up_pressed = false;
        self.arrow_down_pressed = false;
        self.copy_pressed = false;
        self.paste_pressed = false;
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
    }

    pub(crate) fn on_mouse_input(&mut self, state: ElementState) {
        let next = state == ElementState::Pressed;
        if next && !self.mouse_down {
            self.mouse_pressed = true;
        }

        if !next && self.mouse_down {
            self.mouse_released = true;
        }

        self.mouse_down = next;
    }

    pub(crate) fn set_cursor_pos(&mut self, x: f64, y: f64) {
        self.cursor_pos = (x, y);
    }

    pub(crate) fn push_text_input(&mut self, text: &str) {
        self.text_input.push_str(text);
    }

    pub(crate) fn on_named_key_pressed(&mut self, key: NamedKey) {
        match key {
            NamedKey::Backspace => self.backspace_pressed = true,
            NamedKey::Delete => self.delete_pressed = true,
            NamedKey::Enter => self.enter_pressed = true,
            NamedKey::ArrowLeft => self.arrow_left_pressed = true,
            NamedKey::ArrowRight => self.arrow_right_pressed = true,
            NamedKey::ArrowUp => self.arrow_up_pressed = true,
            NamedKey::ArrowDown => self.arrow_down_pressed = true,
            _ => {}
        }
    }

    pub(crate) fn on_copy_pressed(&mut self) {
        self.copy_pressed = true;
    }

    pub(crate) fn on_paste_pressed(&mut self) {
        self.paste_pressed = true;
    }

    pub(crate) fn on_mouse_wheel(&mut self, dx: f64, dy: f64) {
        self.scroll_x += dx;
        self.scroll_y += dy;
    }

    pub(crate) fn set_ctrl_pressed(&mut self, ctrl: bool) {
        self.ctrl_pressed = ctrl;
    }

    pub(crate) fn ctrl_pressed(&self) -> bool {
        self.ctrl_pressed
    }

    pub(crate) fn set_ime_active(&mut self, active: bool) {
        self.ime_active = active;
    }

    pub(crate) fn ime_active(&self) -> bool {
        self.ime_active
    }
}
