use crate::retained::node::TextBoxNode;
use crate::theme::color;
use crate::widget::text;
use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::color::{self as vello_color, AlphaColor};
use vello::peniko::{Color, Fill, FontData};

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/text_box.rs"]
mod tests;

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
    inner_top: f64,
    inner_bottom: f64,
}

/// Renders text box background, border, text/placeholder, caret, and horizontal scrollbar.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, text_box: &mut TextBoxNode) {
    draw_background_and_border(scene, text_box);

    let Some(font) = font else {
        return;
    };

    let metrics: TextMetrics = text_metrics(text_box);
    let text_color: AlphaColor<vello_color::Srgb> = resolve_text_color(text_box);

    draw_text_content(scene, font, text_box, text_color, metrics);
    draw_caret(scene, font, text_box, metrics);
    render_horizontal_scrollbar(scene, text_box);
}

/// Draws text box background fill and border stroke.
fn draw_background_and_border(scene: &mut Scene, text_box: &TextBoxNode) {
    let bg: RoundedRect = RoundedRect::from_rect(text_box.rect, BOX_CORNER_RADIUS);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if text_box.enabled {
            text_box.bg_color
        } else {
            color::Neutral::tone_45_49_55()
        },
        None,
        &bg,
    );

    let border_color: Color = if !text_box.enabled {
        color::Neutral::tone_86_92_101()
    } else if text_box.focused && !text_box.read_only {
        color::AccentBlue::tone_89_176_255()
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
        color::Neutral::tone_147_153_161()
    } else if text_box.text.is_empty() {
        color::Neutral::tone_142_151_163()
    } else {
        text_box.text_color
    }
}

/// Computes shared text layout metrics used by content and caret painting.
fn text_metrics(text_box: &TextBoxNode) -> TextMetrics {
    TextMetrics {
        text_x: text_box.rect.x0 + INNER_PADDING - text_box.scroll_x,
        first_line_baseline: text_box.rect.y0 + INNER_PADDING + f64::from(text_box.font_size)
            - text_box.scroll_y,
        line_height: f64::from(text_box.font_size) * LINE_HEIGHT_RATIO,
        inner_left: text_box.rect.x0 + INNER_PADDING,
        inner_right: text_box.rect.x1 - INNER_PADDING,
        inner_top: text_box.rect.y0 + INNER_PADDING,
        inner_bottom: text_box.rect.y1 - INNER_PADDING,
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
        let placeholder: &str = text_box.placeholder.as_deref().unwrap_or("");
        if text_box.overflow_y.clips()
            && !line_intersects_vertical_clip(
                metrics.first_line_baseline,
                f64::from(text_box.font_size),
                metrics.inner_top,
                metrics.inner_bottom,
            )
        {
            text_box.text_advance = 0.0;
            return;
        }

        if let Some((glyphs, advance)) = text::layout_text(font, placeholder, text_box.font_size) {
            let visible_glyphs: Vec<Glyph> = if text_box.overflow_x.clips() {
                clip_glyphs_horizontally(
                    glyphs,
                    f64::from(advance),
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
        let mut max_advance: f64 = 0.0_f64;
        for (line_index, line_text) in text_box.text.split('\n').enumerate() {
            let baseline_y: f64 =
                metrics.first_line_baseline + line_index as f64 * metrics.line_height;
            if text_box.overflow_y.clips()
                && !line_intersects_vertical_clip(
                    baseline_y,
                    f64::from(text_box.font_size),
                    metrics.inner_top,
                    metrics.inner_bottom,
                )
            {
                continue;
            }
            let Some((glyphs, advance)) = text::layout_text(font, line_text, text_box.font_size)
            else {
                continue;
            };

            max_advance = max_advance.max(f64::from(advance));

            let visible_glyphs: Vec<Glyph> = if text_box.overflow_x.clips() {
                clip_glyphs_horizontally(
                    glyphs,
                    f64::from(advance),
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
    if text_box.focused && text_box.enabled && !text_box.read_only {
        let (caret_line, caret_col) =
            line_col_from_char_index(&text_box.text, text_box.caret_index);
        let caret_line_text: &str = text_box.text.split('\n').nth(caret_line).unwrap_or("");
        let prefix: String = caret_line_text.chars().take(caret_col).collect();
        let prefix_advance: f64 = text::layout_text(font, &prefix, text_box.font_size)
            .map_or(0.0, |(_, advance)| f64::from(advance));
        let caret_x: f64 = metrics.text_x + prefix_advance + CARET_X_OFFSET;
        let caret_x: f64 = if text_box.overflow_x.clips() {
            caret_x.clamp(metrics.inner_left, metrics.inner_right)
        } else {
            caret_x
        };
        let baseline_y: f64 = metrics.first_line_baseline + caret_line as f64 * metrics.line_height;
        if text_box.overflow_y.clips()
            && !line_intersects_vertical_clip(
                baseline_y,
                f64::from(text_box.font_size),
                metrics.inner_top,
                metrics.inner_bottom,
            )
        {
            return;
        }
        let caret_h: f64 = f64::from(text_box.font_size) * CARET_HEIGHT_RATIO;
        let caret_y0: f64 = baseline_y - f64::from(text_box.font_size) * CARET_TOP_OFFSET_RATIO;
        let caret: Rect = Rect::new(caret_x, caret_y0, caret_x + CARET_WIDTH, caret_y0 + caret_h);

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            color::SoftWhite::tone_220_228_240(),
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

    let mut out: Vec<Glyph> = Vec::new();
    for (index, glyph) in glyphs.iter().enumerate() {
        let x0: f64 = draw_origin_x + f64::from(glyph.x);
        let next_x: f64 = if let Some(next) = glyphs.get(index + 1) {
            draw_origin_x + f64::from(next.x)
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

    let inner_left: f64 = text_box.rect.x0 + INNER_PADDING;
    let inner_right: f64 = text_box.rect.x1 - INNER_PADDING;
    let inner_width: f64 = (inner_right - inner_left).max(MIN_INNER_WIDTH);
    let content_width: f64 = text_box_content_width(text_box);
    let max_scroll: f64 = (content_width - inner_width).max(0.0);
    if max_scroll <= 0.0 {
        return;
    }

    let track_height: f64 = SCROLLBAR_TRACK_HEIGHT;
    let track_y: f64 = text_box.rect.y1 - SCROLLBAR_TRACK_BOTTOM_OFFSET;
    let track: Rect = Rect::new(inner_left, track_y - track_height, inner_right, track_y);
    let track_shape: RoundedRect = RoundedRect::from_rect(track, SCROLLBAR_CORNER_RADIUS);
    let track_color: Color = if text_box.enabled {
        color::WhiteAlpha::tone_255_255_255_35()
    } else {
        color::WhiteAlpha::tone_255_255_255_20()
    };

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        track_color,
        None,
        &track_shape,
    );

    let thumb_w: f64 = ((inner_width / content_width) * inner_width)
        .clamp(SCROLLBAR_THUMB_MIN_WIDTH, inner_width)
        .min(inner_width);
    let ratio: f64 = (text_box.scroll_x / max_scroll).clamp(0.0, 1.0);
    let thumb_x0: f64 = inner_left + ratio * (inner_width - thumb_w);
    let thumb: Rect = Rect::new(
        thumb_x0,
        track_y - track_height,
        thumb_x0 + thumb_w,
        track_y,
    );
    let thumb_shape: RoundedRect = RoundedRect::from_rect(thumb, SCROLLBAR_CORNER_RADIUS);
    let thumb_color: Color = if text_box.enabled {
        color::WhiteAlpha::tone_255_255_255_150()
    } else {
        color::WhiteAlpha::tone_255_255_255_90()
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
        f64::from(text::estimate_text_width(
            &text_box.text,
            text_box.font_size,
        ))
    }
}

/// Converts a caret character index into zero-based `(line, column)` coordinates.
fn line_col_from_char_index(text: &str, caret_index: usize) -> (usize, usize) {
    let mut line: usize = 0;
    let mut col: usize = 0;
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

fn line_intersects_vertical_clip(
    baseline_y: f64,
    font_size: f64,
    clip_top: f64,
    clip_bottom: f64,
) -> bool {
    let line_top: f64 = baseline_y - font_size;
    let line_bottom: f64 = baseline_y + font_size * (LINE_HEIGHT_RATIO - 1.0);
    line_bottom >= clip_top && line_top <= clip_bottom
}
