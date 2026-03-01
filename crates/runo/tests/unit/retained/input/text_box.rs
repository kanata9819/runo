
use vello::kurbo::Rect;
use vello::peniko::Color;

use super::*;
use crate::event::UiEvent;
use crate::input::InputFrame;
use crate::retained::RetainedState;
use crate::retained::node::{TextBoxNode, WidgetNode};
use crate::retained::state::UpsertTextBoxArgs;
use crate::widget::text_box::Overflow;

fn sample_text_box(text: &str) -> TextBoxNode {
    TextBoxNode {
        rect: Rect::new(0.0, 0.0, 120.0, 44.0),
        text: text.to_string(),
        placeholder: None,
        font_size: 16.0,
        text_color: Color::from_rgb8(255, 255, 255),
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: Color::from_rgb8(60, 60, 60),
        enabled: true,
        overflow_x: Overflow::Auto,
        overflow_y: Overflow::Hidden,
        text_advance: 0.0,
        caret_index: 0,
        scroll_x: 0.0,
        scroll_y: 0.0,
        hovered: false,
        focused: false,
        changed: false,
    }
}

#[test]
fn char_to_byte_index_handles_multibyte_characters() {
    let text = "AあB";
    assert_eq!(char_to_byte_index(text, 0), 0);
    assert_eq!(char_to_byte_index(text, 1), 1);
    assert_eq!(char_to_byte_index(text, 2), 4);
    assert_eq!(char_to_byte_index(text, 10), text.len());
}

#[test]
fn insert_text_at_caret_advances_caret_by_chars() {
    let mut text = "ac".to_string();
    let mut caret = 1;

    insert_text_at_caret(&mut text, &mut caret, "あい");

    assert_eq!(text, "aあいc");
    assert_eq!(caret, 3);
}

#[test]
fn remove_char_before_and_at_caret_work_for_unicode() {
    let mut text = "AあB".to_string();
    let mut caret = 2;

    let removed_before = remove_char_before_caret(&mut text, &mut caret);
    assert!(removed_before);
    assert_eq!(text, "AB");
    assert_eq!(caret, 1);

    let removed_at = remove_char_at_caret(&mut text, 1);
    assert!(removed_at);
    assert_eq!(text, "A");
}

#[test]
fn move_caret_vertical_clamps_column_on_shorter_lines() {
    let text = "abcd\nef\nxyz";
    let down = move_caret_vertical(text, 3, 1);
    let up = move_caret_vertical(text, down, -1);

    assert_eq!(down, 7);
    assert_eq!(up, 2);
}

#[test]
fn line_col_and_char_index_round_trip() {
    let text = "ab\ncde";
    let (line, col) = line_col_from_char_index(text, 4);
    let lines: Vec<&str> = text.split('\n').collect();
    let index = char_index_from_line_col(&lines, line, col);
    assert_eq!((line, col), (1, 1));
    assert_eq!(index, 4);
}

#[test]
fn set_scroll_from_scrollbar_cursor_clamps_bounds() {
    let mut text_box = sample_text_box("very long text that should exceed the box width");
    text_box.text_advance = 800.0;

    set_scroll_from_scrollbar_cursor(&mut text_box, -100.0);
    assert_eq!(text_box.scroll_x, 0.0);

    let max = text_box_max_scroll_x(&text_box);
    set_scroll_from_scrollbar_cursor(&mut text_box, 1_000.0);
    assert!((text_box.scroll_x - max).abs() < f64::EPSILON);
}

#[test]
fn scrollbar_track_hit_test_uses_bottom_strip() {
    let text_box = sample_text_box("text");
    assert!(text_box_scrollbar_track_contains(&text_box, 20.0, 40.0));
    assert!(!text_box_scrollbar_track_contains(&text_box, 20.0, 5.0));
}

fn empty_input() -> InputFrame {
    InputFrame {
        cursor_pos: (0.0, 0.0),
        mouse_down: false,
        mouse_pressed: false,
        mouse_released: false,
        text_input: String::new(),
        backspace_pressed: false,
        delete_pressed: false,
        enter_pressed: false,
        arrow_left_pressed: false,
        arrow_right_pressed: false,
        arrow_up_pressed: false,
        arrow_down_pressed: false,
        copy_pressed: false,
        paste_pressed: false,
        scroll_x: 0.0,
        scroll_y: 0.0,
    }
}

fn state_with_text_box(id: &str, text: &str) -> RetainedState {
    let mut state = RetainedState::new();
    state.upsert_text_box(UpsertTextBoxArgs {
        id: id.to_string(),
        rect: Rect::new(0.0, 0.0, 160.0, 44.0),
        text: Some(text.to_string()),
        placeholder: Some("p".to_string()),
        font_size: 16.0,
        text_color: Color::from_rgb8(240, 240, 240),
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: Color::from_rgb8(80, 80, 80),
        enabled: true,
        overflow_x: Overflow::Auto,
        overflow_y: Overflow::Auto,
    });
    state
}

#[test]
fn update_text_box_focus_resets_all_focus_flags() {
    let mut state = state_with_text_box("tb", "abc");
    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get_mut("tb") {
        tb.focused = true;
    }
    state.update_text_box_focus();
    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get("tb") {
        assert!(!tb.focused);
    } else {
        panic!("textbox missing");
    }
}

#[test]
fn apply_text_input_emits_event_on_text_change() {
    let mut state = state_with_text_box("tb", "abc");
    state.focused_text_box = Some("tb".to_string());
    let mut input = empty_input();
    input.text_input = "Z".to_string();
    state.apply_text_input(&input, None);

    let events = state.drain_events();
    assert!(events
            .iter()
            .any(|e| matches!(e, UiEvent::TextBoxChanged { text_box, text } if text_box.id() == "tb" && text.contains('Z'))));
}

#[test]
fn apply_text_box_scroll_updates_scroll_positions() {
    let mut state = state_with_text_box("tb", "very long text that should overflow width");
    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get_mut("tb") {
        tb.hovered = true;
        tb.text_advance = 1000.0;
        tb.overflow_x = Overflow::Auto;
        tb.overflow_y = Overflow::Auto;
    }

    let mut input = empty_input();
    input.scroll_y = 30.0;
    input.scroll_x = 5.0;
    state.apply_text_box_scroll(&input);

    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get("tb") {
        assert!(tb.scroll_x >= 0.0);
        assert!(tb.scroll_y >= 0.0);
    } else {
        panic!("textbox missing");
    }
}

#[test]
fn scrollbar_input_sets_active_and_drag_updates_scroll() {
    let mut state = state_with_text_box("tb", "very long text that should overflow width");
    if let Some(WidgetNode::TextBox(tb)) = state.widgets.get_mut("tb") {
        tb.text_advance = 1000.0;
        tb.overflow_x = Overflow::Auto;
    }

    state.handle_text_box_scrollbar_input(true, false, false, (20.0, 40.0));
    assert_eq!(state.active_text_box_scrollbar.as_deref(), Some("tb"));

    state.handle_text_box_scrollbar_input(false, true, false, (120.0, 40.0));
    state.handle_text_box_scrollbar_input(false, false, true, (120.0, 40.0));
    assert!(state.active_text_box_scrollbar.is_none());
}
