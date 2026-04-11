mod body;
mod colors;
mod layout;
mod overlay;
mod text;

use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::ComboBoxNode;

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/combo_box.rs"]
mod tests;

const BORDER_STROKE_WIDTH: f64 = 1.0;

/// Renders the closed combo box body, border, selected text, and open/close arrow.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, combo_box: &ComboBoxNode) {
    body::render(scene, font, combo_box);
}

/// Renders the dropdown item list overlay when the combo box is open and enabled.
pub(super) fn render_dropdown_overlay(
    scene: &mut Scene,
    font: Option<&FontData>,
    combo_box: &ComboBoxNode,
) {
    overlay::render(scene, font, combo_box);
}
