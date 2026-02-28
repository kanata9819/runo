use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::CheckboxResponse;
use crate::retained::node::{CheckboxNode, WidgetNode};
use crate::retained::state::RetainedState;

pub(crate) struct UpsertCheckboxArgs {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) text: Option<String>,
    pub(crate) checked: Option<bool>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl RetainedState {
    pub(crate) fn upsert_checkbox(&mut self, args: UpsertCheckboxArgs) -> CheckboxResponse {
        let UpsertCheckboxArgs {
            id,
            rect,
            text,
            checked,
            font_size,
            text_color,
            enabled,
        } = args;

        let default_checked = checked.unwrap_or(false);

        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());

            self.widgets.insert(
                id.clone(),
                WidgetNode::Checkbox(CheckboxNode {
                    rect,
                    text,
                    checked: default_checked,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                }),
            );

            return CheckboxResponse {
                checked: default_checked,
                hovered: false,
                pressed: false,
                changed: false,
            };
        }

        let entry = self.widgets.get_mut(&id).expect("checkbox entry missing");
        match entry {
            WidgetNode::Checkbox(checkbox) => {
                checkbox.rect = rect;
                checkbox.text = text;
                let _ = checked;
                checkbox.font_size = font_size;
                checkbox.text_color = text_color;
                checkbox.enabled = enabled;

                CheckboxResponse {
                    checked: checkbox.checked,
                    hovered: checkbox.hovered,
                    pressed: checkbox.pressed,
                    changed: checkbox.changed,
                }
            }
            _ => {
                *entry = WidgetNode::Checkbox(CheckboxNode {
                    rect,
                    text,
                    checked: default_checked,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                });

                CheckboxResponse {
                    checked: default_checked,
                    hovered: false,
                    pressed: false,
                    changed: false,
                }
            }
        }
    }

    pub(crate) fn checkbox_response(&self, id: impl AsRef<str>) -> CheckboxResponse {
        let Some(WidgetNode::Checkbox(checkbox)) = self.widgets.get(id.as_ref()) else {
            return CheckboxResponse::default();
        };

        CheckboxResponse {
            checked: checkbox.checked,
            hovered: checkbox.hovered,
            pressed: checkbox.pressed,
            changed: checkbox.changed,
        }
    }

    pub(crate) fn set_checkbox_checked(&mut self, id: impl AsRef<str>, checked: bool) {
        let Some(WidgetNode::Checkbox(checkbox)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        checkbox.changed = checkbox.checked != checked;
        checkbox.checked = checked;
    }

    pub(crate) fn set_checkbox_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::Checkbox(checkbox)) = self.widgets.get_mut(id_ref) else {
            return;
        };

        checkbox.enabled = enabled;

        if !enabled {
            checkbox.hovered = false;
            checkbox.pressed = false;
            checkbox.changed = false;

            if self.active_checkbox.as_deref() == Some(id_ref) {
                self.active_checkbox = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use crate::retained::node::WidgetNode;
    use crate::retained::state::{RetainedState, UpsertCheckboxArgs};

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 120.0, 36.0)
    }

    #[test]
    fn upsert_checkbox_keeps_existing_checked_when_checked_arg_is_none() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_checkbox(UpsertCheckboxArgs {
            id: "cb".to_string(),
            rect: rect(),
            text: Some("cb".to_string()),
            checked: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_checkbox(UpsertCheckboxArgs {
            id: "cb".to_string(),
            rect: rect(),
            text: Some("cb2".to_string()),
            checked: None,
            font_size: 18.0,
            text_color: color,
            enabled: true,
        });

        assert!(state.checkbox_response("cb").checked);
    }

    #[test]
    fn set_checkbox_enabled_false_clears_flags_and_active_id() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_checkbox(UpsertCheckboxArgs {
            id: "cb".to_string(),
            rect: rect(),
            text: Some("cb".to_string()),
            checked: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.active_checkbox = Some("cb".to_string());
        if let Some(WidgetNode::Checkbox(cb)) = state.widgets.get_mut("cb") {
            cb.hovered = true;
            cb.pressed = true;
            cb.changed = true;
        }

        state.set_checkbox_enabled("cb", false);
        if let Some(WidgetNode::Checkbox(cb)) = state.widgets.get("cb") {
            assert!(!cb.enabled);
            assert!(!cb.hovered);
            assert!(!cb.pressed);
            assert!(!cb.changed);
        } else {
            panic!("checkbox missing");
        }
        assert!(state.active_checkbox.is_none());
    }
}
