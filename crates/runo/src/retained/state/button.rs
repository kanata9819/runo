use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ButtonResponse;
use crate::retained::node::{ButtonNode, WidgetNode};
use crate::retained::state::RetainedState;

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
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());

            self.widgets.insert(
                id.clone(),
                WidgetNode::Button(ButtonNode {
                    rect,
                    text,
                    text_overridden: false,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    clicked: false,
                }),
            );

            return ButtonResponse::default();
        }

        let entry = self.widgets.get_mut(&id).expect("button entry missing");
        match entry {
            WidgetNode::Button(button) => {
                button.rect = rect;

                if !button.text_overridden {
                    button.text = text;
                }

                button.font_size = font_size;
                button.text_color = text_color;
                button.enabled = enabled;

                ButtonResponse {
                    hovered: button.hovered,
                    pressed: button.pressed,
                    clicked: button.clicked,
                }
            }
            _ => {
                *entry = WidgetNode::Button(ButtonNode {
                    rect,
                    text,
                    text_overridden: false,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    clicked: false,
                });

                ButtonResponse::default()
            }
        }
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

#[cfg(test)]
mod tests {
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use crate::retained::node::WidgetNode;
    use crate::retained::state::RetainedState;

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 100.0, 40.0)
    }

    #[test]
    fn upsert_button_creates_and_updates_response() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);

        let created = state.upsert_button(
            "btn".to_string(),
            rect(),
            Some("hello".to_string()),
            16.0,
            color,
            true,
        );
        assert!(!created.hovered);
        assert!(!created.pressed);
        assert!(!created.clicked);

        if let Some(WidgetNode::Button(button)) = state.widgets.get("btn") {
            assert_eq!(button.text.as_deref(), Some("hello"));
            assert!(!button.text_overridden);
            assert!(button.enabled);
        } else {
            panic!("button node missing");
        }

        state.set_button_text("btn", Some("override".to_string()));
        let _ = state.upsert_button(
            "btn".to_string(),
            rect(),
            Some("ignored".to_string()),
            20.0,
            color,
            true,
        );
        if let Some(WidgetNode::Button(button)) = state.widgets.get("btn") {
            assert_eq!(button.text.as_deref(), Some("override"));
            assert!(button.text_overridden);
            assert_eq!(button.font_size, 20.0);
        } else {
            panic!("button node missing");
        }
    }

    #[test]
    fn set_button_enabled_false_clears_interaction_and_active_button() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_button(
            "btn".to_string(),
            rect(),
            Some("hello".to_string()),
            16.0,
            color,
            true,
        );

        if let Some(WidgetNode::Button(button)) = state.widgets.get_mut("btn") {
            button.hovered = true;
            button.pressed = true;
            button.clicked = true;
        }
        state.active_button = Some("btn".to_string());

        state.set_button_enabled("btn", false);

        if let Some(WidgetNode::Button(button)) = state.widgets.get("btn") {
            assert!(!button.enabled);
            assert!(!button.hovered);
            assert!(!button.pressed);
            assert!(!button.clicked);
        } else {
            panic!("button node missing");
        }
        assert!(state.active_button.is_none());
    }
}
