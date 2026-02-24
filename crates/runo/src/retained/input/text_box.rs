use vello::kurbo::Rect;
use vello::peniko::FontData;

use super::pointer::contains;
use crate::event::UiEvent;
use crate::input::InputFrame;
use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;
use crate::widget::text::{estimate_text_width, layout_text};

impl RetainedState {
    pub(super) fn handle_text_box_scrollbar_input(
        &mut self,
        mouse_pressed: bool,
        mouse_down: bool,
        mouse_released: bool,
        cursor_pos: (f64, f64),
    ) {
        if mouse_pressed {
            let scrollbar_id = self.order.iter().rev().find_map(|id| {
                let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                    return None;
                };
                if text_box.enabled
                    && text_box.overflow_x.allows_scroll()
                    && text_box_max_scroll_x(text_box) > 0.0
                    && text_box_scrollbar_track_contains(text_box, cursor_pos.0, cursor_pos.1)
                {
                    Some(id.clone())
                } else {
                    None
                }
            });
            self.active_text_box_scrollbar = scrollbar_id;
        }

        if mouse_down
            && let Some(id) = self.active_text_box_scrollbar.clone()
            && let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&id)
            && text_box.enabled
            && text_box.overflow_x.allows_scroll()
            && text_box_max_scroll_x(text_box) > 0.0
        {
            set_scroll_from_scrollbar_cursor(text_box, cursor_pos.0);
        }

        if mouse_released {
            self.active_text_box_scrollbar = None;
        }
    }

    pub(super) fn update_text_box_focus(&mut self) {
        for node in self.widgets.values_mut() {
            if let WidgetNode::TextBox(text_box) = node {
                text_box.focused = false;
            }
        }
    }

    pub(super) fn apply_text_input(&mut self, input: &InputFrame, font: Option<&FontData>) {
        let mut pending_event: Option<UiEvent> = None;
        if let Some(id) = self.focused_text_box.clone()
            && let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&id)
            && text_box.enabled
        {
            text_box.focused = true;

            if input.copy_pressed {
                self.text_clipboard = text_box.text.clone();
                write_system_clipboard(&self.text_clipboard);
            }

            if input.arrow_left_pressed {
                text_box.caret_index = text_box.caret_index.saturating_sub(1);
            }
            if input.arrow_right_pressed {
                let max = text_box.text.chars().count();
                text_box.caret_index = (text_box.caret_index + 1).min(max);
            }
            if input.arrow_up_pressed {
                text_box.caret_index =
                    move_caret_vertical(&text_box.text, text_box.caret_index, -1);
            }
            if input.arrow_down_pressed {
                text_box.caret_index = move_caret_vertical(&text_box.text, text_box.caret_index, 1);
            }

            if input.backspace_pressed
                && remove_char_before_caret(&mut text_box.text, &mut text_box.caret_index)
            {
                text_box.changed = true;
            }

            if input.delete_pressed
                && remove_char_at_caret(&mut text_box.text, text_box.caret_index)
            {
                text_box.changed = true;
            }

            if input.enter_pressed {
                insert_text_at_caret(&mut text_box.text, &mut text_box.caret_index, "\n");
                text_box.changed = true;
            }

            if input.paste_pressed {
                let pasted = read_system_clipboard().unwrap_or_else(|| self.text_clipboard.clone());
                if !pasted.is_empty() {
                    insert_text_at_caret(&mut text_box.text, &mut text_box.caret_index, &pasted);
                    text_box.changed = true;
                }
            }

            if !input.text_input.is_empty() {
                let sanitized: String = input
                    .text_input
                    .chars()
                    .filter(|ch| !ch.is_control())
                    .collect();
                if !sanitized.is_empty() {
                    insert_text_at_caret(&mut text_box.text, &mut text_box.caret_index, &sanitized);
                    text_box.changed = true;
                }
            }

            if text_box.changed {
                sync_text_box_text_advance(text_box, font);
                Self::keep_text_box_end_visible(text_box);
                pending_event = Some(UiEvent::TextBoxChanged {
                    text_box: crate::widget::text_box::TextBoxHandle::new(id),
                    text: text_box.text.clone(),
                });
            }
        }

        if let Some(event) = pending_event {
            self.push_event(event);
        }
    }

    pub(super) fn apply_text_box_scroll(&mut self, input: &InputFrame) {
        if input.scroll_x == 0.0 && input.scroll_y == 0.0 {
            return;
        }

        let target_id = self.order.iter().rev().find_map(|id| {
            let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                return None;
            };
            if text_box.enabled && (text_box.hovered || text_box.focused) {
                Some(id.clone())
            } else {
                None
            }
        });

        let Some(target_id) = target_id else {
            return;
        };
        let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&target_id) else {
            return;
        };

        if text_box.overflow_x.allows_scroll() {
            if self.active_text_box_scrollbar.as_deref() != Some(target_id.as_str()) {
                let wheel_x = if input.scroll_x.abs() > input.scroll_y.abs() * 0.5 {
                    -input.scroll_x
                } else {
                    input.scroll_y
                };
                text_box.scroll_x =
                    (text_box.scroll_x + wheel_x).clamp(0.0, Self::max_scroll_x(text_box));
            } else {
                text_box.scroll_x = text_box.scroll_x.clamp(0.0, Self::max_scroll_x(text_box));
            }
        } else {
            text_box.scroll_x = 0.0;
        }

        if text_box.overflow_y.allows_scroll() {
            text_box.scroll_y =
                (text_box.scroll_y - input.scroll_y).clamp(0.0, Self::max_scroll_y(text_box));
        } else {
            text_box.scroll_y = 0.0;
        }
    }

    fn keep_text_box_end_visible(text_box: &mut crate::retained::node::TextBoxNode) {
        if !matches!(text_box.overflow_x, crate::widget::text_box::Overflow::Auto) {
            text_box.scroll_x = text_box.scroll_x.clamp(0.0, Self::max_scroll_x(text_box));
            return;
        }
        text_box.scroll_x = Self::max_scroll_x(text_box);
    }

    fn max_scroll_x(text_box: &crate::retained::node::TextBoxNode) -> f64 {
        text_box_max_scroll_x(text_box)
    }

    fn max_scroll_y(text_box: &crate::retained::node::TextBoxNode) -> f64 {
        let line_count = text_box.text.split('\n').count().max(1) as f64;
        let content_height = line_count * (text_box.font_size as f64 * 1.35) + 12.0;
        let inner_height = (text_box.rect.height() - 12.0).max(1.0);
        (content_height - inner_height).max(0.0)
    }
}

fn text_box_max_scroll_x(text_box: &crate::retained::node::TextBoxNode) -> f64 {
    let inner_width = (text_box.rect.width() - 24.0).max(1.0);
    let content_width = text_box_content_width(text_box);
    (content_width - inner_width).max(0.0)
}

fn text_box_scrollbar_track_contains(
    text_box: &crate::retained::node::TextBoxNode,
    x: f64,
    y: f64,
) -> bool {
    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;
    let hit_height = 12.0;
    let hit_bottom = text_box.rect.y1 - 2.0;
    let hit_top = (hit_bottom - hit_height).max(text_box.rect.y0);
    let hit = Rect::new(inner_left, hit_top, inner_right, hit_bottom);
    contains(hit, x, y)
}

fn set_scroll_from_scrollbar_cursor(
    text_box: &mut crate::retained::node::TextBoxNode,
    cursor_x: f64,
) {
    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;
    let inner_width = (inner_right - inner_left).max(1.0);
    let content_width = text_box_content_width(text_box);
    let max_scroll = text_box_max_scroll_x(text_box);
    if max_scroll <= 0.0 {
        text_box.scroll_x = 0.0;
        return;
    }

    let thumb_w = ((inner_width / content_width) * inner_width)
        .clamp(18.0, inner_width)
        .min(inner_width);
    let den = (inner_width - thumb_w).max(1.0);
    let ratio = ((cursor_x - inner_left - thumb_w * 0.5) / den).clamp(0.0, 1.0);
    text_box.scroll_x = ratio * max_scroll;
}

fn text_box_content_width(text_box: &crate::retained::node::TextBoxNode) -> f64 {
    if text_box.text_advance > 0.0 {
        text_box.text_advance
    } else {
        estimate_text_width(&text_box.text, text_box.font_size) as f64
    }
}

fn sync_text_box_text_advance(
    text_box: &mut crate::retained::node::TextBoxNode,
    font: Option<&FontData>,
) {
    if let Some(font) = font
        && let Some((_, advance)) = layout_text(font, &text_box.text, text_box.font_size)
    {
        text_box.text_advance = advance as f64;
        return;
    }
    text_box.text_advance = estimate_text_width(&text_box.text, text_box.font_size) as f64;
}

fn write_system_clipboard(text: &str) {
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text.to_string());
    }
}

fn read_system_clipboard() -> Option<String> {
    let mut clipboard = arboard::Clipboard::new().ok()?;
    clipboard.get_text().ok()
}

fn char_to_byte_index(s: &str, char_index: usize) -> usize {
    s.char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(s.len())
}

fn insert_text_at_caret(text: &mut String, caret_index: &mut usize, insert: &str) {
    let byte = char_to_byte_index(text, *caret_index);
    text.insert_str(byte, insert);
    *caret_index += insert.chars().count();
}

fn remove_char_before_caret(text: &mut String, caret_index: &mut usize) -> bool {
    if *caret_index == 0 {
        return false;
    }
    let remove_char = *caret_index - 1;
    let start = char_to_byte_index(text, remove_char);
    let end = char_to_byte_index(text, *caret_index);
    text.replace_range(start..end, "");
    *caret_index -= 1;
    true
}

fn remove_char_at_caret(text: &mut String, caret_index: usize) -> bool {
    let total = text.chars().count();
    if caret_index >= total {
        return false;
    }
    let start = char_to_byte_index(text, caret_index);
    let end = char_to_byte_index(text, caret_index + 1);
    text.replace_range(start..end, "");
    true
}

fn move_caret_vertical(text: &str, caret_index: usize, delta_line: i32) -> usize {
    let (line, col) = line_col_from_char_index(text, caret_index);
    let lines: Vec<&str> = text.split('\n').collect();
    if lines.is_empty() {
        return 0;
    }
    let target_line = (line as i32 + delta_line).clamp(0, lines.len() as i32 - 1) as usize;
    let target_col = col.min(lines[target_line].chars().count());
    char_index_from_line_col(&lines, target_line, target_col)
}

fn line_col_from_char_index(text: &str, caret_index: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in text.chars().enumerate() {
        if i == caret_index {
            return (line, col);
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn char_index_from_line_col(lines: &[&str], line: usize, col: usize) -> usize {
    let mut index = 0;
    for (i, line_text) in lines.iter().enumerate() {
        if i == line {
            return index + col.min(line_text.chars().count());
        }
        index += line_text.chars().count();
        if i + 1 < lines.len() {
            index += 1;
        }
    }
    index
}

#[cfg(test)]
mod tests {
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
}
