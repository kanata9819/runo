use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect, Stroke};
use vello::peniko::{Fill, FontData};

use crate::retained::node::ComboBoxNode;
use crate::widget::text;

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, combo_box: &ComboBoxNode) {
    if !combo_box.enabled || !combo_box.is_open || combo_box.items.is_empty() {
        return;
    }

    let Some(font) = font else {
        return;
    };

    let item_height: f64 = combo_box.rect.height();

    for (index, item) in combo_box.items.iter().enumerate() {
        let y0: f64 = combo_box.rect.y1 + item_height * index as f64;
        let item_rect: Rect = Rect::new(combo_box.rect.x0, y0, combo_box.rect.x1, y0 + item_height);
        let item_bg: RoundedRect =
            RoundedRect::from_rect(item_rect, super::layout::ITEM_CORNER_RADIUS);

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            super::colors::dropdown_item_bg(combo_box, index),
            None,
            &item_bg,
        );

        scene.stroke(
            &Stroke::new(super::BORDER_STROKE_WIDTH),
            Affine::IDENTITY,
            combo_box.border_color,
            None,
            &item_bg,
        );

        if let Some((glyphs, _)) = text::layout_text(font, item, combo_box.font_size) {
            let text_x: f64 = item_rect.x0 + super::layout::TEXT_HORIZONTAL_PADDING;
            let baseline_y: f64 = super::layout::baseline_y(item_rect, combo_box.font_size);
            text::draw_text_run(
                scene,
                font,
                glyphs,
                text_x,
                baseline_y,
                combo_box.font_size,
                combo_box.text_color,
            );
        }
    }
}
