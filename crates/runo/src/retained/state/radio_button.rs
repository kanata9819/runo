use std::collections::HashMap;

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::RadioButtonResponse;
use crate::retained::node::{RadioButtonNode, WidgetNode};
use crate::retained::state::RetainedState;

pub(crate) struct UpsertRadioButtonArgs {
    pub(crate) id: String,
    pub(crate) group: String,
    pub(crate) rect: Rect,
    pub(crate) text: Option<String>,
    pub(crate) selected: Option<bool>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl RetainedState {
    pub(crate) fn upsert_radio_button(
        &mut self,
        args: UpsertRadioButtonArgs,
    ) -> RadioButtonResponse {
        let UpsertRadioButtonArgs {
            id,
            group,
            rect,
            text,
            selected,
            font_size,
            text_color,
            enabled,
        } = args;

        let default_selected = selected.unwrap_or(false);

        if !self.widgets.contains_key(&id) {
            if default_selected {
                Self::clear_radio_group_selection(&mut self.widgets, &group);
            }

            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::RadioButton(RadioButtonNode {
                    rect,
                    group,
                    text,
                    selected: default_selected,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                }),
            );

            return RadioButtonResponse {
                selected: default_selected,
                hovered: false,
                pressed: false,
                changed: false,
            };
        }

        if let Some(WidgetNode::RadioButton(radio_button)) = self.widgets.get_mut(&id) {
            radio_button.rect = rect;
            radio_button.group = group;
            radio_button.text = text;
            let _ = selected;
            radio_button.font_size = font_size;
            radio_button.text_color = text_color;
            radio_button.enabled = enabled;

            return RadioButtonResponse {
                selected: radio_button.selected,
                hovered: radio_button.hovered,
                pressed: radio_button.pressed,
                changed: radio_button.changed,
            };
        }

        if default_selected {
            Self::clear_radio_group_selection(&mut self.widgets, &group);
        }

        self.widgets.insert(
            id,
            WidgetNode::RadioButton(RadioButtonNode {
                rect,
                group,
                text,
                selected: default_selected,
                font_size,
                text_color,
                enabled,
                hovered: false,
                pressed: false,
                changed: false,
            }),
        );

        RadioButtonResponse {
            selected: default_selected,
            hovered: false,
            pressed: false,
            changed: false,
        }
    }

    pub(crate) fn radio_button_response(&self, id: impl AsRef<str>) -> RadioButtonResponse {
        let Some(WidgetNode::RadioButton(radio_button)) = self.widgets.get(id.as_ref()) else {
            return RadioButtonResponse::default();
        };

        RadioButtonResponse {
            selected: radio_button.selected,
            hovered: radio_button.hovered,
            pressed: radio_button.pressed,
            changed: radio_button.changed,
        }
    }

    pub(crate) fn set_radio_button_selected(&mut self, id: impl AsRef<str>, selected: bool) {
        let id_ref = id.as_ref();
        let group = self.widgets.get(id_ref).and_then(|node| match node {
            WidgetNode::RadioButton(radio_button) => Some(radio_button.group.clone()),
            _ => None,
        });

        let Some(group) = group else {
            return;
        };

        if selected {
            Self::clear_radio_group_selection(&mut self.widgets, &group);
        }

        let Some(WidgetNode::RadioButton(radio_button)) = self.widgets.get_mut(id_ref) else {
            return;
        };

        radio_button.changed = radio_button.selected != selected;
        radio_button.selected = selected;
    }

    pub(crate) fn set_radio_button_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::RadioButton(radio_button)) = self.widgets.get_mut(id_ref) else {
            return;
        };

        radio_button.enabled = enabled;

        if !enabled {
            radio_button.hovered = false;
            radio_button.pressed = false;
            radio_button.changed = false;

            if self.active_radio_button.as_deref() == Some(id_ref) {
                self.active_radio_button = None;
            }
        }
    }

    fn clear_radio_group_selection(widgets: &mut HashMap<String, WidgetNode>, group: &str) {
        for node in widgets.values_mut() {
            if let WidgetNode::RadioButton(radio_button) = node
                && radio_button.group == group
            {
                radio_button.selected = false;
                radio_button.changed = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use crate::retained::node::WidgetNode;
    use crate::retained::state::{RetainedState, UpsertRadioButtonArgs};

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 180.0, 36.0)
    }

    #[test]
    fn selecting_true_clears_other_nodes_in_same_group() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r2".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });

        state.set_radio_button_selected("r2", true);
        assert!(!state.radio_button_response("r1").selected);
        assert!(state.radio_button_response("r2").selected);
    }

    #[test]
    fn set_radio_button_enabled_false_clears_flags_and_active_id() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.active_radio_button = Some("r1".to_string());
        if let Some(WidgetNode::RadioButton(r)) = state.widgets.get_mut("r1") {
            r.hovered = true;
            r.pressed = true;
            r.changed = true;
        }

        state.set_radio_button_enabled("r1", false);
        if let Some(WidgetNode::RadioButton(r)) = state.widgets.get("r1") {
            assert!(!r.enabled);
            assert!(!r.hovered);
            assert!(!r.pressed);
            assert!(!r.changed);
        } else {
            panic!("radio button missing");
        }
        assert!(state.active_radio_button.is_none());
    }

    #[test]
    fn set_radio_button_selected_false_updates_changed_flag() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });

        state.set_radio_button_selected("r1", false);
        let response = state.radio_button_response("r1");
        assert!(!response.selected);
        assert!(response.changed);
    }

    #[test]
    fn upsert_radio_button_updates_existing_entry_fields() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: Some("a".to_string()),
            selected: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        let new_rect = Rect::new(5.0, 6.0, 55.0, 26.0);
        let response = state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g2".to_string(),
            rect: new_rect,
            text: Some("b".to_string()),
            selected: Some(true),
            font_size: 18.0,
            text_color: color,
            enabled: false,
        });
        assert!(!response.selected);
        if let Some(WidgetNode::RadioButton(rb)) = state.widgets.get("r1") {
            assert_eq!(rb.group, "g2");
            assert_eq!(rb.text.as_deref(), Some("b"));
            assert_eq!(rb.rect, new_rect);
            assert_eq!(rb.font_size, 18.0);
            assert!(!rb.enabled);
        } else {
            panic!("radio button missing");
        }
    }
}
