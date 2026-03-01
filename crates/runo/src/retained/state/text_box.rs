use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::node::{TextBoxNode, WidgetNode};
use crate::retained::state::RetainedState;
use crate::widget::text::estimate_text_width;
use crate::widget::text_box::{Overflow, TextBoxResponse};

pub(crate) struct UpsertTextBoxArgs {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) text: Option<String>,
    pub(crate) placeholder: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) enabled: bool,
    pub(crate) overflow_x: Overflow,
    pub(crate) overflow_y: Overflow,
}

impl RetainedState {
    pub(crate) fn upsert_text_box(&mut self, args: UpsertTextBoxArgs) -> TextBoxResponse {
        let UpsertTextBoxArgs {
            id,
            rect,
            text,
            placeholder,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled,
            overflow_x,
            overflow_y,
        } = args;
        let initial_text = text.clone().unwrap_or_default();
        let initial_text_advance = estimate_text_width(&initial_text, font_size) as f64;
        let initial_caret_index = initial_text.chars().count();
        let text_for_update = text;
        let placeholder_for_update = placeholder.clone();

        self.upsert_widget_node(
            id,
            || {
                WidgetNode::TextBox(TextBoxNode {
                    rect,
                    text: initial_text.clone(),
                    placeholder: placeholder.clone(),
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    enabled,
                    overflow_x,
                    overflow_y,
                    text_advance: initial_text_advance,
                    caret_index: initial_caret_index,
                    scroll_x: 0.0,
                    scroll_y: 0.0,
                    hovered: false,
                    focused: false,
                    changed: false,
                })
            },
            |entry| match entry {
                WidgetNode::TextBox(text_box) => {
                    text_box.rect = rect;

                    if let Some(next_text) = text_for_update {
                        text_box.text = next_text;
                        text_box.text_advance =
                            estimate_text_width(&text_box.text, text_box.font_size) as f64;
                        text_box.caret_index = text_box.text.chars().count();
                    }

                    text_box.placeholder = placeholder_for_update;

                    if (text_box.font_size - font_size).abs() > f32::EPSILON {
                        text_box.font_size = font_size;
                        text_box.text_advance =
                            estimate_text_width(&text_box.text, text_box.font_size) as f64;
                    }

                    text_box.text_color = text_color;
                    text_box.bg_color = bg_color;
                    text_box.border_color = border_color;
                    text_box.enabled = enabled;
                    text_box.overflow_x = overflow_x;
                    text_box.overflow_y = overflow_y;

                    Some(TextBoxResponse {
                        text: text_box.text.clone(),
                        hovered: text_box.hovered,
                        focused: text_box.focused,
                        changed: text_box.changed,
                    })
                }
                _ => None,
            },
            |node| match node {
                WidgetNode::TextBox(text_box) => TextBoxResponse {
                    text: text_box.text.clone(),
                    hovered: false,
                    focused: false,
                    changed: false,
                },
                _ => TextBoxResponse::default(),
            },
        )
    }

    pub(crate) fn text_box_response(&self, id: impl AsRef<str>) -> TextBoxResponse {
        let Some(WidgetNode::TextBox(text_box)) = self.widgets.get(id.as_ref()) else {
            return TextBoxResponse::default();
        };

        TextBoxResponse {
            text: text_box.text.clone(),
            hovered: text_box.hovered,
            focused: text_box.focused,
            changed: text_box.changed,
        }
    }

    pub(crate) fn set_text_box_text(&mut self, id: impl AsRef<str>, text: impl Into<String>) {
        let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        text_box.text = text.into();
        text_box.text_advance = estimate_text_width(&text_box.text, text_box.font_size) as f64;
        text_box.caret_index = text_box.text.chars().count();
        text_box.changed = true;
    }

    pub(crate) fn set_text_box_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(id_ref) else {
            return;
        };

        text_box.enabled = enabled;

        if !enabled {
            text_box.hovered = false;
            text_box.focused = false;
            text_box.changed = false;

            if self.focused_text_box.as_deref() == Some(id_ref) {
                self.focused_text_box = None;
            }

            if self.active_text_box_scrollbar.as_deref() == Some(id_ref) {
                self.active_text_box_scrollbar = None;
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/retained/state/text_box.rs"]
mod tests;
