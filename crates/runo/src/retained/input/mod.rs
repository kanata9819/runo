mod pointer;
mod text_box;

use vello::peniko::FontData;

use crate::input::InputFrame;
use crate::retained::state::RetainedState;

impl RetainedState {
    pub(crate) fn begin_frame_input(&mut self, input: InputFrame, font: Option<&FontData>) {
        self.update_hover_flags(input.cursor_pos);
        self.handle_mouse_press(input.mouse_pressed);
        self.handle_text_box_scrollbar_input(
            input.mouse_pressed,
            input.mouse_down,
            input.mouse_released,
            input.cursor_pos,
        );
        self.update_button_states(input.mouse_pressed, input.mouse_down, input.mouse_released);
        self.update_checkbox_states(input.mouse_down, input.mouse_released);
        self.update_radio_button_states(input.mouse_down, input.mouse_released);
        self.update_slider_states(
            input.cursor_pos,
            input.mouse_pressed,
            input.mouse_down,
            input.mouse_released,
        );
        self.update_combo_box_states(input.mouse_down, input.mouse_released);
        self.update_text_box_focus();
        self.apply_text_box_scroll(&input);
        self.apply_text_input(&input, font);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_frame_input_runs_with_default_input_and_no_widgets() {
        let mut state = RetainedState::new();
        let input = InputFrame {
            cursor_pos: (0.0, 0.0),
            mouse_down: false,
            mouse_pressed: false,
            mouse_released: false,
            text_input: String::new(),
            backspace_pressed: false,
            delete_pressed: false,
            enter_pressed: false,
            arrow_left_pressed: false,
            arrow_right_pressed: false,
            arrow_up_pressed: false,
            arrow_down_pressed: false,
            copy_pressed: false,
            paste_pressed: false,
            scroll_x: 0.0,
            scroll_y: 0.0,
        };

        state.begin_frame_input(input, None);
        assert!(state.drain_events().is_empty());
    }
}
