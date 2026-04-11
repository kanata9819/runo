use super::*;
use crate::font::load_default_font;
use vello::Glyph;
use vello::kurbo::Rect;
use vello::peniko::Color;

/// Creates a reusable combo box fixture for paint helper tests.
fn sample_combo_box() -> ComboBoxNode {
    ComboBoxNode {
        rect: Rect::new(10.0, 20.0, 210.0, 60.0),
        items: vec!["first".to_string(), "second".to_string()],
        selected_index: 1,
        font_size: 20.0,
        text_color: Color::from_rgb8(240, 240, 240),
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: Color::from_rgb8(1, 2, 3),
        enabled: true,
        hovered: false,
        hovered_item: None,
        pressed: false,
        changed: false,
        is_open: false,
    }
}

#[test]
/// Verifies that selected_index resolves to the expected item text.
fn get_selected_text_returns_selected_item() {
    let combo_box = sample_combo_box();
    assert_eq!(layout::selected_text(&combo_box), "second");
}

#[test]
/// Verifies that out-of-range selected_index falls back to empty text.
fn get_selected_text_returns_empty_when_out_of_bounds() {
    let mut combo_box = sample_combo_box();
    combo_box.selected_index = 99;
    assert_eq!(layout::selected_text(&combo_box), "");
}

#[test]
/// Verifies pressed state color is prioritized over hovered state color.
fn change_color_prefers_pressed_over_hovered() {
    let mut combo_box = sample_combo_box();
    combo_box.pressed = true;
    combo_box.hovered = true;
    assert_eq!(colors::border(&combo_box), Color::from_rgb8(89, 176, 255));
}

#[test]
/// Verifies disabled color is returned regardless of other interaction states.
fn change_color_uses_disabled_color_when_disabled() {
    let mut combo_box = sample_combo_box();
    combo_box.enabled = false;
    combo_box.pressed = true;
    combo_box.hovered = true;
    assert_eq!(colors::border(&combo_box), Color::from_rgb8(86, 92, 101));
}

#[test]
/// Verifies hovered color is selected when enabled and not pressed.
fn change_color_uses_hovered_color_when_hovered_only() {
    let mut combo_box = sample_combo_box();
    combo_box.hovered = true;
    assert_eq!(colors::border(&combo_box), Color::from_rgb8(124, 177, 230));
}

#[test]
/// Verifies baseline y-coordinate formula output for a fixed input.
fn baseline_y_matches_expected_formula() {
    let rect = Rect::new(10.0, 20.0, 210.0, 60.0);
    let font_size = 20.0;
    let y = layout::baseline_y(rect, font_size);
    assert_eq!(y, 47.0);
}

#[test]
/// Highlights hovered dropdown item over selected/default backgrounds.
fn dropdown_item_bg_prioritizes_hovered_item() {
    let mut combo_box = sample_combo_box();
    combo_box.hovered_item = Some(1);
    combo_box.selected_index = 1;
    assert_eq!(
        colors::dropdown_item_bg(&combo_box, 1),
        Color::from_rgb8(63, 80, 102)
    );
}

#[test]
/// Uses selected background when not hovered.
fn dropdown_item_bg_uses_selected_color() {
    let mut combo_box = sample_combo_box();
    combo_box.selected_index = 0;
    assert_eq!(
        colors::dropdown_item_bg(&combo_box, 0),
        Color::from_rgb8(46, 64, 86)
    );
}

#[test]
fn render_and_overlay_are_callable_for_open_and_closed_states() {
    let mut scene = Scene::new();
    let mut combo_box = sample_combo_box();
    render(&mut scene, None, &combo_box);
    render_dropdown_overlay(&mut scene, None, &combo_box);

    if let Some(font) = load_default_font() {
        render(&mut scene, Some(&font), &combo_box);
        combo_box.is_open = true;
        combo_box.hovered_item = Some(0);
        render_dropdown_overlay(&mut scene, Some(&font), &combo_box);
    }
}

#[test]
fn draw_text_helpers_are_callable() {
    let Some(font) = load_default_font() else {
        return;
    };
    let mut scene = Scene::new();
    let combo_box = sample_combo_box();
    text::draw_text_run_at(
        &mut scene,
        &font,
        vec![Glyph {
            id: 1,
            x: 0.0,
            y: 0.0,
        }],
        10.0,
        combo_box.rect,
        combo_box.font_size,
        Color::from_rgb8(255, 255, 255),
    );
    text::draw_selected_text(
        &mut scene,
        &font,
        vec![Glyph {
            id: 2,
            x: 0.0,
            y: 0.0,
        }],
        &combo_box,
    );
}
