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

    let text_x = text_box.rect.x0 + 12.0;
    let baseline_y =
        text_box.rect.y0 + text_box.rect.height() * 0.5 + text_box.font_size as f64 * 0.35;
    draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        text_box.font_size,
        text_color,
    );

    if text_box.focused && text_box.enabled {
        let caret_x = if text_box.text.is_empty() {
            text_x
        } else {
            text_x + total_advance as f64 + 1.0
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
