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
