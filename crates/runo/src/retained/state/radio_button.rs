use std::collections::HashMap;

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::RadioButtonResponse;
use crate::retained::node::{RadioButtonNode, WidgetNode};
use crate::retained::state::RetainedState;

#[cfg(test)]
#[path = "../../../tests/unit/retained/state/radio_button.rs"]
mod tests;

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

        if selected == Some(true) {
            Self::clear_radio_group_selection(&mut self.widgets, &group);
        }

        let group_for_update = group.clone();
        let text_for_update = text.clone();

        self.upsert_widget_node(
            id,
            || {
                WidgetNode::RadioButton(RadioButtonNode {
                    rect,
                    group: group.clone(),
                    text: text.clone(),
                    selected: default_selected,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                })
            },
            |entry| match entry {
                WidgetNode::RadioButton(radio_button) => {
                    radio_button.rect = rect;
                    radio_button.group = group_for_update;
                    radio_button.text = text_for_update;

                    if let Some(selected) = selected {
                        radio_button.selected = selected;
                    }

                    radio_button.font_size = font_size;
                    radio_button.text_color = text_color;
                    radio_button.enabled = enabled;

                    Some(RadioButtonResponse {
                        selected: radio_button.selected,
                        hovered: radio_button.hovered,
                        pressed: radio_button.pressed,
                        changed: radio_button.changed,
                    })
                }
                _ => None,
            },
            |_node| RadioButtonResponse {
                selected: default_selected,
                hovered: false,
                pressed: false,
                changed: false,
            },
        )
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
