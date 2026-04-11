use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::SliderNode;
use crate::widget::text;

pub(super) fn draw_optional_label(
    scene: &mut Scene,
    font: &FontData,
    slider: &SliderNode,
    pad_x: f64,
) {
    if let Some(label_text) = slider.text.as_deref()
        && let Some((glyphs, _)) = text::layout_text(font, label_text, slider.font_size)
    {
        let baseline_y: f64 =
            super::super::text_baseline::top_aligned(slider.rect, slider.font_size);

        text::draw_text_run(
            scene,
            font,
            glyphs,
            slider.rect.x0 + pad_x,
            baseline_y,
            slider.font_size,
            super::colors::text(slider),
        );
    }
}

pub(super) fn draw_value_text(scene: &mut Scene, font: &FontData, slider: &SliderNode, pad_x: f64) {
    let value_text: String = format!("{:.*}", super::layout::VALUE_DECIMALS, slider.value);

    if let Some((glyphs, width)) = text::layout_text(font, &value_text, slider.font_size) {
        let baseline_y: f64 =
            super::super::text_baseline::top_aligned(slider.rect, slider.font_size);

        text::draw_text_run(
            scene,
            font,
            glyphs,
            slider.rect.x1 - pad_x - f64::from(width),
            baseline_y,
            slider.font_size,
            super::colors::text(slider),
        );
    }
}
