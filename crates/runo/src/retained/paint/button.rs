use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};

use super::interaction_color;
use crate::retained::node::ButtonNode;
use crate::theme::color;
use crate::widget::text;

const BUTTON_CORNER_RADIUS: f64 = 10.0;
const TEXT_CENTER_RATIO: f64 = 0.5;
const BASELINE_FONT_OFFSET_RATIO: f64 = 0.35;

/// Renders a button body and optional centered label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let color = change_color(button);
    let rounded = RoundedRect::from_rect(button.rect, BUTTON_CORNER_RADIUS);

    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);

    let (Some(font), Some(text)) = (font, button.text.as_deref()) else {
        return;
    };
    let Some((glyphs, total_advance)) = text::layout_text(font, text, button.font_size) else {
        return;
    };

    draw_text_run(scene, button, font, glyphs, total_advance);
}

/// Resolves button background color from enabled/pressed/hovered state priority.
fn change_color(button: &ButtonNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        button.enabled,
        button.pressed,
        button.hovered,
        color::Neutral::tone_83_90_100(),
        color::AccentBlue::tone_31_122_205(),
        color::AccentBlue::tone_69_160_242(),
        color::AccentBlue::tone_50_144_229(),
    )
}

/// Draws centered button label glyphs with state-aware text color.
fn draw_text_run(
    scene: &mut Scene,
    button: &ButtonNode,
    font: &FontData,
    glyphs: Vec<Glyph>,
    total_advance: f32,
) {
    let text_x = button.rect.x0 + (button.rect.width() - total_advance as f64) * TEXT_CENTER_RATIO;
    let text_y = button.rect.y0
        + button.rect.height() * TEXT_CENTER_RATIO
        + button.font_size as f64 * BASELINE_FONT_OFFSET_RATIO;

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        text_y,
        button.font_size,
        if button.enabled {
            button.text_color
        } else {
            color::Neutral::tone_178_184_192()
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::load_default_font;
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    /// Builds a reusable button fixture for paint helper tests.
    fn sample_button() -> ButtonNode {
        ButtonNode {
            rect: Rect::new(0.0, 0.0, 120.0, 40.0),
            text: Some("Button".to_string()),
            text_overridden: false,
            font_size: 14.0,
            text_color: Color::from_rgb8(255, 255, 255),
            enabled: true,
            hovered: false,
            pressed: false,
            clicked: false,
        }
    }

    #[test]
    /// Uses default enabled color when no interaction state is active.
    fn change_color_uses_default_enabled_color() {
        let button = sample_button();
        assert_eq!(change_color(&button), Color::from_rgb8(50, 144, 229));
    }

    #[test]
    /// Prioritizes pressed color over hovered color.
    fn change_color_prefers_pressed() {
        let mut button = sample_button();
        button.pressed = true;
        button.hovered = true;
        assert_eq!(change_color(&button), Color::from_rgb8(31, 122, 205));
    }

    #[test]
    /// Uses disabled color regardless of hovered/pressed state.
    fn change_color_uses_disabled_color() {
        let mut button = sample_button();
        button.enabled = false;
        button.pressed = true;
        button.hovered = true;
        assert_eq!(change_color(&button), Color::from_rgb8(83, 90, 100));
    }

    #[test]
    fn render_runs_with_and_without_font() {
        let mut scene = Scene::new();
        let button = sample_button();
        render(&mut scene, None, &button);

        if let Some(font) = load_default_font() {
            render(&mut scene, Some(&font), &button);
        }
    }

    #[test]
    fn draw_text_run_is_callable() {
        let Some(font) = load_default_font() else {
            return;
        };
        let mut scene = Scene::new();
        let button = sample_button();
        draw_text_run(
            &mut scene,
            &button,
            &font,
            vec![Glyph {
                id: 1,
                x: 0.0,
                y: 0.0,
            }],
            10.0,
        );
    }
}
