use vello::kurbo::Rect;

use crate::retained::node::ComboBoxNode;

pub(super) const TEXT_HORIZONTAL_PADDING: f64 = 12.0;
pub(super) const ARROW_FONT_SCALE: f32 = 0.85;
pub(super) const COMBO_BOX_CORNER_RADIUS: f64 = 8.0;
pub(super) const ITEM_CORNER_RADIUS: f64 = 0.0;

pub(super) fn selected_text(combo_box: &ComboBoxNode) -> &str {
    combo_box
        .items
        .get(combo_box.selected_index)
        .map_or("", String::as_str)
}

pub(super) fn baseline_y(rect: Rect, font_size: f32) -> f64 {
    super::super::text_baseline::centered(rect, font_size)
}
