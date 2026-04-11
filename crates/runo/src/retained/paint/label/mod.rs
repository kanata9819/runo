mod colors;
mod layout;

use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
#[cfg(test)]
use crate::theme::color;
use crate::widget::text;

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/label.rs"]
mod tests;

/// Renders single-line label text at the label rectangle origin.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some((font, glyphs)) = layout::label_glyphs(font, label) else {
        return;
    };
    let baseline_y: f64 = layout::baseline_y(label);

    text::draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        colors::text_color(label),
    );
}
