use vello::kurbo::Rect;

use crate::event::UiEvent;
use crate::input::InputFrame;
use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

impl RetainedState {
    pub(crate) fn begin_frame_input(&mut self, input: InputFrame) {
        self.update_hover_flags(input.cursor_pos);
        self.handle_mouse_press(input.mouse_pressed);
        self.update_button_states(input.mouse_down, input.mouse_released);
        self.update_text_box_focus();
        self.apply_text_input(&input);
    }

    fn update_hover_flags(&mut self, cursor_pos: (f64, f64)) {
        for node in self.widgets.values_mut() {
            match node {
                WidgetNode::Button(button) => {
                    button.clicked = false;
                    button.hovered = contains(button.rect, cursor_pos.0, cursor_pos.1);
                }
                WidgetNode::TextBox(text_box) => {
                    text_box.changed = false;
                    text_box.hovered = contains(text_box.rect, cursor_pos.0, cursor_pos.1);
                }
                WidgetNode::Label(_) => {}
            }
        }
    }

    fn handle_mouse_press(&mut self, mouse_pressed: bool) {
        if mouse_pressed {
            self.active_button = self.order.iter().rev().find_map(|id| {
                let WidgetNode::Button(button) = self.widgets.get(id)? else {
                    return None;
                };
                if button.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.focused_text_box = self.order.iter().rev().find_map(|id| {
                let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                    return None;
                };
                if text_box.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });
        }
    }

    fn update_button_states(&mut self, mouse_down: bool, mouse_released: bool) {
        let mut clicked_ids = Vec::new();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::Button(button) = node {
                button.pressed = mouse_down
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false);
                if mouse_released
                    && button.hovered
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                {
                    button.clicked = true;
                    clicked_ids.push(id.clone());
                }
            }
        }

        for id in clicked_ids {
            self.push_event(UiEvent::ButtonClicked { id });
        }

        if mouse_released {
            self.active_button = None;
        }
    }

    fn update_text_box_focus(&mut self) {
        for node in self.widgets.values_mut() {
            if let WidgetNode::TextBox(text_box) = node {
                text_box.focused = false;
            }
        }
    }

    fn apply_text_input(&mut self, input: &InputFrame) {
        let mut pending_event: Option<UiEvent> = None;
        if let Some(id) = self.focused_text_box.clone()
            && let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&id)
        {
            text_box.focused = true;

            if input.backspace_pressed {
                if text_box.text.pop().is_some() {
                    text_box.changed = true;
                }
            }

            if !input.text_input.is_empty() {
                for ch in input.text_input.chars() {
                    if !ch.is_control() {
                        text_box.text.push(ch);
                        text_box.changed = true;
                    }
                }
            }

            if text_box.changed {
                pending_event = Some(UiEvent::TextBoxChanged {
                    id,
                    text: text_box.text.clone(),
                });
            }
        }

        if let Some(event) = pending_event {
            self.push_event(event);
        }
    }
}

fn contains(rect: Rect, x: f64, y: f64) -> bool {
    x >= rect.x0 && x <= rect.x1 && y >= rect.y0 && y <= rect.y1
}
