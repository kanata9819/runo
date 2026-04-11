use vello::peniko::color::{AlphaColor, Srgb};

use crate::retained::node::ButtonNode;
use crate::theme::color;

pub(super) fn background(button: &ButtonNode) -> AlphaColor<Srgb> {
    super::super::interaction_color::resolve_interaction_color(
        button.enabled,
        button.pressed,
        button.hovered,
        color::Neutral::tone_83_90_100(),
        color::AccentBlue::tone_31_122_205(),
        color::AccentBlue::tone_69_160_242(),
        color::AccentBlue::tone_50_144_229(),
    )
}
