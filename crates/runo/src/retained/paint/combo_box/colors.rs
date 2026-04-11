use vello::peniko::color::{AlphaColor, Srgb};

use crate::retained::node::ComboBoxNode;
use crate::theme::color;

pub(super) fn border(combo_box: &ComboBoxNode) -> AlphaColor<Srgb> {
    super::super::interaction_color::resolve_interaction_color(
        combo_box.enabled,
        combo_box.pressed,
        combo_box.hovered,
        color::Neutral::tone_86_92_101(),
        color::AccentBlue::tone_89_176_255(),
        color::AccentBlue::tone_124_177_230(),
        combo_box.border_color,
    )
}

pub(super) fn body_background(combo_box: &ComboBoxNode) -> AlphaColor<Srgb> {
    if combo_box.enabled {
        combo_box.bg_color
    } else {
        color::Neutral::tone_45_49_55()
    }
}

pub(super) fn body_text(combo_box: &ComboBoxNode) -> AlphaColor<Srgb> {
    if combo_box.enabled {
        combo_box.text_color
    } else {
        color::Neutral::tone_147_153_161()
    }
}

pub(super) fn arrow(combo_box: &ComboBoxNode) -> AlphaColor<Srgb> {
    if combo_box.enabled {
        color::SoftWhite::tone_186_196_210()
    } else {
        color::Neutral::tone_141_147_154()
    }
}

pub(super) fn dropdown_item_bg(combo_box: &ComboBoxNode, index: usize) -> AlphaColor<Srgb> {
    if combo_box.hovered_item == Some(index) {
        color::Neutral::tone_63_80_102()
    } else if combo_box.selected_index == index {
        color::Neutral::tone_46_64_86()
    } else {
        combo_box.bg_color
    }
}
