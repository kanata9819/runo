use vello::Glyph;
use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
use crate::theme::color;
use crate::widget::text;

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/label.rs"]
mod tests;

/// Renders single-line label text at the label rectangle origin.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some((font, glyphs, baseline_y)) = layout_for_label(font, label) else {
        return;
    };

    text::draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        resolve_label_text_color(label),
    );
}

fn layout_for_label<'a>(
    font: Option<&'a FontData>,
    label: &LabelNode,
) -> Option<(&'a FontData, Vec<Glyph>, f64)> {
    let font = font?;
    let (glyphs, _) = text::layout_text(font, &label.text, label.font_size)?;
    let baseline_y = label.rect.y0 + label.font_size as f64;

    Some((font, glyphs, baseline_y))
}

fn resolve_label_text_color(label: &LabelNode) -> vello::peniko::Color {
    if label.enabled {
        label.text_color
    } else {
        color::Neutral::tone_142_148_156()
    }
}
