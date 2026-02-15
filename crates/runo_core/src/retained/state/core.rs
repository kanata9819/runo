use std::collections::HashMap;

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::event::UiEvent;
use crate::retained::node::{LabelNode, WidgetNode};
use crate::retained::state::RetainedState;

impl RetainedState {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            order: Vec::new(),
            active_button: None,
            active_checkbox: None,
            active_radio_button: None,
            active_slider: None,
            active_combo_box: None,
            active_text_box_scrollbar: None,
            focused_text_box: None,
            events: std::collections::VecDeque::new(),
            text_clipboard: String::new(),
            div_visible: HashMap::new(),
            div_enabled: HashMap::new(),
            div_background: HashMap::new(),
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

    pub(crate) fn set_label_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let Some(WidgetNode::Label(label)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };
        label.enabled = enabled;
    }

    pub(crate) fn div_visible(&self, id: impl AsRef<str>) -> bool {
        self.div_visible.get(id.as_ref()).copied().unwrap_or(true)
    }

    pub(crate) fn div_enabled(&self, id: impl AsRef<str>) -> bool {
        self.div_enabled.get(id.as_ref()).copied().unwrap_or(true)
    }

    pub(crate) fn div_background(&self, id: impl AsRef<str>) -> Option<Color> {
        self.div_background.get(id.as_ref()).copied()
    }

    pub(crate) fn set_div_visible(&mut self, id: impl Into<String>, visible: bool) {
        self.div_visible.insert(id.into(), visible);
    }

    pub(crate) fn set_div_enabled(&mut self, id: impl Into<String>, enabled: bool) {
        self.div_enabled.insert(id.into(), enabled);
    }

    pub(crate) fn set_div_background(&mut self, id: impl Into<String>, color: Color) {
        self.div_background.insert(id.into(), color);
    }

    pub(crate) fn clear_div_background(&mut self, id: impl AsRef<str>) {
        self.div_background.remove(id.as_ref());
    }

    pub(crate) fn pop_event(&mut self) -> Option<UiEvent> {
        self.events.pop_front()
    }

    pub(crate) fn drain_events(&mut self) -> Vec<UiEvent> {
        self.events.drain(..).collect()
    }

    pub(in crate::retained) fn push_event(&mut self, event: UiEvent) {
        self.events.push_back(event);
    }
}
