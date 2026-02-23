use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
use crate::theme::color;
use crate::widget::text;

/// Renders single-line label text at the label rectangle origin.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some(font) = font else {
        return;
    };
    let Some((glyphs, _)) = text::layout_text(font, &label.text, label.font_size) else {
        return;
    };
    let baseline_y = label.rect.y0 + label.font_size as f64;

    text::draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        if label.enabled {
            label.text_color
        } else {
            color::Neutral::tone_142_148_156()
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use vello::kurbo::Rect;

    #[test]
    fn render_returns_early_when_font_is_missing() {
        let mut scene = Scene::new();
        let label = LabelNode {
            rect: Rect::new(0.0, 0.0, 100.0, 30.0),
            text: "hello".to_string(),
            font_size: 18.0,
            text_color: vello::peniko::Color::from_rgb8(255, 255, 255),
            enabled: true,
        };
        render(&mut scene, None, &label);
    }
}
