use vello::Scene;
use vello::kurbo::{Affine, Line, RoundedRect, Stroke};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Color, Fill, FontData};

use super::interaction_color::resolve_interaction_color;
use crate::retained::node::CheckboxNode;
use crate::widget::text::{draw_text_run, layout_text};

/// Renders checkbox indicator, optional check mark, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, checkbox: &CheckboxNode) {
    let indicator_size = indicator_size(checkbox.rect.height());
    let indicator_x = checkbox.rect.x0 + 2.0;
    let indicator_y = checkbox.rect.y0 + (checkbox.rect.height() - indicator_size) * 0.5;
    let indicator_rect = RoundedRect::new(
        indicator_x,
        indicator_y,
        indicator_x + indicator_size,
        indicator_y + indicator_size,
        4.0,
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
        &Stroke::new(1.0),
        Affine::IDENTITY,
        if checkbox.enabled {
            Color::from_rgb8(130, 145, 163)
        } else {
            Color::from_rgb8(88, 94, 102)
        },
        None,
        &indicator_rect,
    );

    if checkbox.checked {
        let check_color = if checkbox.enabled {
            Color::from_rgb8(240, 246, 255)
        } else {
            Color::from_rgb8(167, 173, 181)
        };
        let x0 = indicator_x + indicator_size * 0.22;
        let y0 = indicator_y + indicator_size * 0.56;
        let x1 = indicator_x + indicator_size * 0.44;
        let y1 = indicator_y + indicator_size * 0.76;
        let x2 = indicator_x + indicator_size * 0.80;
        let y2 = indicator_y + indicator_size * 0.28;
        scene.stroke(
            &Stroke::new(2.2),
            Affine::IDENTITY,
            check_color,
            None,
            &Line::new((x0, y0), (x1, y1)),
        );
        scene.stroke(
            &Stroke::new(2.2),
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
    let Some((glyphs, _)) = layout_text(font, text, checkbox.font_size) else {
        return;
    };

    let text_x = indicator_x + indicator_size + 10.0;
    let baseline_y =
        checkbox.rect.y0 + checkbox.rect.height() * 0.5 + checkbox.font_size as f64 * 0.35;
    draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        checkbox.font_size,
        if checkbox.enabled {
            checkbox.text_color
        } else {
            Color::from_rgb8(146, 152, 160)
        },
    );
}

/// Computes checkbox indicator square size from widget height with clamped bounds.
fn indicator_size(height: f64) -> f64 {
    (height - 8.0).clamp(14.0, 24.0)
}

/// Resolves indicator background color from enabled/pressed/hovered/checked state priority.
fn indicator_bg_color(checkbox: &CheckboxNode) -> AlphaColor<Srgb> {
    resolve_interaction_color(
        checkbox.enabled,
        checkbox.pressed,
        checkbox.hovered,
        Color::from_rgb8(43, 47, 53),
        Color::from_rgb8(45, 129, 205),
        Color::from_rgb8(53, 141, 221),
        if checkbox.checked {
            Color::from_rgb8(50, 144, 229)
        } else {
            Color::from_rgb8(36, 42, 50)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use vello::kurbo::Rect;

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
