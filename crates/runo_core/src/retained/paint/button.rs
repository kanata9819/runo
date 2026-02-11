use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::ButtonNode;
use crate::widget::text::{draw_text_run, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let color = if button.pressed {
        Color::from_rgb8(31, 122, 205)
    } else if button.hovered {
        Color::from_rgb8(69, 160, 242)
    } else {
        Color::from_rgb8(50, 144, 229)
    };

    let rounded = RoundedRect::from_rect(button.rect, 10.0);
    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);

    let (Some(font), Some(text)) = (font, button.text.as_deref()) else {
        return;
    };
    let Some((glyphs, total_advance)) = layout_text(font, text, button.font_size) else {
        return;
    };

    let text_x = button.rect.x0 + (button.rect.width() - total_advance as f64) * 0.5;
    let text_y = button.rect.y0 + button.rect.height() * 0.5 + button.font_size as f64 * 0.35;
    draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        text_y,
        button.font_size,
        button.text_color,
    );
}
