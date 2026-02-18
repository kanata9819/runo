use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
use crate::theme::color;
use crate::widget::text;

/// Renders single-line label text at the label rectangle origin.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some(font) = font else {
        return;
    };
    let Some((glyphs, _)) = text::layout_text(font, &label.text, label.font_size) else {
        return;
    };
    let baseline_y = label.rect.y0 + label.font_size as f64;

    text::draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        if label.enabled {
            label.text_color
        } else {
            color::Neutral::tone_142_148_156()
        },
    );
}
