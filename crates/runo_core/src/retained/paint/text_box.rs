use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::TextBoxNode;
use crate::theme::color;
use crate::widget::text;

const BOX_CORNER_RADIUS: f64 = 8.0;
const BORDER_STROKE_WIDTH: f64 = 1.0;
const INNER_PADDING: f64 = 12.0;
const LINE_HEIGHT_RATIO: f64 = 1.35;
const CARET_X_OFFSET: f64 = 1.0;
const CARET_HEIGHT_RATIO: f64 = 1.1;
const CARET_TOP_OFFSET_RATIO: f64 = 0.9;
const CARET_WIDTH: f64 = 1.5;
const SCROLLBAR_TRACK_HEIGHT: f64 = 4.0;
const SCROLLBAR_TRACK_BOTTOM_OFFSET: f64 = 6.0;
const SCROLLBAR_CORNER_RADIUS: f64 = 2.0;
const SCROLLBAR_THUMB_MIN_WIDTH: f64 = 18.0;
const MIN_INNER_WIDTH: f64 = 1.0;

#[derive(Clone, Copy)]
struct TextMetrics {
    text_x: f64,
    first_line_baseline: f64,
    line_height: f64,
    inner_left: f64,
    inner_right: f64,
}

/// Renders text box background, border, text/placeholder, caret, and horizontal scrollbar.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, text_box: &mut TextBoxNode) {
    draw_background_and_border(scene, text_box);

    let Some(font) = font else {
        return;
    };

    let metrics = text_metrics(text_box);
    let text_color = resolve_text_color(text_box);
    draw_text_content(scene, font, text_box, text_color, metrics);
    draw_caret(scene, font, text_box, metrics);
    render_horizontal_scrollbar(scene, text_box);
}

/// Draws text box background fill and border stroke.
fn draw_background_and_border(scene: &mut Scene, text_box: &TextBoxNode) {
    let bg = RoundedRect::from_rect(text_box.rect, BOX_CORNER_RADIUS);
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if text_box.enabled {
            text_box.bg_color
        } else {
            color::rgb(color::widget::TEXT_BOX_DISABLED_BG)
        },
        None,
        &bg,
    );

    let border_color = if !text_box.enabled {
        color::rgb(color::widget::TEXT_BOX_DISABLED_BORDER)
    } else if text_box.focused {
        color::rgb(color::widget::TEXT_BOX_FOCUSED_BORDER)
    } else {
        text_box.border_color
    };
    scene.stroke(
        &vello::kurbo::Stroke::new(BORDER_STROKE_WIDTH),
        Affine::IDENTITY,
        border_color,
        None,
        &bg,
    );
}

/// Resolves text color for normal, placeholder, and disabled states.
fn resolve_text_color(text_box: &TextBoxNode) -> Color {
    if !text_box.enabled {
        color::rgb(color::widget::TEXT_BOX_DISABLED_TEXT)
    } else if text_box.text.is_empty() {
        color::rgb(color::widget::TEXT_BOX_PLACEHOLDER_TEXT)
    } else {
        text_box.text_color
    }
}

/// Computes shared text layout metrics used by content and caret painting.
fn text_metrics(text_box: &TextBoxNode) -> TextMetrics {
    TextMetrics {
        text_x: text_box.rect.x0 + INNER_PADDING - text_box.scroll_x,
        first_line_baseline: text_box.rect.y0 + INNER_PADDING + text_box.font_size as f64
            - text_box.scroll_y,
        line_height: text_box.font_size as f64 * LINE_HEIGHT_RATIO,
        inner_left: text_box.rect.x0 + INNER_PADDING,
        inner_right: text_box.rect.x1 - INNER_PADDING,
    }
}

/// Draws placeholder or text lines and updates cached text advance.
fn draw_text_content(
    scene: &mut Scene,
    font: &FontData,
    text_box: &mut TextBoxNode,
    text_color: Color,
    metrics: TextMetrics,
) {
    if text_box.text.is_empty() {
        let placeholder = text_box.placeholder.as_deref().unwrap_or("");
        if let Some((glyphs, advance)) = text::layout_text(font, placeholder, text_box.font_size) {
            let visible_glyphs = if text_box.overflow_x.clips() {
                clip_glyphs_horizontally(
                    glyphs,
                    advance as f64,
                    metrics.text_x,
                    metrics.inner_left,
                    metrics.inner_right,
                )
            } else {
                glyphs
            };
            if !visible_glyphs.is_empty() {
                text::draw_text_run(
                    scene,
                    font,
                    visible_glyphs,
                    metrics.text_x,
                    metrics.first_line_baseline,
                    text_box.font_size,
                    text_color,
                );
            }
        }
        text_box.text_advance = 0.0;
    } else {
        let mut max_advance = 0.0_f64;
        for (line_index, line_text) in text_box.text.split('\n').enumerate() {
            let baseline_y = metrics.first_line_baseline + line_index as f64 * metrics.line_height;
            let Some((glyphs, advance)) = text::layout_text(font, line_text, text_box.font_size)
            else {
                continue;
            };
            max_advance = max_advance.max(advance as f64);
            let visible_glyphs = if text_box.overflow_x.clips() {
                clip_glyphs_horizontally(
                    glyphs,
                    advance as f64,
                    metrics.text_x,
                    metrics.inner_left,
                    metrics.inner_right,
                )
            } else {
                glyphs
            };
            if !visible_glyphs.is_empty() {
                text::draw_text_run(
                    scene,
                    font,
                    visible_glyphs,
                    metrics.text_x,
                    baseline_y,
                    text_box.font_size,
                    text_color,
                );
            }
        }
        text_box.text_advance = max_advance;
    }
}

/// Draws caret when focused and enabled using the current text metrics.
fn draw_caret(scene: &mut Scene, font: &FontData, text_box: &TextBoxNode, metrics: TextMetrics) {
    if text_box.focused && text_box.enabled {
        let (caret_line, caret_col) =
            line_col_from_char_index(&text_box.text, text_box.caret_index);
        let caret_line_text = text_box.text.split('\n').nth(caret_line).unwrap_or("");
        let prefix: String = caret_line_text.chars().take(caret_col).collect();
        let prefix_advance = text::layout_text(font, &prefix, text_box.font_size)
            .map(|(_, advance)| advance as f64)
            .unwrap_or(0.0);
        let caret_x = metrics.text_x + prefix_advance + CARET_X_OFFSET;
        let caret_x = if text_box.overflow_x.clips() {
            caret_x.clamp(metrics.inner_left, metrics.inner_right)
        } else {
            caret_x
        };
        let baseline_y = metrics.first_line_baseline + caret_line as f64 * metrics.line_height;
        let caret_h = text_box.font_size as f64 * CARET_HEIGHT_RATIO;
        let caret_y0 = baseline_y - text_box.font_size as f64 * CARET_TOP_OFFSET_RATIO;
        let caret = Rect::new(caret_x, caret_y0, caret_x + CARET_WIDTH, caret_y0 + caret_h);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            color::rgb(color::widget::TEXT_BOX_CARET),
            None,
            &caret,
        );
    }
}

/// Returns glyphs that intersect the horizontal clip region in draw-space.
fn clip_glyphs_horizontally(
    glyphs: Vec<Glyph>,
    total_advance: f64,
    draw_origin_x: f64,
    clip_left: f64,
    clip_right: f64,
) -> Vec<Glyph> {
    if glyphs.is_empty() || clip_right <= clip_left {
        return Vec::new();
    }

    let mut out = Vec::new();
    for (index, glyph) in glyphs.iter().enumerate() {
        let x0 = draw_origin_x + glyph.x as f64;
        let next_x = if let Some(next) = glyphs.get(index + 1) {
            draw_origin_x + next.x as f64
        } else {
            draw_origin_x + total_advance
        };
        if next_x >= clip_left && x0 <= clip_right {
            out.push(*glyph);
        }
    }
    out
}

/// Renders the bottom horizontal scrollbar when overflow mode allows scrolling.
fn render_horizontal_scrollbar(scene: &mut Scene, text_box: &TextBoxNode) {
    if !text_box.overflow_x.allows_scroll() {
        return;
    }

    let inner_left = text_box.rect.x0 + INNER_PADDING;
    let inner_right = text_box.rect.x1 - INNER_PADDING;
    let inner_width = (inner_right - inner_left).max(MIN_INNER_WIDTH);
    let content_width = text_box_content_width(text_box);
    let max_scroll = (content_width - inner_width).max(0.0);
    if max_scroll <= 0.0 {
        return;
    }

    let track_height = SCROLLBAR_TRACK_HEIGHT;
    let track_y = text_box.rect.y1 - SCROLLBAR_TRACK_BOTTOM_OFFSET;
    let track = Rect::new(inner_left, track_y - track_height, inner_right, track_y);
    let track_shape = RoundedRect::from_rect(track, SCROLLBAR_CORNER_RADIUS);
    let track_color = if text_box.enabled {
        color::rgba(color::widget::TEXT_BOX_SCROLLBAR_TRACK_ENABLED)
    } else {
        color::rgba(color::widget::TEXT_BOX_SCROLLBAR_TRACK_DISABLED)
    };
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        track_color,
        None,
        &track_shape,
    );

    let thumb_w = ((inner_width / content_width) * inner_width)
        .clamp(SCROLLBAR_THUMB_MIN_WIDTH, inner_width)
        .min(inner_width);
    let ratio = (text_box.scroll_x / max_scroll).clamp(0.0, 1.0);
    let thumb_x0 = inner_left + ratio * (inner_width - thumb_w);
    let thumb = Rect::new(
        thumb_x0,
        track_y - track_height,
        thumb_x0 + thumb_w,
        track_y,
    );
    let thumb_shape = RoundedRect::from_rect(thumb, SCROLLBAR_CORNER_RADIUS);
    let thumb_color = if text_box.enabled {
        color::rgba(color::widget::TEXT_BOX_SCROLLBAR_THUMB_ENABLED)
    } else {
        color::rgba(color::widget::TEXT_BOX_SCROLLBAR_THUMB_DISABLED)
    };
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        thumb_color,
        None,
        &thumb_shape,
    );
}

/// Returns content width from cached layout advance or estimated text width fallback.
fn text_box_content_width(text_box: &TextBoxNode) -> f64 {
    if text_box.text_advance > 0.0 {
        text_box.text_advance
    } else {
        text::estimate_text_width(&text_box.text, text_box.font_size) as f64
    }
}

/// Converts a caret character index into zero-based `(line, column)` coordinates.
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
