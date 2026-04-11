mod colors;
mod layout;
mod text;
mod track;

use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::SliderNode;

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/slider.rs"]
mod tests;

/// Renders slider track, active fill, thumb, optional label, and current numeric value.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, slider: &SliderNode) {
    let geometry: layout::SliderGeometry = layout::geometry(slider);

    track::draw_track(scene, slider, &geometry.track_rect);
    track::draw_active_fill(
        scene,
        slider,
        geometry.track_x0,
        geometry.track_y,
        geometry.track_height,
        geometry.thumb_x,
    );
    track::draw_thumb(scene, slider, geometry.thumb_x, geometry.track_y);

    let Some(font) = font else {
        return;
    };

    text::draw_optional_label(scene, font, slider, geometry.pad_x);
    text::draw_value_text(scene, font, slider, geometry.pad_x);
}
