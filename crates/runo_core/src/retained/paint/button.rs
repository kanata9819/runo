use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Color, Fill, FontData};

use super::interaction_color::resolve_interaction_color;
use crate::retained::node::ButtonNode;
use crate::widget::text;

/// Renders a button body and optional centered label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let color = change_color(button);
    let rounded = RoundedRect::from_rect(button.rect, 10.0);

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
    resolve_interaction_color(
        button.enabled,
        button.pressed,
        button.hovered,
        Color::from_rgb8(83, 90, 100),
        Color::from_rgb8(31, 122, 205),
        Color::from_rgb8(69, 160, 242),
        Color::from_rgb8(50, 144, 229),
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
    let text_x = button.rect.x0 + (button.rect.width() - total_advance as f64) * 0.5;
    let text_y = button.rect.y0 + button.rect.height() * 0.5 + button.font_size as f64 * 0.35;

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
            Color::from_rgb8(178, 184, 192)
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use vello::kurbo::Rect;

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
}
