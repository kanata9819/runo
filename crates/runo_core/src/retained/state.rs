use std::collections::{HashMap, VecDeque};

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ButtonResponse;
use crate::ComboBoxResponse;
use crate::event::UiEvent;
use crate::retained::node::{ButtonNode, ComboBoxNode, LabelNode, TextBoxNode, WidgetNode};
use crate::widget::text_box::TextBoxResponse;

pub(crate) struct RetainedState {
    pub(super) widgets: HashMap<String, WidgetNode>,
    pub(super) order: Vec<String>,
    pub(super) active_button: Option<String>,
    pub(super) active_combo_box: Option<String>,
    pub(super) focused_text_box: Option<String>,
    pub(super) events: VecDeque<UiEvent>,
}

impl RetainedState {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            order: Vec::new(),
            active_button: None,
            active_combo_box: None,
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
        enabled: bool,
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
                button.text = text;
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

    pub(crate) fn upsert_label(
        &mut self,
        id: String,
        rect: Rect,
        text: String,
        font_size: f32,
        text_color: Color,
        enabled: bool,
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
                    enabled,
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
                enabled,
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
        enabled: bool,
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
                    enabled,
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
                text_box.enabled = enabled;
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
                    enabled,
                    hovered: false,
                    focused: false,
                    changed: false,
                });
                TextBoxResponse::default()
            }
        }
    }

    pub(crate) fn upsert_combo_box(
        &mut self,
        id: String,
        rect: Rect,
        items: Vec<String>,
        selected_index: Option<usize>,
        font_size: f32,
        text_color: Color,
        bg_color: Color,
        border_color: Color,
        enabled: bool,
    ) -> ComboBoxResponse {
        let selected_index_override = selected_index;
        let initial_selected_index = if items.is_empty() {
            0
        } else {
            selected_index_override.unwrap_or(0).min(items.len() - 1)
        };

        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::ComboBox(ComboBoxNode {
                    rect,
                    items,
                    selected_index: initial_selected_index,
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    enabled,
                    hovered: false,
                    hovered_item: None,
                    pressed: false,
                    changed: false,
                    is_open: false,
                }),
            );
            return ComboBoxResponse::default();
        }

        let entry = self.widgets.get_mut(&id).expect("combo box entry missing");
        match entry {
            WidgetNode::ComboBox(combo_box) => {
                combo_box.rect = rect;
                combo_box.items = items;
                if combo_box.items.is_empty() {
                    combo_box.selected_index = 0;
                } else if let Some(next_index) = selected_index_override {
                    combo_box.selected_index = next_index.min(combo_box.items.len() - 1);
                } else if combo_box.selected_index >= combo_box.items.len() {
                    combo_box.selected_index = combo_box.items.len() - 1;
                }
                combo_box.font_size = font_size;
                combo_box.text_color = text_color;
                combo_box.bg_color = bg_color;
                combo_box.border_color = border_color;
                combo_box.enabled = enabled;
                ComboBoxResponse {
                    selected_index: combo_box.selected_index,
                    selected_text: combo_box
                        .items
                        .get(combo_box.selected_index)
                        .cloned()
                        .unwrap_or_default(),
                    hovered: combo_box.hovered,
                    pressed: combo_box.pressed,
                    changed: combo_box.changed,
                    is_open: combo_box.is_open,
                }
            }
            _ => {
                *entry = WidgetNode::ComboBox(ComboBoxNode {
                    rect,
                    items,
                    selected_index: initial_selected_index,
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    enabled,
                    hovered: false,
                    hovered_item: None,
                    pressed: false,
                    changed: false,
                    is_open: false,
                });
                ComboBoxResponse::default()
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
        }
    }

    pub(crate) fn combo_box_response(&self, id: impl AsRef<str>) -> ComboBoxResponse {
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get(id.as_ref()) else {
            return ComboBoxResponse::default();
        };
        ComboBoxResponse {
            selected_index: combo_box.selected_index,
            selected_text: combo_box
                .items
                .get(combo_box.selected_index)
                .cloned()
                .unwrap_or_default(),
            hovered: combo_box.hovered,
            pressed: combo_box.pressed,
            changed: combo_box.changed,
            is_open: combo_box.is_open,
        }
    }

    pub(crate) fn set_combo_box_selected_index(&mut self, id: impl AsRef<str>, index: usize) {
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };
        if combo_box.items.is_empty() {
            combo_box.selected_index = 0;
            combo_box.changed = false;
            return;
        }
        let next_index = index.min(combo_box.items.len() - 1);
        combo_box.changed = combo_box.selected_index != next_index;
        combo_box.selected_index = next_index;
    }

    pub(crate) fn set_combo_box_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get_mut(id_ref) else {
            return;
        };
        combo_box.enabled = enabled;
        if !enabled {
            combo_box.hovered = false;
            combo_box.hovered_item = None;
            combo_box.pressed = false;
            combo_box.changed = false;
            combo_box.is_open = false;
            if self.active_combo_box.as_deref() == Some(id_ref) {
                self.active_combo_box = None;
            }
        }
    }

    pub(crate) fn set_label_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let Some(WidgetNode::Label(label)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };
        label.enabled = enabled;
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
