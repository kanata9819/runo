use std::collections::{HashMap, VecDeque};

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ButtonResponse;
use crate::CheckboxResponse;
use crate::ComboBoxResponse;
use crate::RadioButtonResponse;
use crate::SliderResponse;
use crate::event::UiEvent;
use crate::retained::node::{
    ButtonNode, CheckboxNode, ComboBoxNode, LabelNode, RadioButtonNode, SliderNode, TextBoxNode,
    WidgetNode,
};
use crate::widget::text::estimate_text_width;
use crate::widget::text_box::Overflow;
use crate::widget::text_box::TextBoxResponse;

pub(crate) struct RetainedState {
    pub(super) widgets: HashMap<String, WidgetNode>,
    pub(super) order: Vec<String>,
    pub(super) active_button: Option<String>,
    pub(super) active_checkbox: Option<String>,
    pub(super) active_radio_button: Option<String>,
    pub(super) active_slider: Option<String>,
    pub(super) active_combo_box: Option<String>,
    pub(super) active_text_box_scrollbar: Option<String>,
    pub(super) focused_text_box: Option<String>,
    pub(super) events: VecDeque<UiEvent>,
    pub(super) text_clipboard: String,
    pub(super) div_visible: HashMap<String, bool>,
    pub(super) div_enabled: HashMap<String, bool>,
    pub(super) div_background: HashMap<String, Color>,
}

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
            events: VecDeque::new(),
            text_clipboard: String::new(),
            div_visible: HashMap::new(),
            div_enabled: HashMap::new(),
            div_background: HashMap::new(),
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
                    text_overridden: false,
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
                if !button.text_overridden {
                    button.text = text;
                }
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
                    text_overridden: false,
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

    pub(crate) fn upsert_checkbox(
        &mut self,
        id: String,
        rect: Rect,
        text: Option<String>,
        checked: Option<bool>,
        font_size: f32,
        text_color: Color,
        enabled: bool,
    ) -> CheckboxResponse {
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
            return CheckboxResponse::default();
        }

        let entry = self.widgets.get_mut(&id).expect("checkbox entry missing");
        match entry {
            WidgetNode::Checkbox(checkbox) => {
                checkbox.rect = rect;
                checkbox.text = text;
                // Keep internal toggled state after creation.
                // Builder-provided `checked` is treated as initial value only.
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
                CheckboxResponse::default()
            }
        }
    }

    pub(crate) fn upsert_radio_button(
        &mut self,
        id: String,
        group: String,
        rect: Rect,
        text: Option<String>,
        selected: Option<bool>,
        font_size: f32,
        text_color: Color,
        enabled: bool,
    ) -> RadioButtonResponse {
        let default_selected = selected.unwrap_or(false);
        if !self.widgets.contains_key(&id) {
            if default_selected {
                Self::clear_radio_group_selection(&mut self.widgets, &group);
            }
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::RadioButton(RadioButtonNode {
                    rect,
                    group,
                    text,
                    selected: default_selected,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                }),
            );
            return RadioButtonResponse::default();
        }

        if let Some(WidgetNode::RadioButton(radio_button)) = self.widgets.get_mut(&id) {
            radio_button.rect = rect;
            radio_button.group = group;
            radio_button.text = text;
            // Keep internal selected state after creation.
            // Builder-provided `selected` is treated as initial value only.
            let _ = selected;
            radio_button.font_size = font_size;
            radio_button.text_color = text_color;
            radio_button.enabled = enabled;
            return RadioButtonResponse {
                selected: radio_button.selected,
                hovered: radio_button.hovered,
                pressed: radio_button.pressed,
                changed: radio_button.changed,
            };
        }

        if default_selected {
            Self::clear_radio_group_selection(&mut self.widgets, &group);
        }
        self.widgets.insert(
            id,
            WidgetNode::RadioButton(RadioButtonNode {
                rect,
                group,
                text,
                selected: default_selected,
                font_size,
                text_color,
                enabled,
                hovered: false,
                pressed: false,
                changed: false,
            }),
        );
        RadioButtonResponse::default()
    }

    pub(crate) fn upsert_slider(
        &mut self,
        id: String,
        rect: Rect,
        min: f64,
        max: f64,
        value: Option<f64>,
        step: Option<f64>,
        text: Option<String>,
        font_size: f32,
        text_color: Color,
        enabled: bool,
    ) -> SliderResponse {
        let (min, max) = normalize_range(min, max);
        let default_value = snap_and_clamp(value.unwrap_or(min), min, max, step);
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::Slider(SliderNode {
                    rect,
                    min,
                    max,
                    value: default_value,
                    step,
                    text,
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                }),
            );
            return SliderResponse::default();
        }

        if let Some(WidgetNode::Slider(slider)) = self.widgets.get_mut(&id) {
            slider.rect = rect;
            slider.min = min;
            slider.max = max;
            slider.step = step;
            slider.text = text;
            slider.font_size = font_size;
            slider.text_color = text_color;
            slider.enabled = enabled;
            slider.value = snap_and_clamp(slider.value, slider.min, slider.max, slider.step);
            // Keep internal value after creation.
            // Builder-provided `value` is treated as initial value only.
            let _ = value;
            return SliderResponse {
                value: slider.value,
                hovered: slider.hovered,
                pressed: slider.pressed,
                changed: slider.changed,
            };
        }

        self.widgets.insert(
            id,
            WidgetNode::Slider(SliderNode {
                rect,
                min,
                max,
                value: default_value,
                step,
                text,
                font_size,
                text_color,
                enabled,
                hovered: false,
                pressed: false,
                changed: false,
            }),
        );
        SliderResponse::default()
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
        overflow_x: Overflow,
        overflow_y: Overflow,
    ) -> TextBoxResponse {
        if !self.widgets.contains_key(&id) {
            let text = text.unwrap_or_default();
            let text_advance = estimate_text_width(&text, font_size) as f64;
            let caret_index = text.chars().count();
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::TextBox(TextBoxNode {
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
                    text_advance,
                    caret_index,
                    scroll_x: 0.0,
                    scroll_y: 0.0,
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
                    text_box.text_advance =
                        estimate_text_width(&text_box.text, text_box.font_size) as f64;
                    text_box.caret_index = text_box.text.chars().count();
                }
                text_box.placeholder = placeholder;
                if (text_box.font_size - font_size).abs() > f32::EPSILON {
                    text_box.font_size = font_size;
                    // Re-measure on next paint; keep an estimate for current frame logic.
                    text_box.text_advance =
                        estimate_text_width(&text_box.text, text_box.font_size) as f64;
                }
                text_box.text_color = text_color;
                text_box.bg_color = bg_color;
                text_box.border_color = border_color;
                text_box.enabled = enabled;
                text_box.overflow_x = overflow_x;
                text_box.overflow_y = overflow_y;
                TextBoxResponse {
                    text: text_box.text.clone(),
                    hovered: text_box.hovered,
                    focused: text_box.focused,
                    changed: text_box.changed,
                }
            }
            _ => {
                let text = text.unwrap_or_default();
                let text_advance = estimate_text_width(&text, font_size) as f64;
                let caret_index = text.chars().count();
                *entry = WidgetNode::TextBox(TextBoxNode {
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
                    text_advance,
                    caret_index,
                    scroll_x: 0.0,
                    scroll_y: 0.0,
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
        button.text_overridden = true;
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

    pub(crate) fn slider_response(&self, id: impl AsRef<str>) -> SliderResponse {
        let Some(WidgetNode::Slider(slider)) = self.widgets.get(id.as_ref()) else {
            return SliderResponse::default();
        };
        SliderResponse {
            value: slider.value,
            hovered: slider.hovered,
            pressed: slider.pressed,
            changed: slider.changed,
        }
    }

    pub(crate) fn set_slider_value(&mut self, id: impl AsRef<str>, value: f64) {
        let Some(WidgetNode::Slider(slider)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };
        let next = snap_and_clamp(value, slider.min, slider.max, slider.step);
        slider.changed = (slider.value - next).abs() > f64::EPSILON;
        slider.value = next;
    }

    pub(crate) fn set_slider_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::Slider(slider)) = self.widgets.get_mut(id_ref) else {
            return;
        };
        slider.enabled = enabled;
        if !enabled {
            slider.hovered = false;
            slider.pressed = false;
            slider.changed = false;
            if self.active_slider.as_deref() == Some(id_ref) {
                self.active_slider = None;
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

    pub(super) fn push_event(&mut self, event: UiEvent) {
        self.events.push_back(event);
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

fn normalize_range(min: f64, max: f64) -> (f64, f64) {
    if min <= max { (min, max) } else { (max, min) }
}

fn snap_and_clamp(value: f64, min: f64, max: f64, step: Option<f64>) -> f64 {
    let mut clamped = value.clamp(min, max);
    if let Some(step) = step
        && step > 0.0
    {
        let snapped = ((clamped - min) / step).round() * step + min;
        clamped = snapped.clamp(min, max);
    }
    clamped
}
