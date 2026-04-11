use vello::Scene;

use crate::retained::node::CheckboxNode;
use crate::widget::text;

pub(super) fn draw_label(
    scene: &mut Scene,
    font: &vello::peniko::FontData,
    checkbox: &CheckboxNode,
    indicator_x: f64,
    indicator_size: f64,
) {
    let Some(label_text) = checkbox.text.as_deref() else {
        return;
    };

    let Some((glyphs, _)) = text::layout_text(font, label_text, checkbox.font_size) else {
        return;
    };

    let text_x: f64 = indicator_x + indicator_size + super::constants::LABEL_TEXT_SPACING;
    let baseline_y: f64 = super::super::text_baseline::centered(checkbox.rect, checkbox.font_size);

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        checkbox.font_size,
        super::colors::label_text(checkbox),
    );
}
