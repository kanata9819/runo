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
        self.update_combo_box_states(input.mouse_down, input.mouse_released);
        self.update_text_box_focus();
        self.apply_text_box_scroll(&input);
        self.apply_text_input(&input, font);
    }
}
