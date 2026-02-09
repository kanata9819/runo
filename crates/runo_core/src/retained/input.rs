use vello::kurbo::Rect;

use crate::input::InputFrame;
use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

impl RetainedState {
    pub(crate) fn begin_frame_input(&mut self, input: InputFrame) {
        for node in self.widgets.values_mut() {
            if let WidgetNode::Button(button) = node {
                button.clicked = false;
                button.hovered = contains(button.rect, input.cursor_pos.0, input.cursor_pos.1);
            }
        }

        if input.mouse_pressed {
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
        }

        for (id, node) in &mut self.widgets {
            if let WidgetNode::Button(button) = node {
                button.pressed = input.mouse_down
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false);
                if input.mouse_released
                    && button.hovered
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                {
                    button.clicked = true;
                }
            }
        }

        if input.mouse_released {
            self.active_button = None;
        }
    }
}

fn contains(rect: Rect, x: f64, y: f64) -> bool {
    x >= rect.x0 && x <= rect.x1 && y >= rect.y0 && y <= rect.y1
}
