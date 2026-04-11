use super::*;
use crate::font::load_default_font;
use vello::kurbo::Rect;

fn sample_label(enabled: bool) -> LabelNode {
    LabelNode {
        rect: Rect::new(0.0, 0.0, 100.0, 30.0),
        text: "hello".to_string(),
        font_size: 18.0,
        text_color: vello::peniko::Color::from_rgb8(255, 255, 255),
        enabled,
    }
}

#[test]
fn render_returns_early_when_font_is_missing() {
    let mut scene = Scene::new();
    let label = sample_label(true);
    render(&mut scene, None, &label);
}

#[test]
fn resolve_label_text_color_switches_by_enabled_state() {
    let enabled = sample_label(true);
    let disabled = sample_label(false);
    assert_eq!(colors::text_color(&enabled), enabled.text_color);
    assert_eq!(
        colors::text_color(&disabled),
        color::Neutral::tone_142_148_156()
    );
}

#[test]
fn layout_for_label_returns_none_without_font() {
    let label = sample_label(true);
    assert!(layout::label_glyphs(None, &label).is_none());
}

#[test]
fn layout_for_label_with_real_font_computes_baseline() {
    let Some(font) = load_default_font() else {
        return;
    };
    let label = sample_label(true);
    let Some((_font, _glyphs)) = layout::label_glyphs(Some(&font), &label) else {
        return;
    };
    let baseline_y: f64 = layout::baseline_y(&label);
    assert_eq!(baseline_y, label.rect.y0 + label.font_size as f64);
}
