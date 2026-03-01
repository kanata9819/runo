use super::*;
use crate::font::load_default_font;
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

#[test]
fn render_runs_for_checked_and_unchecked_states() {
    let mut scene = Scene::new();
    let mut checkbox = sample_checkbox();
    render(&mut scene, None, &checkbox);

    if let Some(font) = load_default_font() {
        render(&mut scene, Some(&font), &checkbox);
        checkbox.checked = true;
        render(&mut scene, Some(&font), &checkbox);
    }
}
