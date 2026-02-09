use std::collections::HashMap;

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ButtonResponse;
use crate::retained::node::{ButtonNode, LabelNode, WidgetNode};

pub(crate) struct RetainedState {
    pub(super) widgets: HashMap<String, WidgetNode>,
    pub(super) order: Vec<String>,
    pub(super) active_button: Option<String>,
}

impl RetainedState {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            order: Vec::new(),
            active_button: None,
        }
    }

    pub(crate) fn upsert_button(
        &mut self,
        id: String,
        rect: Rect,
        text: Option<String>,
        text_color: Color,
    ) -> ButtonResponse {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::Button(ButtonNode {
                    rect,
                    text,
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
                button.text_color = text_color;
                ButtonResponse {
                    hovered: button.hovered,
                    pressed: button.pressed,
                    clicked: button.clicked,
                }
            }
            WidgetNode::Label(_) => {
                *entry = WidgetNode::Button(ButtonNode {
                    rect,
                    text,
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
}
