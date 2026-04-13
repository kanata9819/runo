use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::input::InputFrame;
use crate::retained::node::WidgetNode;
use crate::retained::state::{RetainedState, UpsertTerminalViewArgs};
use crate::widget::terminal_view::{TerminalBuffer, TerminalCell};

fn rect() -> Rect {
    Rect::new(0.0, 0.0, 260.0, 96.0)
}

fn buffer_with_lines(line_count: usize) -> TerminalBuffer {
    TerminalBuffer {
        lines: (0..line_count)
            .map(|_| vec![TerminalCell::new('x')])
            .collect(),
        cursor_row: line_count.saturating_sub(1),
        cursor_col: 1,
    }
}

#[test]
fn upsert_terminal_view_updates_buffer_and_preserves_hover_state() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);

    state.upsert_terminal_view(UpsertTerminalViewArgs {
        id: "tv".to_string(),
        rect: rect(),
        buffer: buffer_with_lines(2),
        font_size: 16.0,
        text_color: color,
        bg_color: Color::from_rgb8(20, 20, 20),
        border_color: color,
        disable_border: false,
        enabled: true,
    });

    if let Some(WidgetNode::TerminalView(tv)) = state.widgets.get_mut("tv") {
        tv.hovered = true;
    }

    state.upsert_terminal_view(UpsertTerminalViewArgs {
        id: "tv".to_string(),
        rect: rect(),
        buffer: buffer_with_lines(5),
        font_size: 18.0,
        text_color: color,
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: color,
        disable_border: true,
        enabled: true,
    });

    let response = state.terminal_view_response("tv");
    assert!(response.hovered);

    match state.widgets.get("tv") {
        Some(WidgetNode::TerminalView(tv)) => {
            assert_eq!(tv.buffer.lines.len(), 5);
            assert!((tv.font_size - 18.0).abs() < f32::EPSILON);
            assert!(tv.disable_border);
        }
        _ => panic!("terminal view missing"),
    }
}

#[test]
fn set_terminal_view_enabled_false_clears_hover() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);

    state.upsert_terminal_view(UpsertTerminalViewArgs {
        id: "tv".to_string(),
        rect: rect(),
        buffer: buffer_with_lines(1),
        font_size: 16.0,
        text_color: color,
        bg_color: Color::from_rgb8(20, 20, 20),
        border_color: color,
        disable_border: false,
        enabled: true,
    });

    if let Some(WidgetNode::TerminalView(tv)) = state.widgets.get_mut("tv") {
        tv.hovered = true;
    }

    state.set_terminal_view_enabled("tv", false);

    match state.widgets.get("tv") {
        Some(WidgetNode::TerminalView(tv)) => {
            assert!(!tv.enabled);
            assert!(!tv.hovered);
        }
        _ => panic!("terminal view missing"),
    }
}

#[test]
fn apply_terminal_view_scroll_moves_scroll_position() {
    let mut state = RetainedState::new();
    let color = Color::from_rgb8(240, 240, 240);

    state.upsert_terminal_view(UpsertTerminalViewArgs {
        id: "tv".to_string(),
        rect: rect(),
        buffer: buffer_with_lines(20),
        font_size: 16.0,
        text_color: color,
        bg_color: Color::from_rgb8(20, 20, 20),
        border_color: color,
        disable_border: false,
        enabled: true,
    });

    if let Some(WidgetNode::TerminalView(tv)) = state.widgets.get_mut("tv") {
        tv.hovered = true;
        tv.scroll_y = 40.0;
    }

    state.apply_terminal_view_scroll(&InputFrame {
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
        scroll_y: -2.0,
    });

    match state.widgets.get("tv") {
        Some(WidgetNode::TerminalView(tv)) => assert!(tv.scroll_y > 40.0),
        _ => panic!("terminal view missing"),
    }
}
