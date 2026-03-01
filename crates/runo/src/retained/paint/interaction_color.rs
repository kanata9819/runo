use vello::peniko::color::{AlphaColor, Srgb};

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/interaction_color.rs"]
mod tests;

/// Resolves interaction color with priority: disabled > pressed > hovered > default.
#[inline]
pub(super) fn resolve_interaction_color(
    enabled: bool,
    pressed: bool,
    hovered: bool,
    disabled_color: AlphaColor<Srgb>,
    pressed_color: AlphaColor<Srgb>,
    hovered_color: AlphaColor<Srgb>,
    default_color: AlphaColor<Srgb>,
) -> AlphaColor<Srgb> {
    if !enabled {
        disabled_color
    } else if pressed {
        pressed_color
    } else if hovered {
        hovered_color
    } else {
        default_color
    }
}
