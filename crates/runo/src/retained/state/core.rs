use std::collections::HashMap;

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::event::UiEvent;
use crate::retained::node::{LabelNode, WidgetNode};
use crate::retained::state::RetainedState;
use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

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

    pub(crate) fn push_event(&mut self, event: UiEvent) {
        self.events.push_back(event);
    }

    pub(crate) fn take_button_clicked(&mut self, handle: &ButtonHandle) -> bool {
        let Some(index) = self.events.iter().position(
            |event| matches!(event, UiEvent::ButtonClicked { button } if button == handle),
        ) else {
            return false;
        };
        let _ = self.events.remove(index);
        true
    }

    pub(crate) fn take_text_box_changed(&mut self, handle: &TextBoxHandle) -> Option<String> {
        let index = self.events.iter().position(
            |event| matches!(event, UiEvent::TextBoxChanged { text_box, .. } if text_box == handle),
        )?;
        let event = self.events.remove(index)?;
        match event {
            UiEvent::TextBoxChanged { text, .. } => Some(text),
            _ => None,
        }
    }

    pub(crate) fn take_checkbox_changed(&mut self, handle: &CheckboxHandle) -> Option<bool> {
        let index = self.events.iter().position(
            |event| matches!(event, UiEvent::CheckboxChanged { checkbox, .. } if checkbox == handle),
        )?;
        let event = self.events.remove(index)?;
        match event {
            UiEvent::CheckboxChanged { checked, .. } => Some(checked),
            _ => None,
        }
    }

    pub(crate) fn take_slider_changed(&mut self, handle: &SliderHandle) -> Option<f64> {
        let index = self.events.iter().position(
            |event| matches!(event, UiEvent::SliderChanged { slider, .. } if slider == handle),
        )?;
        let event = self.events.remove(index)?;
        match event {
            UiEvent::SliderChanged { value, .. } => Some(value),
            _ => None,
        }
    }

    pub(crate) fn take_radio_button_changed(&mut self, handle: &RadioButtonHandle) -> Option<bool> {
        let index = self.events.iter().position(|event| {
            matches!(event, UiEvent::RadioButtonChanged { radio_button, .. } if radio_button == handle)
        })?;
        let event = self.events.remove(index)?;
        match event {
            UiEvent::RadioButtonChanged { selected, .. } => Some(selected),
            _ => None,
        }
    }

    pub(crate) fn take_combo_box_changed(
        &mut self,
        handle: &ComboBoxHandle,
    ) -> Option<(usize, String)> {
        let index = self.events.iter().position(
            |event| matches!(event, UiEvent::ComboBoxChanged { combo_box, .. } if combo_box == handle),
        )?;
        let event = self.events.remove(index)?;
        match event {
            UiEvent::ComboBoxChanged {
                selected_index,
                selected_text,
                ..
            } => Some((selected_index, selected_text)),
            _ => None,
        }
    }
}
