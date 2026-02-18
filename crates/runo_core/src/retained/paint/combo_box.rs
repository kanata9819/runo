use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Color, Fill, FontData};
use vello::{Glyph, Scene};

use crate::retained::node::ComboBoxNode;
use crate::widget::text;

const TEXT_HORIZONTAL_PADDING: f64 = 12.0;
const BASELINE_VERTICAL_RATIO: f64 = 0.5;
const BASELINE_FONT_OFFSET_RATIO: f64 = 0.35;
const ARROW_FONT_SCALE: f32 = 0.85;
const COMBO_BOX_CORNER_RADIUS: f64 = 8.0;
const ITEM_CORNER_RADIUS: f64 = 0.0;
const BORDER_STROKE_WIDTH: f64 = 1.0;

/// Returns the combo box border color based on enable/press/hover state priority.
fn change_color(combo_box: &ComboBoxNode) -> AlphaColor<Srgb> {
    if !combo_box.enabled {
        Color::from_rgb8(86, 92, 101)
    } else if combo_box.pressed {
        Color::from_rgb8(89, 176, 255)
    } else if combo_box.hovered {
        Color::from_rgb8(124, 177, 230)
    } else {
        combo_box.border_color
    }
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
    rect.y0
        + rect.height() * BASELINE_VERTICAL_RATIO
        + font_size as f64 * BASELINE_FONT_OFFSET_RATIO
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
    draw_text_run_at(
        scene,
        font,
        glyphs,
        text_x,
        combo_box.rect,
        combo_box.font_size,
        if combo_box.enabled {
            combo_box.text_color
        } else {
            Color::from_rgb8(147, 153, 161)
        },
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
            Color::from_rgb8(45, 49, 55)
        },
        None,
        &bg,
    );

    let border_color = change_color(combo_box);
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
        draw_text_run_at(
            scene,
            font,
            glyphs,
            arrow_x,
            combo_box.rect,
            combo_box.font_size * ARROW_FONT_SCALE,
            if combo_box.enabled {
                Color::from_rgb8(186, 196, 210)
            } else {
                Color::from_rgb8(141, 147, 154)
            },
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
            Color::from_rgb8(63, 80, 102)
        } else if combo_box.selected_index == index {
            Color::from_rgb8(46, 64, 86)
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
            let baseline_y = item_rect.y0
                + item_rect.height() * BASELINE_VERTICAL_RATIO
                + combo_box.font_size as f64 * BASELINE_FONT_OFFSET_RATIO;
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(get_selected_text(&combo_box), "second");
    }

    #[test]
    /// Verifies that out-of-range selected_index falls back to empty text.
    fn get_selected_text_returns_empty_when_out_of_bounds() {
        let mut combo_box = sample_combo_box();
        combo_box.selected_index = 99;
        assert_eq!(get_selected_text(&combo_box), "");
    }

    #[test]
    /// Verifies pressed state color is prioritized over hovered state color.
    fn change_color_prefers_pressed_over_hovered() {
        let mut combo_box = sample_combo_box();
        combo_box.pressed = true;
        combo_box.hovered = true;
        assert_eq!(change_color(&combo_box), Color::from_rgb8(89, 176, 255));
    }

    #[test]
    /// Verifies disabled color is returned regardless of other interaction states.
    fn change_color_uses_disabled_color_when_disabled() {
        let mut combo_box = sample_combo_box();
        combo_box.enabled = false;
        combo_box.pressed = true;
        combo_box.hovered = true;
        assert_eq!(change_color(&combo_box), Color::from_rgb8(86, 92, 101));
    }

    #[test]
    /// Verifies baseline y-coordinate formula output for a fixed input.
    fn baseline_y_matches_expected_formula() {
        let rect = Rect::new(10.0, 20.0, 210.0, 60.0);
        let font_size = 20.0;
        let y = baseline_y(rect, font_size);
        assert_eq!(y, 47.0);
    }
}
