use std::collections::{HashMap, HashSet};

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::event::UiEvent;
use crate::retained::node::{DivNode, LabelNode, WidgetNode};
use crate::retained::state::{RetainedState, clear_slot_if_absent};
use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

fn retain_seen_map<T>(map: &mut HashMap<String, T>, seen: &HashSet<String>) {
    map.retain(|id, _| seen.contains(id));
}

fn retain_seen_order(order: &mut Vec<String>, seen: &HashSet<String>) {
    order.retain(|id| seen.contains(id));
}

impl RetainedState {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            order: Vec::new(),
            seen_widget_ids: HashSet::new(),
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

    pub(crate) fn begin_build_pass(&mut self) {
        self.seen_widget_ids.clear();
    }

    pub(crate) fn prune_unseen_widgets(&mut self) {
        let seen = self.seen_widget_ids.clone();
        retain_seen_map(&mut self.widgets, &seen);
        retain_seen_order(&mut self.order, &seen);
        retain_seen_map(&mut self.div_visible, &seen);
        retain_seen_map(&mut self.div_enabled, &seen);
        retain_seen_map(&mut self.div_background, &seen);
        self.prune_active_widget_ids();
        self.events
            .retain(|event| seen.contains(event_widget_id(event)));
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
        self.upsert_widget_node(
            id,
            || {
                WidgetNode::Label(LabelNode {
                    rect,
                    text: text.clone(),
                    font_size,
                    text_color,
                    enabled,
                })
            },
            |_entry| None,
            |_node| (),
        );
    }

    pub(crate) fn upsert_div(
        &mut self,
        id: String,
        rect: Rect,
        radius: f64,
        default_bg_color: Option<Color>,
        border_color: Option<Color>,
        border_width: f64,
    ) {
        let visible = self.div_visible(&id);
        let bg_color = self.div_background(&id).or(default_bg_color);
        self.upsert_widget_node(
            id,
            || {
                WidgetNode::Div(DivNode {
                    rect,
                    radius,
                    visible,
                    bg_color,
                    default_bg_color,
                    border_color,
                    border_width,
                })
            },
            |entry| {
                let WidgetNode::Div(div) = entry else {
                    return None;
                };

                div.rect = rect;
                div.radius = radius;
                div.visible = visible;
                div.bg_color = bg_color;
                div.default_bg_color = default_bg_color;
                div.border_color = border_color;
                div.border_width = border_width;
                Some(())
            },
            |_node| (),
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
        let id = id.into();
        self.div_visible.insert(id.clone(), visible);
        if let Some(WidgetNode::Div(div)) = self.widgets.get_mut(&id) {
            div.visible = visible;
        }
    }

    pub(crate) fn set_div_enabled(&mut self, id: impl Into<String>, enabled: bool) {
        self.div_enabled.insert(id.into(), enabled);
    }

    pub(crate) fn set_div_background(&mut self, id: impl Into<String>, color: Color) {
        let id = id.into();
        self.div_background.insert(id.clone(), color);
        if let Some(WidgetNode::Div(div)) = self.widgets.get_mut(&id) {
            div.bg_color = Some(color);
        }
    }

    pub(crate) fn clear_div_background(&mut self, id: impl AsRef<str>) {
        let id = id.as_ref();
        self.div_background.remove(id);
        if let Some(WidgetNode::Div(div)) = self.widgets.get_mut(id) {
            div.bg_color = div.default_bg_color;
        }
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
        self.take_event(
            |event| matches!(event, UiEvent::ButtonClicked { button } if button == handle),
            |_event| Some(()),
        )
        .is_some()
    }

    pub(crate) fn take_text_box_changed(&mut self, handle: &TextBoxHandle) -> Option<String> {
        self.take_event(
            |event| matches!(event, UiEvent::TextBoxChanged { text_box, .. } if text_box == handle),
            |event| match event {
                UiEvent::TextBoxChanged { text, .. } => Some(text),
                _ => None,
            },
        )
    }

    pub(crate) fn take_checkbox_changed(&mut self, handle: &CheckboxHandle) -> Option<bool> {
        self.take_event(
            |event| matches!(event, UiEvent::CheckboxChanged { checkbox, .. } if checkbox == handle),
            |event| match event {
                UiEvent::CheckboxChanged { checked, .. } => Some(checked),
                _ => None,
            },
        )
    }

    pub(crate) fn take_slider_changed(&mut self, handle: &SliderHandle) -> Option<f64> {
        self.take_event(
            |event| matches!(event, UiEvent::SliderChanged { slider, .. } if slider == handle),
            |event| match event {
                UiEvent::SliderChanged { value, .. } => Some(value),
                _ => None,
            },
        )
    }

    pub(crate) fn take_radio_button_changed(&mut self, handle: &RadioButtonHandle) -> Option<bool> {
        self.take_event(
            |event| {
                matches!(event, UiEvent::RadioButtonChanged { radio_button, .. } if radio_button == handle)
            },
            |event| match event {
                UiEvent::RadioButtonChanged { selected, .. } => Some(selected),
                _ => None,
            },
        )
    }

    pub(crate) fn take_combo_box_changed(
        &mut self,
        handle: &ComboBoxHandle,
    ) -> Option<(usize, String)> {
        self.take_event(
            |event| matches!(event, UiEvent::ComboBoxChanged { combo_box, .. } if combo_box == handle),
            |event| match event {
                UiEvent::ComboBoxChanged {
                    selected_index,
                    selected_text,
                    ..
                } => Some((selected_index, selected_text)),
                _ => None,
            },
        )
    }

    fn take_event<T>(
        &mut self,
        matches: impl Fn(&UiEvent) -> bool,
        map: impl FnOnce(UiEvent) -> Option<T>,
    ) -> Option<T> {
        let index = self.events.iter().position(matches)?;
        let event = self.events.remove(index)?;
        map(event)
    }

    fn prune_active_widget_ids(&mut self) {
        clear_slot_if_absent(&mut self.active_button, &self.widgets);
        clear_slot_if_absent(&mut self.active_checkbox, &self.widgets);
        clear_slot_if_absent(&mut self.active_radio_button, &self.widgets);
        clear_slot_if_absent(&mut self.active_slider, &self.widgets);
        clear_slot_if_absent(&mut self.active_combo_box, &self.widgets);
        clear_slot_if_absent(&mut self.active_text_box_scrollbar, &self.widgets);
        clear_slot_if_absent(&mut self.focused_text_box, &self.widgets);
    }
}

fn event_widget_id(event: &UiEvent) -> &str {
    match event {
        UiEvent::ButtonClicked { button } => button.id(),
        UiEvent::CheckboxChanged { checkbox, .. } => checkbox.id(),
        UiEvent::RadioButtonChanged { radio_button, .. } => radio_button.id(),
        UiEvent::SliderChanged { slider, .. } => slider.id(),
        UiEvent::TextBoxChanged { text_box, .. } => text_box.id(),
        UiEvent::ComboBoxChanged { combo_box, .. } => combo_box.id(),
    }
}
