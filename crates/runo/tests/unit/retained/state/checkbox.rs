use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::node::WidgetNode;
use crate::retained::state::{RetainedState, UpsertCheckboxArgs};

fn rect() -> Rect {
    Rect::new(0.0, 0.0, 120.0, 36.0)
}

#[test]
fn upsert_checkbox_keeps_existing_checked_when_checked_arg_is_none() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_checkbox(UpsertCheckboxArgs {
        id: "cb".to_string(),
        rect: rect(),
        text: Some("cb".to_string()),
        checked: Some(true),
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });
    state.upsert_checkbox(UpsertCheckboxArgs {
        id: "cb".to_string(),
        rect: rect(),
        text: Some("cb2".to_string()),
        checked: None,
        font_size: 18.0,
        text_color: color,
        enabled: true,
    });

    assert!(state.checkbox_response("cb").checked);
}

#[test]
fn upsert_checkbox_updates_existing_checked_when_checked_arg_is_some() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_checkbox(UpsertCheckboxArgs {
        id: "cb".to_string(),
        rect: rect(),
        text: Some("cb".to_string()),
        checked: Some(true),
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });
    state.upsert_checkbox(UpsertCheckboxArgs {
        id: "cb".to_string(),
        rect: rect(),
        text: Some("cb".to_string()),
        checked: Some(false),
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });

    assert!(!state.checkbox_response("cb").checked);
}

#[test]
fn set_checkbox_enabled_false_clears_flags_and_active_id() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_checkbox(UpsertCheckboxArgs {
        id: "cb".to_string(),
        rect: rect(),
        text: Some("cb".to_string()),
        checked: Some(false),
        font_size: 16.0,
        text_color: color,
        enabled: true,
    });
    state.active_checkbox = Some("cb".to_string());
    if let Some(WidgetNode::Checkbox(cb)) = state.widgets.get_mut("cb") {
        cb.hovered = true;
        cb.pressed = true;
        cb.changed = true;
    }

    state.set_checkbox_enabled("cb", false);
    if let Some(WidgetNode::Checkbox(cb)) = state.widgets.get("cb") {
        assert!(!cb.enabled);
        assert!(!cb.hovered);
        assert!(!cb.pressed);
        assert!(!cb.changed);
    } else {
        panic!("checkbox missing");
    }
    assert!(state.active_checkbox.is_none());
}
