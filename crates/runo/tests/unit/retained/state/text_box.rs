
use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::node::WidgetNode;
use crate::retained::state::{RetainedState, UpsertTextBoxArgs};
use crate::widget::text_box::Overflow;

fn rect() -> Rect {
    Rect::new(0.0, 0.0, 260.0, 44.0)
}

#[test]
fn upsert_text_box_updates_text_when_text_is_provided() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_text_box(UpsertTextBoxArgs {
        id: "tb".to_string(),
        rect: rect(),
        text: Some("a".to_string()),
        placeholder: Some("p".to_string()),
        font_size: 16.0,
        text_color: color,
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: color,
        enabled: true,
        overflow_x: Overflow::Auto,
        overflow_y: Overflow::Hidden,
    });

    state.upsert_text_box(UpsertTextBoxArgs {
        id: "tb".to_string(),
        rect: rect(),
        text: Some("updated".to_string()),
        placeholder: Some("p2".to_string()),
        font_size: 20.0,
        text_color: color,
        bg_color: Color::from_rgb8(40, 40, 40),
        border_color: color,
        enabled: true,
        overflow_x: Overflow::Scroll,
        overflow_y: Overflow::Auto,
    });

    let response = state.text_box_response("tb");
    assert_eq!(response.text, "updated");
}

#[test]
fn set_text_box_enabled_false_clears_focus_and_active_scrollbar() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);
    state.upsert_text_box(UpsertTextBoxArgs {
        id: "tb".to_string(),
        rect: rect(),
        text: Some("a".to_string()),
        placeholder: None,
        font_size: 16.0,
        text_color: color,
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: color,
        enabled: true,
        overflow_x: Overflow::Auto,
        overflow_y: Overflow::Hidden,
    });
    state.focused_text_box = Some("tb".to_string());
    state.active_text_box_scrollbar = Some("tb".to_string());
    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get_mut("tb") {
        tb.hovered = true;
        tb.focused = true;
        tb.changed = true;
    }

    state.set_text_box_enabled("tb", false);
    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get("tb") {
        assert!(!tb.enabled);
        assert!(!tb.hovered);
        assert!(!tb.focused);
        assert!(!tb.changed);
    } else {
        panic!("text box missing");
    }
    assert!(state.focused_text_box.is_none());
    assert!(state.active_text_box_scrollbar.is_none());
}
