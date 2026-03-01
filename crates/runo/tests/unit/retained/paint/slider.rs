use super::*;
use crate::font::load_default_font;
use crate::retained::node::SliderNode;
use vello::kurbo::Rect;
use vello::peniko::Color;

fn sample_slider() -> SliderNode {
    SliderNode {
        rect: Rect::new(0.0, 0.0, 220.0, 48.0),
        min: 0.0,
        max: 1.0,
        value: 0.5,
        step: Some(0.1),
        text: Some("Volume".to_string()),
        font_size: 16.0,
        text_color: Color::from_rgb8(240, 240, 240),
        enabled: true,
        hovered: false,
        pressed: false,
        changed: false,
    }
}

#[test]
/// Maps midpoint value to midpoint ratio.
fn value_ratio_maps_midpoint() {
    assert_eq!(value_ratio(50.0, 0.0, 100.0), 0.5);
}

#[test]
/// Clamps values below minimum to zero.
fn value_ratio_clamps_below_min() {
    assert_eq!(value_ratio(-10.0, 0.0, 100.0), 0.0);
}

#[test]
/// Clamps values above maximum to one.
fn value_ratio_clamps_above_max() {
    assert_eq!(value_ratio(110.0, 0.0, 100.0), 1.0);
}

#[test]
/// Returns zero ratio when range span is effectively zero.
fn value_ratio_returns_zero_for_degenerate_range() {
    assert_eq!(value_ratio(5.0, 1.0, 1.0), 0.0);
}

#[test]
fn render_and_internal_draw_helpers_are_callable() {
    let mut scene = Scene::new();
    let slider = sample_slider();
    render(&mut scene, None, &slider);
    let track_rect = RoundedRect::new(12.0, 20.0, 200.0, 26.0, 3.0);
    draw_track(&mut scene, &slider, &track_rect);
    draw_active_fill(&mut scene, &slider, 12.0, 23.0, 6.0, 120.0);
    draw_thumb(&mut scene, &slider, 120.0, 23.0);

    if let Some(font) = load_default_font() {
        render(&mut scene, Some(&font), &slider);
        draw_optional_label(&mut scene, &font, &slider, 12.0);
        draw_value_text(&mut scene, &font, &slider, 12.0);
    }
}
