use vello::peniko::color::{AlphaColor, Srgb};

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

#[cfg(test)]
mod tests {
    use super::resolve_interaction_color;
    use vello::peniko::Color;

    #[test]
    /// Returns disabled color regardless of interaction state when not enabled.
    fn resolve_interaction_color_prefers_disabled() {
        let c = resolve_interaction_color(
            false,
            true,
            true,
            Color::from_rgb8(1, 2, 3),
            Color::from_rgb8(4, 5, 6),
            Color::from_rgb8(7, 8, 9),
            Color::from_rgb8(10, 11, 12),
        );
        assert_eq!(c, Color::from_rgb8(1, 2, 3));
    }

    #[test]
    /// Returns pressed color before hovered color when enabled.
    fn resolve_interaction_color_prefers_pressed() {
        let c = resolve_interaction_color(
            true,
            true,
            true,
            Color::from_rgb8(1, 2, 3),
            Color::from_rgb8(4, 5, 6),
            Color::from_rgb8(7, 8, 9),
            Color::from_rgb8(10, 11, 12),
        );
        assert_eq!(c, Color::from_rgb8(4, 5, 6));
    }
}
