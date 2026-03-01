use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::node::WidgetNode;
use crate::retained::state::{RetainedState, UpsertSliderArgs};

fn rect() -> Rect {
    Rect::new(0.0, 0.0, 220.0, 40.0)
}

#[test]
fn upsert_slider_swapped_range_and_snaps_initial_value() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    let response = state.upsert_slider(UpsertSliderArgs {
        id: "s".to_string(),
        rect: rect(),
        min: 10.0,
        max: 0.0,
        value: Some(5.2),
        step: Some(0.5),
        text: Some("s".to_string()),
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });
    assert!((response.value - 5.0).abs() < f64::EPSILON);
}

#[test]
fn set_slider_enabled_false_clears_flags_and_active_id() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_slider(UpsertSliderArgs {
        id: "s".to_string(),
        rect: rect(),
        min: 0.0,
        max: 1.0,
        value: Some(0.3),
        step: None,
        text: None,
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });
    state.active_slider = Some("s".to_string());
    if let Some(WidgetNode::Slider(s)) = state.widgets.get_mut("s") {
        s.hovered = true;
        s.pressed = true;
        s.changed = true;
    }

    state.set_slider_enabled("s", false);
    if let Some(WidgetNode::Slider(s)) = state.widgets.get("s") {
        assert!(!s.enabled);
        assert!(!s.hovered);
        assert!(!s.pressed);
        assert!(!s.changed);
    } else {
        panic!("slider missing");
    }
    assert!(state.active_slider.is_none());
}

#[test]
fn set_slider_value_without_change_keeps_changed_false() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_slider(UpsertSliderArgs {
        id: "s".to_string(),
        rect: rect(),
        min: 0.0,
        max: 1.0,
        value: Some(0.5),
        step: Some(0.25),
        text: None,
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });
    state.set_slider_value("s", 0.5);
    let response = state.slider_response("s");
    assert!((response.value - 0.5).abs() < f64::EPSILON);
    assert!(!response.changed);
}

#[test]
fn upsert_slider_updates_existing_entry_and_clamps_current_value() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_slider(UpsertSliderArgs {
        id: "s".to_string(),
        rect: rect(),
        min: 0.0,
        max: 10.0,
        value: Some(9.0),
        step: Some(1.0),
        text: Some("a".to_string()),
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });

    let response = state.upsert_slider(UpsertSliderArgs {
        id: "s".to_string(),
        rect: Rect::new(1.0, 1.0, 100.0, 20.0),
        min: 0.0,
        max: 5.0,
        value: Some(100.0),
        step: Some(0.5),
        text: Some("b".to_string()),
        font_size: 18.0,
        text_color: color,
        enabled: false,
    });
    assert!(response.value <= 5.0);
}
