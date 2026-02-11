use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill, FontData};

use crate::retained::node::ComboBoxNode;
use crate::widget::text::{draw_text_run, layout_text};

pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, combo_box: &ComboBoxNode) {
    let bg = RoundedRect::from_rect(combo_box.rect, 8.0);
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        combo_box.bg_color,
        None,
        &bg,
    );

    let border_color = if combo_box.pressed {
        Color::from_rgb8(89, 176, 255)
    } else if combo_box.hovered {
        Color::from_rgb8(124, 177, 230)
    } else {
        combo_box.border_color
    };
    scene.stroke(
        &vello::kurbo::Stroke::new(1.0),
        Affine::IDENTITY,
        border_color,
        None,
        &bg,
    );

    let Some(font) = font else {
        return;
    };

    let selected_text = combo_box
        .items
        .get(combo_box.selected_index)
        .map(String::as_str)
        .unwrap_or("");

    if let Some((glyphs, _)) = layout_text(font, selected_text, combo_box.font_size) {
        let text_x = combo_box.rect.x0 + 12.0;
        let baseline_y =
            combo_box.rect.y0 + combo_box.rect.height() * 0.5 + combo_box.font_size as f64 * 0.35;
        draw_text_run(
            scene,
            font,
            glyphs,
            text_x,
            baseline_y,
            combo_box.font_size,
            combo_box.text_color,
        );
    }

    let arrow = if combo_box.is_open { "^" } else { "v" };
    if let Some((glyphs, arrow_w)) = layout_text(font, arrow, combo_box.font_size * 0.85) {
        let arrow_x = combo_box.rect.x1 - arrow_w as f64 - 12.0;
        let arrow_y = combo_box.rect.y0
            + combo_box.rect.height() * 0.5
            + (combo_box.font_size * 0.85) as f64 * 0.35;
        draw_text_run(
            scene,
            font,
            glyphs,
            arrow_x,
            arrow_y,
            combo_box.font_size * 0.85,
            Color::from_rgb8(186, 196, 210),
        );
    }
}

pub(super) fn render_overlay(scene: &mut Scene, font: Option<&FontData>, combo_box: &ComboBoxNode) {
    if !combo_box.is_open || combo_box.items.is_empty() {
        return;
    }
    let Some(font) = font else {
        return;
    };

    let item_height = combo_box.rect.height();
    for (index, item) in combo_box.items.iter().enumerate() {
        let y0 = combo_box.rect.y1 + item_height * index as f64;
        let item_rect = Rect::new(combo_box.rect.x0, y0, combo_box.rect.x1, y0 + item_height);
        let item_bg = RoundedRect::from_rect(item_rect, 0.0);

        let bg_color = if combo_box.hovered_item == Some(index) {
            Color::from_rgb8(63, 80, 102)
        } else if combo_box.selected_index == index {
            Color::from_rgb8(46, 64, 86)
        } else {
            combo_box.bg_color
        };

        scene.fill(Fill::NonZero, Affine::IDENTITY, bg_color, None, &item_bg);
        scene.stroke(
            &vello::kurbo::Stroke::new(1.0),
            Affine::IDENTITY,
            combo_box.border_color,
            None,
            &item_bg,
        );

        if let Some((glyphs, _)) = layout_text(font, item, combo_box.font_size) {
            let text_x = item_rect.x0 + 12.0;
            let baseline_y =
                item_rect.y0 + item_rect.height() * 0.5 + combo_box.font_size as f64 * 0.35;
            draw_text_run(
                scene,
                font,
                glyphs,
                text_x,
                baseline_y,
                combo_box.font_size,
                combo_box.text_color,
            );
        }
    }
}
