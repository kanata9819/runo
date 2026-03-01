use vello::kurbo::Rect;
use vello::peniko::Color;

use super::*;
use crate::retained::node::{ComboBoxNode, SliderNode, WidgetNode};
use crate::retained::{
    RetainedState, UpsertCheckboxArgs, UpsertComboBoxArgs, UpsertRadioButtonArgs, UpsertSliderArgs,
};

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
    state.upsert_button(
        "btn".to_string(),
        rect,
        Some("b".to_string()),
        14.0,
        color,
        true,
    );
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
    assert!(
        events
            .iter()
            .any(|e| matches!(e, UiEvent::ButtonClicked { button } if button.id() == "btn"))
    );
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
    assert!(
        events.iter().any(
            |e| matches!(e, UiEvent::CheckboxChanged { checkbox, .. } if checkbox.id() == "cb")
        )
    );
    assert!(
            events
                .iter()
                .any(|e| matches!(e, UiEvent::RadioButtonChanged { radio_button, .. } if radio_button.id() == "rb"))
        );
    assert!(
        events
            .iter()
            .any(|e| matches!(e, UiEvent::SliderChanged { slider, .. } if slider.id() == "sl"))
    );
    assert!(events.iter().any(
        |e| matches!(e, UiEvent::ComboBoxChanged { combo_box, .. } if combo_box.id() == "co")
    ));
}
