use winit::event::ElementState;

#[derive(Clone, Copy)]
pub(crate) struct InputFrame {
    pub(crate) cursor_pos: (f64, f64),
    pub(crate) mouse_down: bool,
    pub(crate) mouse_pressed: bool,
    pub(crate) mouse_released: bool,
}

#[derive(Default)]
pub(crate) struct InputState {
    cursor_pos: (f64, f64),
    mouse_down: bool,
    mouse_pressed: bool,
    mouse_released: bool,
}

impl InputState {
    pub(crate) fn frame(&self) -> InputFrame {
        InputFrame {
            cursor_pos: self.cursor_pos,
            mouse_down: self.mouse_down,
            mouse_pressed: self.mouse_pressed,
            mouse_released: self.mouse_released,
        }
    }

    pub(crate) fn end_frame(&mut self) {
        self.mouse_pressed = false;
        self.mouse_released = false;
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
}
