use vello::peniko::color::{AlphaColor, Srgb};

use crate::retained::node::SliderNode;
use crate::theme::color;

pub(super) fn active_fill(slider: &SliderNode) -> AlphaColor<Srgb> {
    super::super::interaction_color::resolve_interaction_color(
        slider.enabled,
        slider.pressed,
        slider.hovered,
        color::Neutral::tone_78_82_90(),
        color::AccentBlue::tone_37_132_214(),
        color::AccentBlue::tone_62_154_234(),
        color::AccentBlue::tone_50_144_229(),
    )
}

pub(super) fn track(slider: &SliderNode) -> vello::peniko::Color {
    if slider.enabled {
        color::Neutral::tone_56_63_74()
    } else {
        color::Neutral::tone_48_52_58()
    }
}

pub(super) fn thumb_fill(slider: &SliderNode) -> vello::peniko::Color {
    if slider.enabled {
        color::SoftWhite::tone_240_246_255()
    } else {
        color::Neutral::tone_163_169_177()
    }
}

pub(super) fn text(slider: &SliderNode) -> vello::peniko::Color {
    if slider.enabled {
        slider.text_color
    } else {
        color::Neutral::tone_146_152_160()
    }
}
