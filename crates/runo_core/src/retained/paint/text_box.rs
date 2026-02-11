use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::TextBoxNode;
use crate::widget::text::{draw_text_run, estimate_text_width, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, text_box: &mut TextBoxNode) {
    let bg = RoundedRect::from_rect(text_box.rect, 8.0);
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if text_box.enabled {
            text_box.bg_color
        } else {
            Color::from_rgb8(45, 49, 55)
        },
        None,
        &bg,
    );

    let border_color = if !text_box.enabled {
        Color::from_rgb8(86, 92, 101)
    } else if text_box.focused {
        Color::from_rgb8(89, 176, 255)
    } else {
        text_box.border_color
    };
    scene.stroke(
        &vello::kurbo::Stroke::new(1.0),
        Affine::IDENTITY,
        border_color,
        None,
        &bg,
    );

    let Some(font) = font else {
        return;
    };

    let text_color = if !text_box.enabled {
        Color::from_rgb8(147, 153, 161)
    } else if text_box.text.is_empty() {
        Color::from_rgb8(142, 151, 163)
    } else {
        text_box.text_color
    };

    let text_x = text_box.rect.x0 + 12.0 - text_box.scroll_x;
    let first_line_baseline =
        text_box.rect.y0 + 12.0 + text_box.font_size as f64 - text_box.scroll_y;
    let line_height = text_box.font_size as f64 * 1.35;
    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;

    if text_box.text.is_empty() {
        let placeholder = text_box.placeholder.as_deref().unwrap_or("");
        if let Some((glyphs, advance)) = layout_text(font, placeholder, text_box.font_size) {
            let visible_glyphs = if text_box.overflow_x.clips() {
                clip_glyphs_horizontally(glyphs, advance as f64, text_x, inner_left, inner_right)
            } else {
                glyphs
            };
            if !visible_glyphs.is_empty() {
                draw_text_run(
                    scene,
                    font,
                    visible_glyphs,
                    text_x,
                    first_line_baseline,
                    text_box.font_size,
                    text_color,
                );
            }
        }
        text_box.text_advance = 0.0;
    } else {
        let mut max_advance = 0.0_f64;
        for (line_index, line_text) in text_box.text.split('\n').enumerate() {
            let baseline_y = first_line_baseline + line_index as f64 * line_height;
            let Some((glyphs, advance)) = layout_text(font, line_text, text_box.font_size) else {
                continue;
            };
            max_advance = max_advance.max(advance as f64);
            let visible_glyphs = if text_box.overflow_x.clips() {
                clip_glyphs_horizontally(glyphs, advance as f64, text_x, inner_left, inner_right)
            } else {
                glyphs
            };
            if !visible_glyphs.is_empty() {
                draw_text_run(
                    scene,
                    font,
                    visible_glyphs,
                    text_x,
                    baseline_y,
                    text_box.font_size,
                    text_color,
                );
            }
        }
        text_box.text_advance = max_advance;
    }

    if text_box.focused && text_box.enabled {
        let (caret_line, caret_col) =
            line_col_from_char_index(&text_box.text, text_box.caret_index);
        let caret_line_text = text_box.text.split('\n').nth(caret_line).unwrap_or("");
        let prefix: String = caret_line_text.chars().take(caret_col).collect();
        let prefix_advance = layout_text(font, &prefix, text_box.font_size)
            .map(|(_, advance)| advance as f64)
            .unwrap_or(0.0);
        let caret_x = text_x + prefix_advance + 1.0;
        let caret_x = if text_box.overflow_x.clips() {
            caret_x.clamp(inner_left, inner_right)
        } else {
            caret_x
        };
        let baseline_y = first_line_baseline + caret_line as f64 * line_height;
        let caret_h = text_box.font_size as f64 * 1.1;
        let caret_y0 = baseline_y - text_box.font_size as f64 * 0.9;
        let caret = Rect::new(caret_x, caret_y0, caret_x + 1.5, caret_y0 + caret_h);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgb8(220, 228, 240),
            None,
            &caret,
        );
    }

    render_horizontal_scrollbar(scene, text_box);
}

fn clip_glyphs_horizontally(
    glyphs: Vec<vello::Glyph>,
    total_advance: f64,
    draw_origin_x: f64,
    clip_left: f64,
    clip_right: f64,
) -> Vec<vello::Glyph> {
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
            out.push(glyph.clone());
        }
    }
    out
}

fn render_horizontal_scrollbar(scene: &mut Scene, text_box: &TextBoxNode) {
    if !text_box.overflow_x.allows_scroll() {
        return;
    }

    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;
    let inner_width = (inner_right - inner_left).max(1.0);
    let content_width = text_box_content_width(text_box);
    let max_scroll = (content_width - inner_width).max(0.0);
    if max_scroll <= 0.0 {
        return;
    }

    let track_height = 4.0;
    let track_y = text_box.rect.y1 - 6.0;
    let track = Rect::new(inner_left, track_y - track_height, inner_right, track_y);
    let track_shape = RoundedRect::from_rect(track, 2.0);
    let track_color = if text_box.enabled {
        Color::from_rgba8(255, 255, 255, 35)
    } else {
        Color::from_rgba8(255, 255, 255, 20)
    };
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        track_color,
        None,
        &track_shape,
    );

    let thumb_w = ((inner_width / content_width) * inner_width)
        .clamp(18.0, inner_width)
        .min(inner_width);
    let ratio = (text_box.scroll_x / max_scroll).clamp(0.0, 1.0);
    let thumb_x0 = inner_left + ratio * (inner_width - thumb_w);
    let thumb = Rect::new(
        thumb_x0,
        track_y - track_height,
        thumb_x0 + thumb_w,
        track_y,
    );
    let thumb_shape = RoundedRect::from_rect(thumb, 2.0);
    let thumb_color = if text_box.enabled {
        Color::from_rgba8(255, 255, 255, 150)
    } else {
        Color::from_rgba8(255, 255, 255, 90)
    };
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        thumb_color,
        None,
        &thumb_shape,
    );
}

fn text_box_content_width(text_box: &TextBoxNode) -> f64 {
    if text_box.text_advance > 0.0 {
        text_box.text_advance
    } else {
        estimate_text_width(&text_box.text, text_box.font_size) as f64
    }
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
