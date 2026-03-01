use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};

use super::interaction_color;
use crate::retained::node::ButtonNode;
use crate::theme::color;
use crate::widget::text;

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/button.rs"]
mod tests;

const BUTTON_CORNER_RADIUS: f64 = 10.0;
const TEXT_CENTER_RATIO: f64 = 0.5;
const BASELINE_FONT_OFFSET_RATIO: f64 = 0.35;

/// Renders a button body and optional centered label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let color = change_color(button);
    let rounded = RoundedRect::from_rect(button.rect, BUTTON_CORNER_RADIUS);

    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);

    let (Some(font), Some(text)) = (font, button.text.as_deref()) else {
        return;
    };

    let Some((glyphs, total_advance)) = text::layout_text(font, text, button.font_size) else {
        return;
    };

    draw_text_run(scene, button, font, glyphs, total_advance);
}

/// Resolves button background color from enabled/pressed/hovered state priority.
fn change_color(button: &ButtonNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        button.enabled,
        button.pressed,
        button.hovered,
        color::Neutral::tone_83_90_100(),
        color::AccentBlue::tone_31_122_205(),
        color::AccentBlue::tone_69_160_242(),
        color::AccentBlue::tone_50_144_229(),
    )
}

/// Draws centered button label glyphs with state-aware text color.
fn draw_text_run(
    scene: &mut Scene,
    button: &ButtonNode,
    font: &FontData,
    glyphs: Vec<Glyph>,
    total_advance: f32,
) {
    let text_x = button.rect.x0 + (button.rect.width() - total_advance as f64) * TEXT_CENTER_RATIO;
    let text_y = button.rect.y0
        + button.rect.height() * TEXT_CENTER_RATIO
        + button.font_size as f64 * BASELINE_FONT_OFFSET_RATIO;

    let color = if button.enabled {
        button.text_color
    } else {
        color::Neutral::tone_178_184_192()
    };

    text::draw_text_run(scene, font, glyphs, text_x, text_y, button.font_size, color);
}
