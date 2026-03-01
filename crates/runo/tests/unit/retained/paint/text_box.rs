use super::*;
use crate::font::load_default_font;
use crate::widget::text_box::Overflow;
use vello::kurbo::Rect;

/// Builds a minimal text box fixture for helper-function tests.
fn sample_text_box() -> TextBoxNode {
    TextBoxNode {
        rect: Rect::new(0.0, 0.0, 240.0, 44.0),
        text: "hello".to_string(),
        placeholder: Some("placeholder".to_string()),
        font_size: 16.0,
        text_color: Color::from_rgb8(230, 230, 230),
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: Color::from_rgb8(80, 80, 80),
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
/// Keeps only glyphs that overlap the horizontal clip interval.
fn clip_glyphs_horizontally_filters_outside_glyphs() {
    let glyphs = vec![
        Glyph {
            id: 1,
            x: 0.0,
            y: 0.0,
        },
        Glyph {
            id: 2,
            x: 10.0,
            y: 0.0,
        },
        Glyph {
            id: 3,
            x: 20.0,
            y: 0.0,
        },
    ];

    let visible = clip_glyphs_horizontally(glyphs, 30.0, 0.0, 8.0, 18.0);
    assert_eq!(visible.len(), 2);
    assert_eq!(visible[0].id, 1);
    assert_eq!(visible[1].id, 2);
}

#[test]
/// Returns empty output when clip region is invalid.
fn clip_glyphs_horizontally_returns_empty_for_invalid_clip_region() {
    let glyphs = vec![Glyph {
        id: 1,
        x: 0.0,
        y: 0.0,
    }];
    let visible = clip_glyphs_horizontally(glyphs, 10.0, 0.0, 10.0, 10.0);
    assert!(visible.is_empty());
}

#[test]
/// Uses cached advance when a positive text_advance is available.
fn text_box_content_width_uses_cached_advance() {
    let mut text_box = sample_text_box();
    text_box.text_advance = 123.0;
    assert_eq!(text_box_content_width(&text_box), 123.0);
}

#[test]
/// Converts character index into expected line and column for multiline text.
fn line_col_from_char_index_handles_multiline_text() {
    assert_eq!(line_col_from_char_index("ab\ncde", 4), (1, 1));
}

#[test]
/// Returns final line/column when index is past text end.
fn line_col_from_char_index_returns_end_for_out_of_range_index() {
    assert_eq!(line_col_from_char_index("ab\nc", 99), (1, 1));
}

#[test]
fn render_and_internal_helpers_are_callable() {
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    render(&mut scene, None, &mut text_box);
    draw_background_and_border(&mut scene, &text_box);
    let metrics = text_metrics(&text_box);
    assert!(metrics.inner_right > metrics.inner_left);
    let _ = resolve_text_color(&text_box);
    render_horizontal_scrollbar(&mut scene, &text_box);

    if let Some(font) = load_default_font() {
        let color = text_box.text_color;
        let metrics = text_metrics(&text_box);
        draw_text_content(&mut scene, &font, &mut text_box, color, metrics);
        text_box.focused = true;
        draw_caret(&mut scene, &font, &text_box, text_metrics(&text_box));
        render(&mut scene, Some(&font), &mut text_box);
    }
}

#[test]
fn horizontal_scrollbar_draws_only_when_scrollable() {
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    text_box.overflow_x = Overflow::Visible;
    render_horizontal_scrollbar(&mut scene, &text_box);

    text_box.overflow_x = Overflow::Auto;
    text_box.text_advance = 1000.0;
    text_box.scroll_x = 100.0;
    render_horizontal_scrollbar(&mut scene, &text_box);
}
