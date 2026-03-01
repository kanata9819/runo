
use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

fn rect() -> Rect {
    Rect::new(0.0, 0.0, 100.0, 40.0)
}

#[test]
fn upsert_button_creates_and_updates_response() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);

    let created = state.upsert_button(
        "btn".to_string(),
        rect(),
        Some("hello".to_string()),
        16.0,
        color,
        true,
    );
    assert!(!created.hovered);
    assert!(!created.pressed);
    assert!(!created.clicked);

    if let Some(WidgetNode::Button(button)) = state.widgets.get("btn") {
        assert_eq!(button.text.as_deref(), Some("hello"));
        assert!(!button.text_overridden);
        assert!(button.enabled);
    } else {
        panic!("button node missing");
    }

    state.set_button_text("btn", Some("override".to_string()));
    let _ = state.upsert_button(
        "btn".to_string(),
        rect(),
        Some("ignored".to_string()),
        20.0,
        color,
        true,
    );
    if let Some(WidgetNode::Button(button)) = state.widgets.get("btn") {
        assert_eq!(button.text.as_deref(), Some("override"));
        assert!(button.text_overridden);
        assert_eq!(button.font_size, 20.0);
    } else {
        panic!("button node missing");
    }
}

#[test]
fn set_button_enabled_false_clears_interaction_and_active_button() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_button(
        "btn".to_string(),
        rect(),
        Some("hello".to_string()),
        16.0,
        color,
        true,
    );

    if let Some(WidgetNode::Button(button)) = state.widgets.get_mut("btn") {
        button.hovered = true;
        button.pressed = true;
        button.clicked = true;
    }
    state.active_button = Some("btn".to_string());

    state.set_button_enabled("btn", false);

    if let Some(WidgetNode::Button(button)) = state.widgets.get("btn") {
        assert!(!button.enabled);
        assert!(!button.hovered);
        assert!(!button.pressed);
        assert!(!button.clicked);
    } else {
        panic!("button node missing");
    }
    assert!(state.active_button.is_none());
}
