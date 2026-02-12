use vello::Scene;
use vello::kurbo::{Affine, Circle, Stroke};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::RadioButtonNode;
use crate::widget::text::{draw_text_run, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, radio_button: &RadioButtonNode) {
    let indicator_size = (radio_button.rect.height() - 8.0).clamp(14.0, 24.0);
    let indicator_radius = indicator_size * 0.5;
    let center_x = radio_button.rect.x0 + 2.0 + indicator_radius;
    let center_y = radio_button.rect.y0 + radio_button.rect.height() * 0.5;
    let outer_circle = Circle::new((center_x, center_y), indicator_radius);

    let outer_bg = if !radio_button.enabled {
        Color::from_rgb8(43, 47, 53)
    } else if radio_button.pressed {
        Color::from_rgb8(45, 129, 205)
    } else if radio_button.hovered {
        Color::from_rgb8(53, 141, 221)
    } else {
        Color::from_rgb8(36, 42, 50)
    };

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        outer_bg,
        None,
        &outer_circle,
    );
    scene.stroke(
        &Stroke::new(1.0),
        Affine::IDENTITY,
        if radio_button.enabled {
            Color::from_rgb8(130, 145, 163)
        } else {
            Color::from_rgb8(88, 94, 102)
        },
        None,
        &outer_circle,
    );

    if radio_button.selected {
        let inner_radius = indicator_radius * 0.45;
        let inner_circle = Circle::new((center_x, center_y), inner_radius);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            if radio_button.enabled {
                Color::from_rgb8(240, 246, 255)
            } else {
                Color::from_rgb8(167, 173, 181)
            },
            None,
            &inner_circle,
        );
    }

    let Some(font) = font else {
        return;
    };
    let Some(text) = radio_button.text.as_deref() else {
        return;
    };
    let Some((glyphs, _)) = layout_text(font, text, radio_button.font_size) else {
        return;
    };
    let text_x = center_x + indicator_radius + 10.0;
    let baseline_y = radio_button.rect.y0
        + radio_button.rect.height() * 0.5
        + radio_button.font_size as f64 * 0.35;
    draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        radio_button.font_size,
        if radio_button.enabled {
            radio_button.text_color
        } else {
            Color::from_rgb8(146, 152, 160)
        },
    );
}
