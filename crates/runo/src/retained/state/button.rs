use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ButtonResponse;
use crate::retained::node::{ButtonNode, WidgetNode};
use crate::retained::state::RetainedState;

#[cfg(test)]
#[path = "../../../tests/unit/retained/state/button.rs"]
mod tests;

impl RetainedState {
    pub(crate) fn upsert_button(
        &mut self,
        id: String,
        rect: Rect,
        text: Option<String>,
        font_size: f32,
        text_color: Color,
        enabled: bool,
    ) -> ButtonResponse {
        let text_for_update = text.clone();
        self.upsert_widget_node(
            id,
            || {
                WidgetNode::Button(ButtonNode {
                    rect,
                    text: text.clone(),
                    text_overridden: false,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    clicked: false,
                })
            },
            |entry| match entry {
                WidgetNode::Button(button) => {
                    button.rect = rect;

                    if !button.text_overridden {
                        button.text = text_for_update;
                    }

                    button.font_size = font_size;
                    button.text_color = text_color;
                    button.enabled = enabled;

                    Some(ButtonResponse {
                        hovered: button.hovered,
                        pressed: button.pressed,
                        clicked: button.clicked,
                    })
                }
                _ => None,
            },
            |_node| ButtonResponse::default(),
        )
    }

    pub(crate) fn button_response(&self, id: impl AsRef<str>) -> ButtonResponse {
        let Some(WidgetNode::Button(button)) = self.widgets.get(id.as_ref()) else {
            return ButtonResponse::default();
        };

        ButtonResponse {
            hovered: button.hovered,
            pressed: button.pressed,
            clicked: button.clicked,
        }
    }

    pub(crate) fn set_button_text(&mut self, id: impl AsRef<str>, text: Option<String>) {
        let Some(WidgetNode::Button(button)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        button.text = text;
        button.text_overridden = true;
    }

    pub(crate) fn set_button_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::Button(button)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        button.enabled = enabled;

        if !enabled {
            button.hovered = false;
            button.pressed = false;
            button.clicked = false;

            if self.active_button.as_deref() == Some(id_ref) {
                self.active_button = None;
            }
        }
    }
}
