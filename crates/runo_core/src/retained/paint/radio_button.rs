use vello::Scene;
use vello::kurbo::{Affine, Circle, Stroke};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};

use super::interaction_color;
use crate::retained::node::RadioButtonNode;
use crate::theme::color;
use crate::widget::text;

const INDICATOR_X_OFFSET: f64 = 2.0;
const INDICATOR_BORDER_WIDTH: f64 = 1.0;
const INDICATOR_SIZE_OFFSET: f64 = 8.0;
const INDICATOR_SIZE_MIN: f64 = 14.0;
const INDICATOR_SIZE_MAX: f64 = 24.0;
const OUTER_RADIUS_RATIO: f64 = 0.5;
const INNER_RADIUS_RATIO: f64 = 0.45;
const BASELINE_VERTICAL_RATIO: f64 = 0.5;
const BASELINE_FONT_OFFSET_RATIO: f64 = 0.35;
const LABEL_TEXT_SPACING: f64 = 10.0;

/// Renders radio button indicator, selected dot, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, radio_button: &RadioButtonNode) {
    let indicator_size = indicator_size(radio_button.rect.height());
    let indicator_radius = indicator_size * OUTER_RADIUS_RATIO;
    let center_x = radio_button.rect.x0 + INDICATOR_X_OFFSET + indicator_radius;
    let center_y = radio_button.rect.y0 + radio_button.rect.height() * BASELINE_VERTICAL_RATIO;
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
        &Stroke::new(INDICATOR_BORDER_WIDTH),
        Affine::IDENTITY,
        if radio_button.enabled {
            color::rgb(color::widget::RADIO_BORDER_ENABLED)
        } else {
            color::rgb(color::widget::RADIO_BORDER_DISABLED)
        },
        None,
        &outer_circle,
    );

    if radio_button.selected {
        let inner_radius = indicator_radius * INNER_RADIUS_RATIO;
        let inner_circle = Circle::new((center_x, center_y), inner_radius);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            if radio_button.enabled {
                color::rgb(color::widget::RADIO_MARK_ENABLED)
            } else {
                color::rgb(color::widget::RADIO_MARK_DISABLED)
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
    let text_x = center_x + indicator_radius + LABEL_TEXT_SPACING;
    let baseline_y = radio_button.rect.y0
        + radio_button.rect.height() * BASELINE_VERTICAL_RATIO
        + radio_button.font_size as f64 * BASELINE_FONT_OFFSET_RATIO;

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
            color::rgb(color::widget::RADIO_TEXT_DISABLED)
        },
    );
}

/// Computes radio indicator diameter from widget height with clamped bounds.
fn indicator_size(height: f64) -> f64 {
    (height - INDICATOR_SIZE_OFFSET).clamp(INDICATOR_SIZE_MIN, INDICATOR_SIZE_MAX)
}

/// Resolves radio outer indicator color from enabled/pressed/hovered state priority.
fn outer_bg_color(radio_button: &RadioButtonNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        radio_button.enabled,
        radio_button.pressed,
        radio_button.hovered,
        color::rgb(color::widget::RADIO_DISABLED_BG),
        color::rgb(color::widget::RADIO_PRESSED_BG),
        color::rgb(color::widget::RADIO_HOVER_BG),
        color::rgb(color::widget::RADIO_ENABLED_BG),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use vello::kurbo::Rect;
    use vello::peniko::Color;

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
