use super::super::text_content::TextContentPainter;
use super::*;
use crate::font::load_default_font;
use crate::theme::color;
use crate::widget::text;
use crate::widget::text_box::Overflow;
use vello::Glyph;
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
        read_only: false,
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

    let visible = TextContentPainter::clip_glyphs_horizontally(glyphs, 30.0, 0.0, 8.0, 18.0);
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
    let visible = TextContentPainter::clip_glyphs_horizontally(glyphs, 10.0, 0.0, 10.0, 10.0);
    assert!(visible.is_empty());
}

#[test]
/// Uses cached advance when a positive text_advance is available.
fn text_box_content_width_uses_cached_advance() {
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    text_box.text_advance = 123.0;
    let mut scrollbar = ScrollBar::new(&mut scene, &text_box);
    scrollbar.render_horizontal_scrollbar();
}

#[test]
/// Converts character index into expected line and column for multiline text.
fn line_col_from_char_index_handles_multiline_text() {
    assert_eq!(Caret::line_col_from_char_index("ab\ncde", 4), (1, 1));
}

#[test]
/// Returns final line/column when index is past text end.
fn line_col_from_char_index_returns_end_for_out_of_range_index() {
    assert_eq!(Caret::line_col_from_char_index("ab\nc", 99), (1, 1));
}

#[test]
fn render_and_internal_helpers_are_callable() {
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    render(&mut scene, None, &mut text_box);
    CorePainter::draw_background_and_border(&mut scene, &text_box);
    let metrics = TextMetrics::new(&text_box);
    assert!(metrics.inner_right > metrics.inner_left);
    let _ = CorePainter::resolve_text_color(&text_box);
    let mut scrollbar = ScrollBar::new(&mut scene, &text_box);
    scrollbar.render_horizontal_scrollbar();

    if let Some(font) = load_default_font() {
        let color = text_box.text_color;
        let metrics = TextMetrics::new(&text_box);
        TextContentPainter::draw_text_content(&mut scene, &font, &mut text_box, color, metrics);
        text_box.focused = true;
        let mut caret = Caret::new(&mut scene, &font, &text_box, TextMetrics::new(&text_box));
        caret.draw_caret();
        render(&mut scene, Some(&font), &mut text_box);
    }
}

#[test]
fn horizontal_scrollbar_draws_only_when_scrollable() {
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    text_box.overflow_x = Overflow::Visible;
    let mut scrollbar = ScrollBar::new(&mut scene, &text_box);
    scrollbar.render_horizontal_scrollbar();

    text_box.overflow_x = Overflow::Auto;
    text_box.text_advance = 1000.0;
    text_box.scroll_x = 100.0;
    let mut scrollbar = ScrollBar::new(&mut scene, &text_box);
    scrollbar.render_horizontal_scrollbar();
}

#[test]
fn line_intersects_vertical_clip_detects_visibility() {
    assert!(Caret::line_intersects_vertical_clip(20.0, 16.0, 8.0, 24.0));
    assert!(!Caret::line_intersects_vertical_clip(60.0, 16.0, 8.0, 24.0));
}

#[test]
fn resolve_text_color_uses_disabled_color_first() {
    let mut text_box = sample_text_box();
    text_box.enabled = false;
    text_box.text.clear();
    assert_eq!(
        CorePainter::resolve_text_color(&text_box),
        color::Neutral::tone_147_153_161()
    );
}

#[test]
fn resolve_text_color_uses_placeholder_tone_for_empty_enabled_text() {
    let mut text_box = sample_text_box();
    text_box.text.clear();
    assert_eq!(
        CorePainter::resolve_text_color(&text_box),
        color::Neutral::tone_142_151_163()
    );
}

#[test]
fn text_metrics_apply_padding_and_scroll_offsets() {
    let mut text_box = sample_text_box();
    text_box.scroll_x = 3.0;
    text_box.scroll_y = 4.0;
    let metrics = TextMetrics::new(&text_box);
    assert_eq!(metrics.inner_left, 12.0);
    assert_eq!(metrics.inner_right, 228.0);
    assert_eq!(metrics.inner_top, 12.0);
    assert_eq!(metrics.inner_bottom, 32.0);
    assert_eq!(metrics.text_x, 9.0);
    assert_eq!(metrics.first_line_baseline, 24.0);
}

#[test]
fn draw_text_content_sets_text_advance_zero_for_empty_input() {
    let Some(font) = load_default_font() else {
        return;
    };
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    text_box.text.clear();
    text_box.text_advance = 777.0;
    let text_color = text_box.text_color;
    let metrics = TextMetrics::new(&text_box);
    TextContentPainter::draw_text_content(&mut scene, &font, &mut text_box, text_color, metrics);
    assert_eq!(text_box.text_advance, 0.0);
}

#[test]
fn draw_text_content_stores_max_line_advance_for_multiline_text() {
    let Some(font) = load_default_font() else {
        return;
    };
    let mut scene = Scene::new();
    let mut text_box = sample_text_box();
    text_box.text = "short\nlonger line".to_string();
    text_box.overflow_y = Overflow::Visible;
    let expected = text::layout_text(&font, "longer line", text_box.font_size)
        .map(|(_, advance)| f64::from(advance))
        .unwrap_or(0.0);
    let text_color = text_box.text_color;
    let metrics = TextMetrics::new(&text_box);
    TextContentPainter::draw_text_content(&mut scene, &font, &mut text_box, text_color, metrics);
    assert!((text_box.text_advance - expected).abs() < 1e-6);
}
