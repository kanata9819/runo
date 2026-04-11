use vello::kurbo::Rect;
use vello::peniko::FontData;
use vello::{Glyph, Scene};

use crate::retained::node::ComboBoxNode;
use crate::widget::text;

pub(super) fn draw_text_run_at(
    scene: &mut Scene,
    font: &FontData,
    glyphs: Vec<Glyph>,
    x: f64,
    rect: Rect,
    font_size: f32,
    color: vello::peniko::Color,
) {
    text::draw_text_run(
        scene,
        font,
        glyphs,
        x,
        super::layout::baseline_y(rect, font_size),
        font_size,
        color,
    );
}

pub(super) fn draw_selected_text(
    scene: &mut Scene,
    font: &FontData,
    glyphs: Vec<Glyph>,
    combo_box: &ComboBoxNode,
) {
    let text_x: f64 = combo_box.rect.x0 + super::layout::TEXT_HORIZONTAL_PADDING;

    draw_text_run_at(
        scene,
        font,
        glyphs,
        text_x,
        combo_box.rect,
        combo_box.font_size,
        super::colors::body_text(combo_box),
    );
}
