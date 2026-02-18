use vello::Scene;
use vello::kurbo::{Affine, Line, RoundedRect, Stroke};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};

use super::interaction_color;
use crate::retained::node::CheckboxNode;
use crate::theme::color;
use crate::widget::text;

const INDICATOR_X_OFFSET: f64 = 2.0;
const INDICATOR_CORNER_RADIUS: f64 = 4.0;
const INDICATOR_BORDER_WIDTH: f64 = 1.0;
const CHECK_STROKE_WIDTH: f64 = 2.2;
const CHECK_X0_RATIO: f64 = 0.22;
const CHECK_Y0_RATIO: f64 = 0.56;
const CHECK_X1_RATIO: f64 = 0.44;
const CHECK_Y1_RATIO: f64 = 0.76;
const CHECK_X2_RATIO: f64 = 0.80;
const CHECK_Y2_RATIO: f64 = 0.28;
const LABEL_TEXT_SPACING: f64 = 10.0;
const BASELINE_VERTICAL_RATIO: f64 = 0.5;
const BASELINE_FONT_OFFSET_RATIO: f64 = 0.35;
const INDICATOR_SIZE_OFFSET: f64 = 8.0;
const INDICATOR_SIZE_MIN: f64 = 14.0;
const INDICATOR_SIZE_MAX: f64 = 24.0;

/// Renders checkbox indicator, optional check mark, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, checkbox: &CheckboxNode) {
    let indicator_size = indicator_size(checkbox.rect.height());
    let indicator_x = checkbox.rect.x0 + INDICATOR_X_OFFSET;
    let indicator_y =
        checkbox.rect.y0 + (checkbox.rect.height() - indicator_size) * BASELINE_VERTICAL_RATIO;
    let indicator_rect = RoundedRect::new(
        indicator_x,
        indicator_y,
        indicator_x + indicator_size,
        indicator_y + indicator_size,
        INDICATOR_CORNER_RADIUS,
    );

    let bg_color = indicator_bg_color(checkbox);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        bg_color,
        None,
        &indicator_rect,
    );
    scene.stroke(
        &Stroke::new(INDICATOR_BORDER_WIDTH),
        Affine::IDENTITY,
        if checkbox.enabled {
            color::rgb(color::widget::CHECKBOX_BORDER_ENABLED)
        } else {
            color::rgb(color::widget::CHECKBOX_BORDER_DISABLED)
        },
        None,
        &indicator_rect,
    );

    if checkbox.checked {
        let check_color = if checkbox.enabled {
            color::rgb(color::widget::CHECKBOX_MARK_ENABLED)
        } else {
            color::rgb(color::widget::CHECKBOX_MARK_DISABLED)
        };
        let x0 = indicator_x + indicator_size * CHECK_X0_RATIO;
        let y0 = indicator_y + indicator_size * CHECK_Y0_RATIO;
        let x1 = indicator_x + indicator_size * CHECK_X1_RATIO;
        let y1 = indicator_y + indicator_size * CHECK_Y1_RATIO;
        let x2 = indicator_x + indicator_size * CHECK_X2_RATIO;
        let y2 = indicator_y + indicator_size * CHECK_Y2_RATIO;
        scene.stroke(
            &Stroke::new(CHECK_STROKE_WIDTH),
            Affine::IDENTITY,
            check_color,
            None,
            &Line::new((x0, y0), (x1, y1)),
        );
        scene.stroke(
            &Stroke::new(CHECK_STROKE_WIDTH),
            Affine::IDENTITY,
            check_color,
            None,
            &Line::new((x1, y1), (x2, y2)),
        );
    }

    let Some(font) = font else {
        return;
    };
    let Some(text) = checkbox.text.as_deref() else {
        return;
    };
    let Some((glyphs, _)) = text::layout_text(font, text, checkbox.font_size) else {
        return;
    };

    let text_x = indicator_x + indicator_size + LABEL_TEXT_SPACING;
    let baseline_y = checkbox.rect.y0
        + checkbox.rect.height() * BASELINE_VERTICAL_RATIO
        + checkbox.font_size as f64 * BASELINE_FONT_OFFSET_RATIO;
    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        checkbox.font_size,
        if checkbox.enabled {
            checkbox.text_color
        } else {
            color::rgb(color::widget::CHECKBOX_TEXT_DISABLED)
        },
    );
}

/// Computes checkbox indicator square size from widget height with clamped bounds.
fn indicator_size(height: f64) -> f64 {
    (height - INDICATOR_SIZE_OFFSET).clamp(INDICATOR_SIZE_MIN, INDICATOR_SIZE_MAX)
}

/// Resolves indicator background color from enabled/pressed/hovered/checked state priority.
fn indicator_bg_color(checkbox: &CheckboxNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        checkbox.enabled,
        checkbox.pressed,
        checkbox.hovered,
        color::rgb(color::widget::CHECKBOX_DISABLED_BG),
        color::rgb(color::widget::CHECKBOX_PRESSED_BG),
        color::rgb(color::widget::CHECKBOX_HOVER_BG),
        if checkbox.checked {
            color::rgb(color::widget::CHECKBOX_CHECKED_BG)
        } else {
            color::rgb(color::widget::CHECKBOX_UNCHECKED_BG)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    /// Creates a reusable checkbox fixture for helper-function tests.
    fn sample_checkbox() -> CheckboxNode {
        CheckboxNode {
            rect: Rect::new(0.0, 0.0, 160.0, 24.0),
            text: Some("Check".to_string()),
            checked: false,
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
    /// Prioritizes pressed state over hovered and checked states.
    fn indicator_bg_color_prefers_pressed() {
        let mut checkbox = sample_checkbox();
        checkbox.pressed = true;
        checkbox.hovered = true;
        checkbox.checked = true;
        assert_eq!(
            indicator_bg_color(&checkbox),
            Color::from_rgb8(45, 129, 205)
        );
    }

    #[test]
    /// Returns disabled indicator color regardless of interaction states.
    fn indicator_bg_color_uses_disabled_color() {
        let mut checkbox = sample_checkbox();
        checkbox.enabled = false;
        checkbox.pressed = true;
        checkbox.hovered = true;
        checkbox.checked = true;
        assert_eq!(indicator_bg_color(&checkbox), Color::from_rgb8(43, 47, 53));
    }
}
