//! Core painter for `TextBox`.
//!
//! This module renders background, border, text/placeholder, caret,
//! and horizontal scrollbar with separated responsibilities.

use super::caret::Caret;
use super::metrics::TextMetrics;
use super::scrollbar::ScrollBar;
use crate::retained::node::TextBoxNode;
use crate::theme::color;
use crate::widget::text;
use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::color::{self as vello_color, AlphaColor};
use vello::peniko::{Color, Fill, FontData};

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/text_box.rs"]
mod tests;

/// Entry point for rendering the full `TextBox`.
///
/// Draws background and border first, then renders text/placeholder,
/// caret, and horizontal scrollbar only when a font is available.
pub(crate) fn render(scene: &mut Scene, font: Option<&FontData>, text_box: &mut TextBoxNode) {
    draw_background_and_border(scene, text_box);

    let Some(font) = font else {
        return;
    };

    let metrics: TextMetrics = TextMetrics::new(text_box);
    let text_color: AlphaColor<vello_color::Srgb> = resolve_text_color(text_box);
    draw_text_content(scene, font, text_box, text_color, metrics);

    let mut caret: Caret = Caret::new(scene, font, text_box, metrics);
    caret.draw_caret();

    let mut scrollbar: ScrollBar = ScrollBar::new(scene, text_box);
    scrollbar.render_horizontal_scrollbar();
}

/// Draws `TextBox` background fill and border stroke.
///
/// Colors are chosen based on `enabled`, `focused`, and `read_only` states.
fn draw_background_and_border(scene: &mut Scene, text_box: &TextBoxNode) {
    let bg: RoundedRect = RoundedRect::from_rect(text_box.rect, TextMetrics::BOX_CORNER_RADIUS);

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
        &vello::kurbo::Stroke::new(TextMetrics::BORDER_STROKE_WIDTH),
        Affine::IDENTITY,
        border_color,
        None,
        &bg,
    );
}

/// Resolves text color by state.
///
/// Returns different colors for disabled, placeholder, and normal text.
fn resolve_text_color(text_box: &TextBoxNode) -> Color {
    if !text_box.enabled {
        color::Neutral::tone_147_153_161()
    } else if text_box.text.is_empty() {
        color::Neutral::tone_142_151_163()
    } else {
        text_box.text_color
    }
}

/// Draws text content or placeholder and updates cached text advance.
///
/// Lays out multi-line text per line and applies vertical/horizontal clipping
/// when required.
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
            && !Caret::line_intersects_vertical_clip(
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
                && !Caret::line_intersects_vertical_clip(
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

/// Returns only glyphs that intersect the horizontal clip region.
///
/// Visibility is computed from each glyph start and the next glyph position
/// (or `total_advance` for the last glyph).
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
