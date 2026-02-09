use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::{ButtonNode, LabelNode, WidgetNode};
use crate::retained::state::RetainedState;
use crate::widget::text::{draw_text_run, layout_text};

impl RetainedState {
    pub(crate) fn render(&self, scene: &mut Scene, font: Option<&FontData>) {
        for id in &self.order {
            let Some(node) = self.widgets.get(id) else {
                continue;
            };
            match node {
                WidgetNode::Button(button) => render_button(scene, font, button),
                WidgetNode::Label(label) => render_label(scene, font, label),
            }
        }
    }
}

fn render_button(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let color = if button.pressed {
        Color::from_rgb8(31, 122, 205)
    } else if button.hovered {
        Color::from_rgb8(69, 160, 242)
    } else {
        Color::from_rgb8(50, 144, 229)
    };

    let rounded = RoundedRect::from_rect(button.rect, 10.0);
    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);

    let (Some(font), Some(text)) = (font, button.text.as_deref()) else {
        return;
    };
    let font_size = 18.0_f32;
    let Some((glyphs, total_advance)) = layout_text(font, text, font_size) else {
        return;
    };

    let text_x = button.rect.x0 + (button.rect.width() - total_advance as f64) * 0.5;
    let text_y = button.rect.y0 + button.rect.height() * 0.5 + font_size as f64 * 0.35;
    draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        text_y,
        font_size,
        button.text_color,
    );
}

fn render_label(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
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
