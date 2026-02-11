use std::collections::{HashMap, VecDeque};

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ButtonResponse;
use crate::event::UiEvent;
use crate::retained::node::{ButtonNode, LabelNode, TextBoxNode, WidgetNode};
use crate::widget::text_box::TextBoxResponse;

pub(crate) struct RetainedState {
    pub(super) widgets: HashMap<String, WidgetNode>,
    pub(super) order: Vec<String>,
    pub(super) active_button: Option<String>,
    pub(super) focused_text_box: Option<String>,
    pub(super) events: VecDeque<UiEvent>,
}

impl RetainedState {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            order: Vec::new(),
            active_button: None,
            focused_text_box: None,
            events: VecDeque::new(),
        }
    }

    pub(crate) fn upsert_button(
        &mut self,
        id: String,
        rect: Rect,
        text: Option<String>,
        font_size: f32,
        text_color: Color,
    ) -> ButtonResponse {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::Button(ButtonNode {
                    rect,
                    text,
                    font_size,
                    text_color,
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
                button.text = text;
                button.font_size = font_size;
                button.text_color = text_color;
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
                    font_size,
                    text_color,
                    hovered: false,
                    pressed: false,
                    clicked: false,
                });
                ButtonResponse::default()
            }
        }
    }

    pub(crate) fn upsert_label(
        &mut self,
        id: String,
        rect: Rect,
        text: String,
        font_size: f32,
        text_color: Color,
    ) {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::Label(LabelNode {
                    rect,
                    text,
                    font_size,
                    text_color,
                }),
            );
            return;
        }

        self.widgets.insert(
            id,
            WidgetNode::Label(LabelNode {
                rect,
                text,
                font_size,
                text_color,
            }),
        );
    }

    pub(crate) fn upsert_text_box(
        &mut self,
        id: String,
        rect: Rect,
        text: Option<String>,
        placeholder: Option<String>,
        font_size: f32,
        text_color: Color,
        bg_color: Color,
        border_color: Color,
    ) -> TextBoxResponse {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::TextBox(TextBoxNode {
                    rect,
                    text: text.unwrap_or_default(),
                    placeholder,
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    hovered: false,
                    focused: false,
                    changed: false,
                }),
            );
            return TextBoxResponse::default();
        }

        let entry = self.widgets.get_mut(&id).expect("text box entry missing");
        match entry {
            WidgetNode::TextBox(text_box) => {
                text_box.rect = rect;
                if let Some(next_text) = text {
                    text_box.text = next_text;
                }
                text_box.placeholder = placeholder;
                text_box.font_size = font_size;
                text_box.text_color = text_color;
                text_box.bg_color = bg_color;
                text_box.border_color = border_color;
                TextBoxResponse {
                    text: text_box.text.clone(),
                    hovered: text_box.hovered,
                    focused: text_box.focused,
                    changed: text_box.changed,
                }
            }
            _ => {
                *entry = WidgetNode::TextBox(TextBoxNode {
                    rect,
                    text: text.unwrap_or_default(),
                    placeholder,
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    hovered: false,
                    focused: false,
                    changed: false,
                });
                TextBoxResponse::default()
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
        text_box.changed = true;
    }

    pub(crate) fn pop_event(&mut self) -> Option<UiEvent> {
        self.events.pop_front()
    }

    pub(crate) fn drain_events(&mut self) -> Vec<UiEvent> {
        self.events.drain(..).collect()
    }

    pub(super) fn push_event(&mut self, event: UiEvent) {
        self.events.push_back(event);
    }
}
