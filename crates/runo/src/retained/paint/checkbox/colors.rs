use vello::peniko::color::{AlphaColor, Srgb};

use crate::retained::node::CheckboxNode;
use crate::theme::color;

pub(super) fn indicator_background(checkbox: &CheckboxNode) -> AlphaColor<Srgb> {
    super::super::interaction_color::resolve_interaction_color(
        checkbox.enabled,
        checkbox.pressed,
        checkbox.hovered,
        color::Neutral::tone_43_47_53(),
        color::AccentBlue::tone_45_129_205(),
        color::AccentBlue::tone_53_141_221(),
        if checkbox.checked {
            color::AccentBlue::tone_50_144_229()
        } else {
            color::Neutral::tone_36_42_50()
        },
    )
}

pub(super) fn indicator_border(checkbox: &CheckboxNode) -> vello::peniko::Color {
    if checkbox.enabled {
        color::Neutral::tone_130_145_163()
    } else {
        color::Neutral::tone_88_94_102()
    }
}

pub(super) fn check_mark(checkbox: &CheckboxNode) -> vello::peniko::Color {
    if checkbox.enabled {
        color::SoftWhite::tone_240_246_255()
    } else {
        color::Neutral::tone_167_173_181()
    }
}

pub(super) fn label_text(checkbox: &CheckboxNode) -> vello::peniko::Color {
    if checkbox.enabled {
        checkbox.text_color
    } else {
        color::Neutral::tone_146_152_160()
    }
}
