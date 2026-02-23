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
            self.push_event(UiEvent::ButtonClicked { id });
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
            self.push_event(UiEvent::CheckboxChanged { id, checked });
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
                id: selected_id,
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
            self.push_event(UiEvent::SliderChanged { id, value });
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
                id,
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
mod tests {
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use super::*;
    use crate::retained::node::{ComboBoxNode, SliderNode, WidgetNode};
    use crate::retained::{RetainedState, UpsertCheckboxArgs, UpsertComboBoxArgs, UpsertRadioButtonArgs, UpsertSliderArgs};

    fn sample_combo_box(is_open: bool) -> ComboBoxNode {
        ComboBoxNode {
            rect: Rect::new(10.0, 10.0, 110.0, 30.0),
            items: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            selected_index: 0,
            font_size: 14.0,
            text_color: Color::from_rgb8(255, 255, 255),
            bg_color: Color::from_rgb8(20, 20, 20),
            border_color: Color::from_rgb8(60, 60, 60),
            enabled: true,
            hovered: false,
            hovered_item: None,
            pressed: false,
            changed: false,
            is_open,
        }
    }

    fn sample_slider(step: Option<f64>) -> SliderNode {
        SliderNode {
            rect: Rect::new(0.0, 0.0, 100.0, 24.0),
            min: 0.0,
            max: 1.0,
            value: 0.0,
            step,
            text: None,
            font_size: 14.0,
            text_color: Color::from_rgb8(255, 255, 255),
            enabled: true,
            hovered: false,
            pressed: false,
            changed: false,
        }
    }

    #[test]
    fn contains_includes_edges() {
        let rect = Rect::new(10.0, 20.0, 30.0, 40.0);
        assert!(contains(rect, 10.0, 20.0));
        assert!(contains(rect, 30.0, 40.0));
        assert!(!contains(rect, 9.99, 30.0));
        assert!(!contains(rect, 20.0, 40.01));
    }

    #[test]
    fn combo_item_index_requires_open_state() {
        let closed = sample_combo_box(false);
        assert_eq!(combo_item_index_at(&closed, 50.0, 35.0), None);

        let open = sample_combo_box(true);
        assert_eq!(combo_item_index_at(&open, 50.0, 35.0), Some(0));
        assert_eq!(combo_item_index_at(&open, 50.0, 55.0), Some(1));
    }

    #[test]
    fn combo_expanded_contains_main_and_items() {
        let open = sample_combo_box(true);
        assert!(combo_expanded_contains(&open, 20.0, 20.0));
        assert!(combo_expanded_contains(&open, 20.0, 55.0));
        assert!(!combo_expanded_contains(&open, 200.0, 200.0));
    }

    #[test]
    fn slider_value_clamps_to_range() {
        let slider = sample_slider(None);
        assert_eq!(slider_value_from_cursor(&slider, -100.0), 0.0);
        assert_eq!(slider_value_from_cursor(&slider, 500.0), 1.0);
    }

    #[test]
    fn slider_value_respects_step_rounding() {
        let slider = sample_slider(Some(0.25));
        let value = slider_value_from_cursor(&slider, 40.0);
        assert!((value - 0.25).abs() < f64::EPSILON);
    }

    fn state_with_interactive_widgets() -> RetainedState {
        let mut state = RetainedState::new();
        let rect = Rect::new(0.0, 0.0, 140.0, 36.0);
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_button("btn".to_string(), rect, Some("b".to_string()), 14.0, color, true);
        state.upsert_checkbox(UpsertCheckboxArgs {
            id: "cb".to_string(),
            rect,
            text: Some("c".to_string()),
            checked: Some(false),
            font_size: 14.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "rb".to_string(),
            group: "g".to_string(),
            rect,
            text: Some("r".to_string()),
            selected: Some(false),
            font_size: 14.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_slider(UpsertSliderArgs {
            id: "sl".to_string(),
            rect,
            min: 0.0,
            max: 1.0,
            value: Some(0.0),
            step: Some(0.1),
            text: None,
            font_size: 14.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_combo_box(UpsertComboBoxArgs {
            id: "co".to_string(),
            rect,
            items: vec!["A".to_string(), "B".to_string()],
            selected_index: Some(0),
            font_size: 14.0,
            text_color: color,
            bg_color: Color::from_rgb8(20, 20, 20),
            border_color: color,
            enabled: true,
        });
        state
    }

    #[test]
    fn update_hover_flags_marks_widgets_under_cursor() {
        let mut state = state_with_interactive_widgets();
        state.update_hover_flags((10.0, 10.0));

        match state.widgets.get("btn") {
            Some(WidgetNode::Button(button)) => assert!(button.hovered),
            _ => panic!("button missing"),
        }
        match state.widgets.get("co") {
            Some(WidgetNode::ComboBox(combo_box)) => assert!(combo_box.hovered),
            _ => panic!("combo missing"),
        }
    }

    #[test]
    fn handle_mouse_press_sets_active_widget_ids() {
        let mut state = state_with_interactive_widgets();
        state.update_hover_flags((10.0, 10.0));
        state.handle_mouse_press(true);

        assert_eq!(state.active_button.as_deref(), Some("btn"));
        assert_eq!(state.active_checkbox.as_deref(), Some("cb"));
        assert_eq!(state.active_radio_button.as_deref(), Some("rb"));
        assert_eq!(state.active_slider.as_deref(), Some("sl"));
        assert_eq!(state.active_combo_box.as_deref(), Some("co"));
    }

    #[test]
    fn update_button_states_pushes_click_event_and_clears_active_on_release() {
        let mut state = state_with_interactive_widgets();
        state.update_hover_flags((10.0, 10.0));
        state.handle_mouse_press(true);
        state.update_button_states(true, true, false);
        state.update_button_states(false, false, true);

        let events = state.drain_events();
        assert!(events.iter().any(|e| matches!(e, UiEvent::ButtonClicked { id } if id == "btn")));
        assert!(state.active_button.is_none());
    }

    #[test]
    fn checkbox_radio_slider_and_combo_emit_events() {
        let mut state = state_with_interactive_widgets();
        state.update_hover_flags((10.0, 10.0));
        state.handle_mouse_press(true);

        state.update_checkbox_states(true, true);
        state.active_radio_button = Some("rb".to_string());
        state.update_radio_button_states(true, true);
        state.active_slider = Some("sl".to_string());
        state.update_slider_states((120.0, 10.0), true, true, true);

        state.active_combo_box = Some("co".to_string());
        state.update_combo_box_states(true, true); // open
        // pick the second item so selected_index actually changes (0 -> 1)
        state.update_hover_flags((10.0, 80.0));
        state.active_combo_box = Some("co".to_string());
        state.update_combo_box_states(true, true); // select

        let events = state.drain_events();
        assert!(events.iter().any(|e| matches!(e, UiEvent::CheckboxChanged { id, .. } if id == "cb")));
        assert!(events.iter().any(|e| matches!(e, UiEvent::RadioButtonChanged { id, .. } if id == "rb")));
        assert!(events.iter().any(|e| matches!(e, UiEvent::SliderChanged { id, .. } if id == "sl")));
        assert!(events.iter().any(|e| matches!(e, UiEvent::ComboBoxChanged { id, .. } if id == "co")));
    }
}
