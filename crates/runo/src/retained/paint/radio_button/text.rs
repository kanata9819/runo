use vello::Scene;

use crate::retained::node::RadioButtonNode;
use crate::widget::text;

pub(super) fn draw_label(
    scene: &mut Scene,
    font: &vello::peniko::FontData,
    radio_button: &RadioButtonNode,
    center_x: f64,
    indicator_radius: f64,
) {
    let Some(label_text) = radio_button.text.as_deref() else {
        return;
    };

    let Some((glyphs, _)) = text::layout_text(font, label_text, radio_button.font_size) else {
        return;
    };

    let text_x: f64 = center_x + indicator_radius + super::constants::LABEL_TEXT_SPACING;
    let baseline_y: f64 =
        super::super::text_baseline::centered(radio_button.rect, radio_button.font_size);

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        radio_button.font_size,
        super::colors::label_text(radio_button),
    );
}
