mod button;
mod checkbox;
mod combo_box;
mod core;
mod radio_button;
mod slider;
mod text_box;

use std::collections::{HashMap, VecDeque};

use vello::peniko::Color;

use crate::event::UiEvent;
use crate::retained::node::WidgetNode;

pub(crate) use checkbox::UpsertCheckboxArgs;
pub(crate) use combo_box::UpsertComboBoxArgs;
pub(crate) use radio_button::UpsertRadioButtonArgs;
pub(crate) use slider::UpsertSliderArgs;
pub(crate) use text_box::UpsertTextBoxArgs;

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

impl RetainedState {
    pub(super) fn upsert_widget_node<R, FNode, FUpdate, FNewResponse>(
        &mut self,
        id: String,
        mut make_new_node: FNode,
        update_existing: FUpdate,
        new_or_replaced_response: FNewResponse,
    ) -> R
    where
        FNode: FnMut() -> WidgetNode,
        FUpdate: FnOnce(&mut WidgetNode) -> Option<R>,
        FNewResponse: FnOnce(&WidgetNode) -> R,
    {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(id.clone(), make_new_node());
            let node = self
                .widgets
                .get(&id)
                .expect("newly inserted widget entry missing");

            return new_or_replaced_response(node);
        }

        let entry = self.widgets.get_mut(&id).expect("widget entry missing");

        if let Some(response) = update_existing(entry) {
            return response;
        }

        *entry = make_new_node();
        new_or_replaced_response(entry)
    }
}

#[cfg(test)]
mod tests {
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use super::*;

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 120.0, 40.0)
    }

    #[test]
    fn normalize_range_swaps_when_min_greater_than_max() {
        assert_eq!(normalize_range(10.0, 2.0), (2.0, 10.0));
        assert_eq!(normalize_range(-1.0, 3.0), (-1.0, 3.0));
    }

    #[test]
    fn snap_and_clamp_applies_step_and_bounds() {
        assert!((snap_and_clamp(0.73, 0.0, 1.0, Some(0.25)) - 0.75).abs() < f64::EPSILON);
        assert_eq!(snap_and_clamp(-3.0, 0.0, 1.0, Some(0.1)), 0.0);
        assert_eq!(snap_and_clamp(3.0, 0.0, 1.0, Some(0.1)), 1.0);
    }

    #[test]
    fn slider_set_value_respects_step_and_changed_flag() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(255, 255, 255);
        state.upsert_slider(UpsertSliderArgs {
            id: "s".to_string(),
            rect: rect(),
            min: 0.0,
            max: 1.0,
            value: Some(0.0),
            step: Some(0.25),
            text: None,
            font_size: 14.0,
            text_color: color,
            enabled: true,
        });

        state.set_slider_value("s", 0.62);
        let response = state.slider_response("s");
        assert!((response.value - 0.5).abs() < f64::EPSILON);
        assert!(response.changed);
    }

    #[test]
    fn combo_box_selected_index_is_clamped() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(255, 255, 255);
        state.upsert_combo_box(UpsertComboBoxArgs {
            id: "c".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string()],
            selected_index: Some(0),
            font_size: 14.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });

        state.set_combo_box_selected_index("c", 99);
        let response = state.combo_box_response("c");
        assert_eq!(response.selected_index, 1);
        assert_eq!(response.selected_text, "b");
    }

    #[test]
    fn combo_box_set_items_keeps_or_clamps_selection() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(255, 255, 255);
        state.upsert_combo_box(UpsertComboBoxArgs {
            id: "c".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            selected_index: Some(2),
            font_size: 14.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });

        state.set_combo_box_items("c", ["x", "y", "z", "w"]);
        let response = state.combo_box_response("c");
        assert_eq!(response.selected_index, 2);
        assert_eq!(response.selected_text, "z");

        state.set_combo_box_items("c", ["only"]);
        let response = state.combo_box_response("c");
        assert_eq!(response.selected_index, 0);
        assert_eq!(response.selected_text, "only");
    }

    #[test]
    fn combo_box_set_items_empty_resets_selection_and_closes() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(255, 255, 255);
        state.upsert_combo_box(UpsertComboBoxArgs {
            id: "c".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string()],
            selected_index: Some(1),
            font_size: 14.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });

        state.set_combo_box_items("c", std::iter::empty::<&str>());
        let response = state.combo_box_response("c");
        assert_eq!(response.selected_index, 0);
        assert_eq!(response.selected_text, "");
        assert!(!response.is_open);
    }

    #[test]
    fn selecting_radio_button_clears_same_group_selection() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(255, 255, 255);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 14.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r2".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(false),
            font_size: 14.0,
            text_color: color,
            enabled: true,
        });

        state.set_radio_button_selected("r2", true);
        assert!(!state.radio_button_response("r1").selected);
        assert!(state.radio_button_response("r2").selected);
    }

    #[test]
    fn div_state_defaults_and_background_clear_work() {
        let mut state = RetainedState::new();
        assert!(state.div_visible("panel"));
        assert!(state.div_enabled("panel"));
        assert!(state.div_background("panel").is_none());

        let bg = Color::from_rgb8(10, 20, 30);
        state.set_div_visible("panel", false);
        state.set_div_enabled("panel", false);
        state.set_div_background("panel", bg);
        assert!(!state.div_visible("panel"));
        assert!(!state.div_enabled("panel"));
        assert!(state.div_background("panel").is_some());

        state.clear_div_background("panel");
        assert!(state.div_background("panel").is_none());
    }

    #[test]
    fn event_queue_pop_and_drain_preserve_order() {
        let mut state = RetainedState::new();
        state.push_event(UiEvent::ButtonClicked {
            button: crate::widget::button::ButtonHandle::new("a".to_string()),
        });
        state.push_event(UiEvent::ButtonClicked {
            button: crate::widget::button::ButtonHandle::new("b".to_string()),
        });

        match state.pop_event() {
            Some(UiEvent::ButtonClicked { button }) => assert_eq!(button.id(), "a"),
            _ => panic!("unexpected event"),
        }
        let remaining = state.drain_events();
        assert_eq!(remaining.len(), 1);
        match &remaining[0] {
            UiEvent::ButtonClicked { button } => assert_eq!(button.id(), "b"),
            _ => panic!("unexpected event"),
        }
    }

    #[test]
    fn upsert_helper_keeps_order_unique_and_supports_type_replacement() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);

        state.upsert_label(
            "same".to_string(),
            rect(),
            "label".to_string(),
            14.0,
            color,
            true,
        );
        assert_eq!(state.order, vec!["same".to_string()]);

        let _ = state.upsert_button(
            "same".to_string(),
            rect(),
            Some("b".to_string()),
            14.0,
            color,
            true,
        );
        assert_eq!(state.order, vec!["same".to_string()]);
        assert!(matches!(
            state.widgets.get("same"),
            Some(crate::retained::node::WidgetNode::Button(_))
        ));
    }
}
