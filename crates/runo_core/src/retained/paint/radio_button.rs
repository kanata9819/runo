use vello::Scene;
use vello::kurbo::{Affine, Circle, Stroke};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Color, Fill, FontData};

use super::interaction_color;
use crate::retained::node::RadioButtonNode;
use crate::widget::text;

/// Renders radio button indicator, selected dot, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, radio_button: &RadioButtonNode) {
    let indicator_size = indicator_size(radio_button.rect.height());
    let indicator_radius = indicator_size * 0.5;
    let center_x = radio_button.rect.x0 + 2.0 + indicator_radius;
    let center_y = radio_button.rect.y0 + radio_button.rect.height() * 0.5;
    let outer_circle = Circle::new((center_x, center_y), indicator_radius);

    let outer_bg = outer_bg_color(radio_button);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        outer_bg,
        None,
        &outer_circle,
    );
    scene.stroke(
        &Stroke::new(1.0),
        Affine::IDENTITY,
        if radio_button.enabled {
            Color::from_rgb8(130, 145, 163)
        } else {
            Color::from_rgb8(88, 94, 102)
        },
        None,
        &outer_circle,
    );

    if radio_button.selected {
        let inner_radius = indicator_radius * 0.45;
        let inner_circle = Circle::new((center_x, center_y), inner_radius);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            if radio_button.enabled {
                Color::from_rgb8(240, 246, 255)
            } else {
                Color::from_rgb8(167, 173, 181)
            },
            None,
            &inner_circle,
        );
    }

    let Some(font) = font else {
        return;
    };
    let Some(text) = radio_button.text.as_deref() else {
        return;
    };
    let Some((glyphs, _)) = text::layout_text(font, text, radio_button.font_size) else {
        return;
    };
    let text_x = center_x + indicator_radius + 10.0;
    let baseline_y = radio_button.rect.y0
        + radio_button.rect.height() * 0.5
        + radio_button.font_size as f64 * 0.35;

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        radio_button.font_size,
        if radio_button.enabled {
            radio_button.text_color
        } else {
            Color::from_rgb8(146, 152, 160)
        },
    );
}

/// Computes radio indicator diameter from widget height with clamped bounds.
fn indicator_size(height: f64) -> f64 {
    (height - 8.0).clamp(14.0, 24.0)
}

/// Resolves radio outer indicator color from enabled/pressed/hovered state priority.
fn outer_bg_color(radio_button: &RadioButtonNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        radio_button.enabled,
        radio_button.pressed,
        radio_button.hovered,
        Color::from_rgb8(43, 47, 53),
        Color::from_rgb8(45, 129, 205),
        Color::from_rgb8(53, 141, 221),
        Color::from_rgb8(36, 42, 50),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use vello::kurbo::Rect;

    /// Creates a reusable radio button fixture for helper-function tests.
    fn sample_radio_button() -> RadioButtonNode {
        RadioButtonNode {
            rect: Rect::new(0.0, 0.0, 180.0, 24.0),
            group: "group".to_string(),
            text: Some("Radio".to_string()),
            selected: false,
            font_size: 14.0,
            text_color: Color::from_rgb8(255, 255, 255),
            enabled: true,
            hovered: false,
            pressed: false,
            changed: false,
        }
    }

    #[test]
    /// Clamps indicator size to minimum when control is short.
    fn indicator_size_clamps_to_min() {
        assert_eq!(indicator_size(10.0), 14.0);
    }

    #[test]
    /// Clamps indicator size to maximum when control is tall.
    fn indicator_size_clamps_to_max() {
        assert_eq!(indicator_size(100.0), 24.0);
    }

    #[test]
    /// Prioritizes pressed color over hovered color.
    fn outer_bg_color_prefers_pressed() {
        let mut radio_button = sample_radio_button();
        radio_button.pressed = true;
        radio_button.hovered = true;
        assert_eq!(
            outer_bg_color(&radio_button),
            Color::from_rgb8(45, 129, 205)
        );
    }

    #[test]
    /// Uses disabled color regardless of interaction states.
    fn outer_bg_color_uses_disabled_color() {
        let mut radio_button = sample_radio_button();
        radio_button.enabled = false;
        radio_button.pressed = true;
        radio_button.hovered = true;
        assert_eq!(outer_bg_color(&radio_button), Color::from_rgb8(43, 47, 53));
    }
}
