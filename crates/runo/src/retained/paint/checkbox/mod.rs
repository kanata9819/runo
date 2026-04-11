mod colors;
mod constants;
mod indicator;
mod text;

use vello::Scene;
use vello::kurbo::{Affine, Stroke};
use vello::peniko::{Fill, FontData};

use crate::retained::node::CheckboxNode;

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/checkbox.rs"]
mod tests;

/// Renders checkbox indicator, optional check mark, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, checkbox: &CheckboxNode) {
    let geometry: indicator::IndicatorGeometry = indicator::indicator_geometry(checkbox.rect);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        colors::indicator_background(checkbox),
        None,
        &geometry.rect,
    );

    scene.stroke(
        &Stroke::new(constants::INDICATOR_BORDER_WIDTH),
        Affine::IDENTITY,
        colors::indicator_border(checkbox),
        None,
        &geometry.rect,
    );

    if checkbox.checked {
        indicator::draw_check_mark(scene, checkbox, &geometry);
    }

    let Some(font) = font else {
        return;
    };

    text::draw_label(scene, font, checkbox, geometry.x, geometry.size);
}
