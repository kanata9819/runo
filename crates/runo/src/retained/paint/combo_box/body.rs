use vello::Scene;
use vello::kurbo::{Affine, RoundedRect, Stroke};
use vello::peniko::{Fill, FontData};

use crate::retained::node::ComboBoxNode;
use crate::widget::text;

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, combo_box: &ComboBoxNode) {
    let bg: RoundedRect =
        RoundedRect::from_rect(combo_box.rect, super::layout::COMBO_BOX_CORNER_RADIUS);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        super::colors::body_background(combo_box),
        None,
        &bg,
    );

    scene.stroke(
        &Stroke::new(super::BORDER_STROKE_WIDTH),
        Affine::IDENTITY,
        super::colors::border(combo_box),
        None,
        &bg,
    );

    let Some(font) = font else {
        return;
    };

    let selected_text: &str = super::layout::selected_text(combo_box);
    if let Some((glyphs, _)) = text::layout_text(font, selected_text, combo_box.font_size) {
        super::text::draw_selected_text(scene, font, glyphs, combo_box);
    }

    let arrow: &str = if combo_box.is_open { "^" } else { "v" };
    if let Some((glyphs, arrow_width)) = text::layout_text(
        font,
        arrow,
        combo_box.font_size * super::layout::ARROW_FONT_SCALE,
    ) {
        let arrow_x: f64 =
            combo_box.rect.x1 - f64::from(arrow_width) - super::layout::TEXT_HORIZONTAL_PADDING;

        super::text::draw_text_run_at(
            scene,
            font,
            glyphs,
            arrow_x,
            combo_box.rect,
            combo_box.font_size * super::layout::ARROW_FONT_SCALE,
            super::colors::arrow(combo_box),
        );
    }
}
