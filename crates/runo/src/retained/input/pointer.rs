use vello::kurbo::Rect;

use crate::event::UiEvent;
use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

impl RetainedState {
    pub(super) fn update_hover_flags(&mut self, cursor_pos: (f64, f64)) {
        let open_overlay_id = self.order.iter().rev().find_map(|id| {
            let WidgetNode::ComboBox(combo_box) = self.widgets.get(id)? else {
                return None;
            };

            if combo_box.enabled
                && combo_box.is_open
                && combo_expanded_contains(combo_box, cursor_pos.0, cursor_pos.1)
            {
                Some(id.clone())
            } else {
                None
            }
        });

        for (id, node) in &mut self.widgets {
            match node {
                WidgetNode::Button(button) => {
                    button.clicked = false;
                    button.hovered = if !button.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(button.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::Checkbox(checkbox) => {
                    checkbox.changed = false;
                    checkbox.hovered = if !checkbox.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(checkbox.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::RadioButton(radio_button) => {
                    radio_button.changed = false;
                    radio_button.hovered = if !radio_button.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(radio_button.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::Slider(slider) => {
                    slider.changed = false;
                    slider.hovered = if !slider.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(slider.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::TextBox(text_box) => {
                    text_box.changed = false;
                    text_box.hovered = if !text_box.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(text_box.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::ComboBox(combo_box) => {
                    combo_box.changed = false;
                    if !combo_box.enabled {
                        combo_box.hovered = false;
                        combo_box.hovered_item = None;
                        combo_box.pressed = false;
                        combo_box.is_open = false;
                        continue;
                    }
                    combo_box.hovered = contains(combo_box.rect, cursor_pos.0, cursor_pos.1);
                    combo_box.hovered_item = if open_overlay_id
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                    {
                        combo_item_index_at(combo_box, cursor_pos.0, cursor_pos.1)
                    } else {
                        None
                    };
                }
                WidgetNode::Label(_) => {}
            }
        }
    }

    pub(super) fn handle_mouse_press(&mut self, mouse_pressed: bool) {
        if mouse_pressed {
            self.active_button = self.order.iter().rev().find_map(|id| {
                let WidgetNode::Button(button) = self.widgets.get(id)? else {
                    return None;
                };

                if button.enabled && button.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.active_checkbox = self.order.iter().rev().find_map(|id| {
                let WidgetNode::Checkbox(checkbox) = self.widgets.get(id)? else {
                    return None;
                };

                if checkbox.enabled && checkbox.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.active_radio_button = self.order.iter().rev().find_map(|id| {
                let WidgetNode::RadioButton(radio_button) = self.widgets.get(id)? else {
                    return None;
                };

                if radio_button.enabled && radio_button.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.active_slider = self.order.iter().rev().find_map(|id| {
                let WidgetNode::Slider(slider) = self.widgets.get(id)? else {
                    return None;
                };

                if slider.enabled && slider.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.focused_text_box = self.order.iter().rev().find_map(|id| {
                let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                    return None;
                };

                if text_box.enabled && text_box.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.active_combo_box = self.order.iter().rev().find_map(|id| {
                let WidgetNode::ComboBox(combo_box) = self.widgets.get(id)? else {
                    return None;
                };

                if combo_box.enabled && (combo_box.hovered || combo_box.hovered_item.is_some()) {
                    Some(id.clone())
                } else {
                    None
                }
            });
        }
    }

    pub(super) fn update_button_states(
        &mut self,
        mouse_pressed: bool,
        mouse_down: bool,
        mouse_released: bool,
    ) {
        let mut clicked_ids = Vec::new();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::Button(button) = node {
                if !button.enabled {
                    button.pressed = false;
                    button.clicked = false;
                    continue;
                }

                button.pressed = mouse_down
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false);

                if mouse_pressed
                    && button.hovered
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                {
                    button.clicked = true;
                    clicked_ids.push(id.clone());
                }
            }
        }

        for id in clicked_ids {
            self.push_event(UiEvent::ButtonClicked {
                button: crate::widget::button::ButtonHandle::new(id),
            });
        }

        if mouse_released {
            self.active_button = None;
        }
    }

    pub(super) fn update_checkbox_states(&mut self, mouse_down: bool, mouse_released: bool) {
        let mut changed = Vec::new();
        let active_checkbox = self.active_checkbox.clone();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::Checkbox(checkbox) = node {
                if !checkbox.enabled {
                    checkbox.hovered = false;
                    checkbox.pressed = false;
                    checkbox.changed = false;
                    continue;
                }

                let is_active = active_checkbox
                    .as_ref()
                    .map(|active| active == id)
                    .unwrap_or(false);

                checkbox.pressed = mouse_down && is_active;

                if mouse_released && is_active && checkbox.hovered {
                    checkbox.checked = !checkbox.checked;
                    checkbox.changed = true;
                    changed.push((id.clone(), checkbox.checked));
                }
            }
        }

        for (id, checked) in changed {
            self.push_event(UiEvent::CheckboxChanged {
                checkbox: crate::widget::checkbox::CheckboxHandle::new(id),
                checked,
            });
        }

        if mouse_released {
            self.active_checkbox = None;
        }
    }

    pub(super) fn update_radio_button_states(&mut self, mouse_down: bool, mouse_released: bool) {
        let active_radio_button = self.active_radio_button.clone();
        let mut selected_id: Option<String> = None;
        let mut selected_group = String::new();

        for (id, node) in &mut self.widgets {
            if let WidgetNode::RadioButton(radio_button) = node {
                if !radio_button.enabled {
                    radio_button.hovered = false;
                    radio_button.pressed = false;
                    radio_button.changed = false;
                    continue;
                }

                let is_active = active_radio_button
                    .as_ref()
                    .map(|active| active == id)
                    .unwrap_or(false);

                radio_button.pressed = mouse_down && is_active;

                if mouse_released && is_active && radio_button.hovered && !radio_button.selected {
                    selected_id = Some(id.clone());
                    selected_group = radio_button.group.clone();
                }
            }
        }

        if let Some(selected_id) = selected_id {
            for (id, node) in &mut self.widgets {
                if let WidgetNode::RadioButton(radio_button) = node
                    && radio_button.group == selected_group
                {
                    let next_selected = id == &selected_id;
                    radio_button.changed = radio_button.selected != next_selected;
                    radio_button.selected = next_selected;
                }
            }

            self.push_event(UiEvent::RadioButtonChanged {
                radio_button: crate::widget::radio_button::RadioButtonHandle::new(selected_id),
                group: selected_group,
                selected: true,
            });
        }

        if mouse_released {
            self.active_radio_button = None;
        }
    }

    pub(super) fn update_slider_states(
        &mut self,
        cursor_pos: (f64, f64),
        mouse_pressed: bool,
        mouse_down: bool,
        mouse_released: bool,
    ) {
        let active_slider = self.active_slider.clone();
        let mut changed = Vec::new();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::Slider(slider) = node {
                if !slider.enabled {
                    slider.hovered = false;
                    slider.pressed = false;
                    slider.changed = false;
                    continue;
                }

                let is_active = active_slider
                    .as_ref()
                    .map(|active| active == id)
                    .unwrap_or(false);

                slider.pressed = mouse_down && is_active;

                if (mouse_pressed || mouse_down) && is_active {
                    let next_value = slider_value_from_cursor(slider, cursor_pos.0);
                    slider.changed = (slider.value - next_value).abs() > f64::EPSILON;
                    slider.value = next_value;

                    if slider.changed {
                        changed.push((id.clone(), slider.value));
                    }
                }
            }
        }

        for (id, value) in changed {
            self.push_event(UiEvent::SliderChanged {
                slider: crate::widget::slider::SliderHandle::new(id),
                value,
            });
        }

        if mouse_released {
            self.active_slider = None;
        }
    }

    pub(super) fn update_combo_box_states(&mut self, mouse_down: bool, mouse_released: bool) {
        let mut changed = Vec::new();
        let active_combo_box = self.active_combo_box.clone();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::ComboBox(combo_box) = node {
                if !combo_box.enabled {
                    Self::reset_disabled_combo_box(combo_box);
                    continue;
                }

                let is_active = active_combo_box
                    .as_ref()
                    .map(|active| active == id)
                    .unwrap_or(false);

                combo_box.pressed = mouse_down && is_active;

                if !mouse_released {
                    continue;
                }

                if let Some((selected_index, selected_text)) =
                    Self::apply_combo_box_release(combo_box, is_active)
                {
                    changed.push((id.clone(), selected_index, selected_text));
                }
            }
        }

        for (id, selected_index, selected_text) in changed {
            self.push_event(UiEvent::ComboBoxChanged {
                combo_box: crate::widget::combo_box::ComboBoxHandle::new(id),
                selected_index,
                selected_text,
            });
        }

        if mouse_released {
            self.active_combo_box = None;
        }
    }

    fn reset_disabled_combo_box(combo_box: &mut crate::retained::node::ComboBoxNode) {
        combo_box.hovered = false;
        combo_box.hovered_item = None;
        combo_box.pressed = false;
        combo_box.changed = false;
        combo_box.is_open = false;
    }

    fn apply_combo_box_release(
        combo_box: &mut crate::retained::node::ComboBoxNode,
        is_active: bool,
    ) -> Option<(usize, String)> {
        if !is_active {
            combo_box.is_open = false;
            return None;
        }

        if combo_box.is_open {
            if let Some(index) = combo_box.hovered_item
                && index < combo_box.items.len()
            {
                combo_box.changed = combo_box.selected_index != index;
                combo_box.selected_index = index;
                combo_box.is_open = false;

                if combo_box.changed {
                    return Some((
                        combo_box.selected_index,
                        combo_box.items[combo_box.selected_index].clone(),
                    ));
                }

                return None;
            }

            combo_box.is_open = false;
            return None;
        }

        if combo_box.hovered {
            combo_box.is_open = true;
        }
        None
    }
}

pub(super) fn contains(rect: Rect, x: f64, y: f64) -> bool {
    x >= rect.x0 && x <= rect.x1 && y >= rect.y0 && y <= rect.y1
}

fn combo_item_index_at(
    combo_box: &crate::retained::node::ComboBoxNode,
    x: f64,
    y: f64,
) -> Option<usize> {
    if !combo_box.is_open || combo_box.items.is_empty() {
        return None;
    }

    let item_height = combo_box.rect.height();
    for index in 0..combo_box.items.len() {
        let top = combo_box.rect.y1 + item_height * index as f64;
        let rect = Rect::new(combo_box.rect.x0, top, combo_box.rect.x1, top + item_height);

        if contains(rect, x, y) {
            return Some(index);
        }
    }

    None
}

fn combo_expanded_contains(
    combo_box: &crate::retained::node::ComboBoxNode,
    x: f64,
    y: f64,
) -> bool {
    if contains(combo_box.rect, x, y) {
        return true;
    }
    combo_item_index_at(combo_box, x, y).is_some()
}

fn slider_value_from_cursor(slider: &crate::retained::node::SliderNode, cursor_x: f64) -> f64 {
    let x0 = slider.rect.x0 + 12.0;
    let x1 = slider.rect.x1 - 12.0;
    let width = (x1 - x0).max(1.0);
    let ratio = ((cursor_x - x0) / width).clamp(0.0, 1.0);
    let mut value = slider.min + (slider.max - slider.min) * ratio;

    if let Some(step) = slider.step
        && step > 0.0
    {
        value = ((value - slider.min) / step).round() * step + slider.min;
    }

    value.clamp(slider.min, slider.max)
}

#[cfg(test)]
#[path = "../../../tests/unit/retained/input/pointer.rs"]
mod tests;
