use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::TextBoxNode;
use crate::widget::text::{draw_text_run, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, text_box: &TextBoxNode) {
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

    let draw_text = if text_box.text.is_empty() {
        text_box.placeholder.as_deref().unwrap_or("")
    } else {
        text_box.text.as_str()
    };

    let text_color = if !text_box.enabled {
        Color::from_rgb8(147, 153, 161)
    } else if text_box.text.is_empty() {
        Color::from_rgb8(142, 151, 163)
    } else {
        text_box.text_color
    };

    let Some((glyphs, total_advance)) = layout_text(font, draw_text, text_box.font_size) else {
        return;
    };

    let text_x = text_box.rect.x0 + 12.0 - text_box.scroll_x;
    let baseline_y =
        text_box.rect.y0 + text_box.rect.height() * 0.5 + text_box.font_size as f64 * 0.35
            - text_box.scroll_y;
    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;
    let visible_glyphs = if text_box.overflow_x.clips() {
        clip_glyphs_horizontally(
            glyphs,
            total_advance as f64,
            text_x,
            inner_left,
            inner_right,
        )
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

    if text_box.focused && text_box.enabled {
        let caret_x = if text_box.text.is_empty() {
            text_x
        } else {
            text_x + total_advance as f64 + 1.0
        };
        let caret_x = if text_box.overflow_x.clips() {
            caret_x.clamp(inner_left, inner_right)
        } else {
            caret_x
        };
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
