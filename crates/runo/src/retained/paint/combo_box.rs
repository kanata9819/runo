use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};
use vello::{Glyph, Scene};

use super::interaction_color;
use super::text_baseline;
use crate::retained::node::ComboBoxNode;
use crate::theme::color;
use crate::widget::text;

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/combo_box.rs"]
mod tests;

const TEXT_HORIZONTAL_PADDING: f64 = 12.0;
const ARROW_FONT_SCALE: f32 = 0.85;
const COMBO_BOX_CORNER_RADIUS: f64 = 8.0;
const ITEM_CORNER_RADIUS: f64 = 0.0;
const BORDER_STROKE_WIDTH: f64 = 1.0;

/// Returns the combo box border color based on enable/press/hover state priority.
fn indicator_bg_color(combo_box: &ComboBoxNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        combo_box.enabled,
        combo_box.pressed,
        combo_box.hovered,
        color::Neutral::tone_86_92_101(),
        color::AccentBlue::tone_89_176_255(),
        color::AccentBlue::tone_124_177_230(),
        combo_box.border_color,
    )
}

/// Returns the currently selected item text, or an empty string when index is out of range.
fn get_selected_text(combo_box: &ComboBoxNode) -> &str {
    combo_box
        .items
        .get(combo_box.selected_index)
        .map(String::as_str)
        .unwrap_or("")
}

#[inline]
/// Computes the text baseline y-coordinate for a given rect and font size.
fn baseline_y(rect: Rect, font_size: f32) -> f64 {
    text_baseline::centered(rect, font_size)
}

#[inline]
/// Draws a pre-laid-out glyph run at x and computed baseline within the provided rect.
fn draw_text_run_at(
    scene: &mut Scene,
    font: &FontData,
    glyphs: Vec<Glyph>,
    x: f64,
    rect: Rect,
    font_size: f32,
    color: AlphaColor<Srgb>,
) {
    text::draw_text_run(
        scene,
        font,
        glyphs,
        x,
        baseline_y(rect, font_size),
        font_size,
        color,
    );
}

/// Draws the selected value text for the main combo box body with state-aware text color.
fn draw_text_run<'a>(
    scene: &'a mut Scene,
    font: &'a FontData,
    glyphs: Vec<Glyph>,
    combo_box: &'a ComboBoxNode,
) {
    let text_x = combo_box.rect.x0 + TEXT_HORIZONTAL_PADDING;
    let color = if combo_box.enabled {
        combo_box.text_color
    } else {
        color::Neutral::tone_147_153_161()
    };

    draw_text_run_at(
        scene,
        font,
        glyphs,
        text_x,
        combo_box.rect,
        combo_box.font_size,
        color,
    );
}

/// Renders the closed combo box body, border, selected text, and open/close arrow.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, combo_box: &ComboBoxNode) {
    let bg = RoundedRect::from_rect(combo_box.rect, COMBO_BOX_CORNER_RADIUS);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if combo_box.enabled {
            combo_box.bg_color
        } else {
            color::Neutral::tone_45_49_55()
        },
        None,
        &bg,
    );

    let border_color = indicator_bg_color(combo_box);
    scene.stroke(
        &vello::kurbo::Stroke::new(BORDER_STROKE_WIDTH),
        Affine::IDENTITY,
        border_color,
        None,
        &bg,
    );

    let Some(font) = font else {
        return;
    };

    let selected_text = get_selected_text(combo_box);
    if let Some((glyphs, _)) = text::layout_text(font, selected_text, combo_box.font_size) {
        draw_text_run(scene, font, glyphs, combo_box);
    }

    let arrow = if combo_box.is_open { "^" } else { "v" };
    if let Some((glyphs, arrow_w)) =
        text::layout_text(font, arrow, combo_box.font_size * ARROW_FONT_SCALE)
    {
        let arrow_x = combo_box.rect.x1 - arrow_w as f64 - TEXT_HORIZONTAL_PADDING;
        let color = if combo_box.enabled {
            color::SoftWhite::tone_186_196_210()
        } else {
            color::Neutral::tone_141_147_154()
        };

        draw_text_run_at(
            scene,
            font,
            glyphs,
            arrow_x,
            combo_box.rect,
            combo_box.font_size * ARROW_FONT_SCALE,
            color,
        );
    }
}

/// Renders the dropdown item list overlay when the combo box is open and enabled.
pub(super) fn render_dropdown_overlay(
    scene: &mut Scene,
    font: Option<&FontData>,
    combo_box: &ComboBoxNode,
) {
    if !combo_box.enabled || !combo_box.is_open || combo_box.items.is_empty() {
        return;
    }

    let Some(font) = font else {
        return;
    };

    let item_height = combo_box.rect.height();
    for (index, item) in combo_box.items.iter().enumerate() {
        let y0 = combo_box.rect.y1 + item_height * index as f64;
        let item_rect = Rect::new(combo_box.rect.x0, y0, combo_box.rect.x1, y0 + item_height);
        let item_bg = RoundedRect::from_rect(item_rect, ITEM_CORNER_RADIUS);

        let bg_color = if combo_box.hovered_item == Some(index) {
            color::Neutral::tone_63_80_102()
        } else if combo_box.selected_index == index {
            color::Neutral::tone_46_64_86()
        } else {
            combo_box.bg_color
        };

        scene.fill(Fill::NonZero, Affine::IDENTITY, bg_color, None, &item_bg);
        scene.stroke(
            &vello::kurbo::Stroke::new(BORDER_STROKE_WIDTH),
            Affine::IDENTITY,
            combo_box.border_color,
            None,
            &item_bg,
        );

        if let Some((glyphs, _)) = text::layout_text(font, item, combo_box.font_size) {
            let text_x = item_rect.x0 + TEXT_HORIZONTAL_PADDING;
            let baseline_y = baseline_y(item_rect, combo_box.font_size);
            text::draw_text_run(
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
