mod colors;
mod constants;
mod indicator;
mod text;

use vello::Scene;
use vello::kurbo::{Affine, Stroke};
use vello::peniko::{Fill, FontData};

use crate::retained::node::RadioButtonNode;

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/radio_button.rs"]
mod tests;

/// Renders radio button indicator, selected dot, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, radio_button: &RadioButtonNode) {
    let geometry: indicator::IndicatorGeometry = indicator::indicator_geometry(radio_button.rect);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        colors::outer_background(radio_button),
        None,
        &geometry.outer_circle,
    );

    scene.stroke(
        &Stroke::new(constants::INDICATOR_BORDER_WIDTH),
        Affine::IDENTITY,
        colors::border(radio_button),
        None,
        &geometry.outer_circle,
    );

    if radio_button.selected {
        indicator::draw_selected_dot(scene, radio_button, &geometry);
    }

    let Some(font) = font else {
        return;
    };

    text::draw_label(
        scene,
        font,
        radio_button,
        geometry.center_x,
        geometry.radius,
    );
}
