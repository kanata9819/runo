use vello::peniko::color::{AlphaColor, Srgb};

use crate::retained::node::RadioButtonNode;
use crate::theme::color;

pub(super) fn outer_background(radio_button: &RadioButtonNode) -> AlphaColor<Srgb> {
    super::super::interaction_color::resolve_interaction_color(
        radio_button.enabled,
        radio_button.pressed,
        radio_button.hovered,
        color::Neutral::tone_43_47_53(),
        color::AccentBlue::tone_45_129_205(),
        color::AccentBlue::tone_53_141_221(),
        color::Neutral::tone_36_42_50(),
    )
}

pub(super) fn border(radio_button: &RadioButtonNode) -> vello::peniko::Color {
    if radio_button.enabled {
        color::Neutral::tone_130_145_163()
    } else {
        color::Neutral::tone_88_94_102()
    }
}

pub(super) fn inner_dot(radio_button: &RadioButtonNode) -> vello::peniko::Color {
    if radio_button.enabled {
        color::SoftWhite::tone_240_246_255()
    } else {
        color::Neutral::tone_167_173_181()
    }
}

pub(super) fn label_text(radio_button: &RadioButtonNode) -> vello::peniko::Color {
    if radio_button.enabled {
        radio_button.text_color
    } else {
        color::Neutral::tone_146_152_160()
    }
}
