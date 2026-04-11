use vello::peniko::FontData;
use vello::{Glyph, Scene};

use crate::retained::node::ButtonNode;
use crate::theme::color;
use crate::widget::text;

pub(super) const TEXT_CENTER_RATIO: f64 = 0.5;

pub(super) fn draw_centered_text(
    scene: &mut Scene,
    button: &ButtonNode,
    font: &FontData,
    glyphs: Vec<Glyph>,
    total_advance: f32,
) {
    let text_x: f64 =
        button.rect.x0 + (button.rect.width() - f64::from(total_advance)) * TEXT_CENTER_RATIO;
    let text_y: f64 = super::super::text_baseline::centered(button.rect, button.font_size);

    let text_color: vello::peniko::Color = if button.enabled {
        button.text_color
    } else {
        color::Neutral::tone_178_184_192()
    };

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        text_y,
        button.font_size,
        text_color,
    );
}
