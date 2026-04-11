mod colors;
mod text;

use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::{Fill, FontData};

use crate::retained::node::ButtonNode;
use crate::widget::text as text_widget;

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/button.rs"]
mod tests;

const BUTTON_CORNER_RADIUS: f64 = 10.0;

/// Renders a button body and optional centered label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let rounded_rect: RoundedRect = RoundedRect::from_rect(button.rect, BUTTON_CORNER_RADIUS);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        colors::background(button),
        None,
        &rounded_rect,
    );

    let (Some(font), Some(text)) = (font, button.text.as_deref()) else {
        return;
    };

    let Some((glyphs, total_advance)) = text_widget::layout_text(font, text, button.font_size)
    else {
        return;
    };

    text::draw_centered_text(scene, button, font, glyphs, total_advance);
}
