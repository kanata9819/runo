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
