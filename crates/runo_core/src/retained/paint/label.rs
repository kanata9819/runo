use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
use crate::widget::text::{draw_text_run, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some(font) = font else {
        return;
    };
    let Some((glyphs, _)) = layout_text(font, &label.text, label.font_size) else {
        return;
    };
    let baseline_y = label.rect.y0 + label.font_size as f64;
    draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        label.text_color,
    );
}
