use vello::Glyph;
use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
use crate::theme::color;
use crate::widget::text;

/// Renders single-line label text at the label rectangle origin.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some((font, glyphs, baseline_y)) = layout_for_label(font, label) else {
        return;
    };

    text::draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        resolve_label_text_color(label),
    );
}

fn layout_for_label<'a>(
    font: Option<&'a FontData>,
    label: &LabelNode,
) -> Option<(&'a FontData, Vec<Glyph>, f64)> {
    let font = font?;
    let (glyphs, _) = text::layout_text(font, &label.text, label.font_size)?;
    let baseline_y = label.rect.y0 + label.font_size as f64;
    Some((font, glyphs, baseline_y))
}

fn resolve_label_text_color(label: &LabelNode) -> vello::peniko::Color {
    if label.enabled {
        label.text_color
    } else {
        color::Neutral::tone_142_148_156()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(resolve_label_text_color(&enabled), enabled.text_color);
        assert_eq!(
            resolve_label_text_color(&disabled),
            color::Neutral::tone_142_148_156()
        );
    }

    #[test]
    fn layout_for_label_returns_none_without_font() {
        let label = sample_label(true);
        assert!(layout_for_label(None, &label).is_none());
    }

    #[test]
    fn layout_for_label_with_real_font_computes_baseline() {
        let Some(font) = load_default_font() else {
            return;
        };
        let label = sample_label(true);
        let Some((_, _glyphs, baseline_y)) = layout_for_label(Some(&font), &label) else {
            return;
        };
        assert_eq!(baseline_y, label.rect.y0 + label.font_size as f64);
    }
}
