use vello::Scene;
use vello::kurbo::{Affine, Line, RoundedRect, Stroke};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};

use super::interaction_color;
use super::text_baseline;
use crate::retained::node::CheckboxNode;
use crate::theme::color;
use crate::widget::text;

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/checkbox.rs"]
mod tests;

const INDICATOR_X_OFFSET: f64 = 2.0;
const INDICATOR_CORNER_RADIUS: f64 = 4.0;
const INDICATOR_BORDER_WIDTH: f64 = 1.0;
const CHECK_STROKE_WIDTH: f64 = 2.2;
const CHECK_X0_RATIO: f64 = 0.22;
const CHECK_Y0_RATIO: f64 = 0.56;
const CHECK_X1_RATIO: f64 = 0.44;
const CHECK_Y1_RATIO: f64 = 0.76;
const CHECK_X2_RATIO: f64 = 0.80;
const CHECK_Y2_RATIO: f64 = 0.28;
const LABEL_TEXT_SPACING: f64 = 10.0;
const BASELINE_VERTICAL_RATIO: f64 = 0.5;
const INDICATOR_SIZE_OFFSET: f64 = 8.0;
const INDICATOR_SIZE_MIN: f64 = 14.0;
const INDICATOR_SIZE_MAX: f64 = 24.0;

/// Renders checkbox indicator, optional check mark, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, checkbox: &CheckboxNode) {
    let indicator_size = indicator_size(checkbox.rect.height());
    let indicator_x = checkbox.rect.x0 + INDICATOR_X_OFFSET;
    let indicator_y =
        checkbox.rect.y0 + (checkbox.rect.height() - indicator_size) * BASELINE_VERTICAL_RATIO;
    let indicator_rect = RoundedRect::new(
        indicator_x,
        indicator_y,
        indicator_x + indicator_size,
        indicator_y + indicator_size,
        INDICATOR_CORNER_RADIUS,
    );

    let bg_color = indicator_bg_color(checkbox);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        bg_color,
        None,
        &indicator_rect,
    );

    let color = if checkbox.enabled {
        color::Neutral::tone_130_145_163()
    } else {
        color::Neutral::tone_88_94_102()
    };

    scene.stroke(
        &Stroke::new(INDICATOR_BORDER_WIDTH),
        Affine::IDENTITY,
        color,
        None,
        &indicator_rect,
    );

    if checkbox.checked {
        let check_color = if checkbox.enabled {
            color::SoftWhite::tone_240_246_255()
        } else {
            color::Neutral::tone_167_173_181()
        };
        let x0 = indicator_x + indicator_size * CHECK_X0_RATIO;
        let y0 = indicator_y + indicator_size * CHECK_Y0_RATIO;
        let x1 = indicator_x + indicator_size * CHECK_X1_RATIO;
        let y1 = indicator_y + indicator_size * CHECK_Y1_RATIO;
        let x2 = indicator_x + indicator_size * CHECK_X2_RATIO;
        let y2 = indicator_y + indicator_size * CHECK_Y2_RATIO;

        scene.stroke(
            &Stroke::new(CHECK_STROKE_WIDTH),
            Affine::IDENTITY,
            check_color,
            None,
            &Line::new((x0, y0), (x1, y1)),
        );

        scene.stroke(
            &Stroke::new(CHECK_STROKE_WIDTH),
            Affine::IDENTITY,
            check_color,
            None,
            &Line::new((x1, y1), (x2, y2)),
        );
    }

    let Some(font) = font else {
        return;
    };

    let Some(text) = checkbox.text.as_deref() else {
        return;
    };

    let Some((glyphs, _)) = text::layout_text(font, text, checkbox.font_size) else {
        return;
    };

    let text_x = indicator_x + indicator_size + LABEL_TEXT_SPACING;
    let baseline_y = text_baseline::centered(checkbox.rect, checkbox.font_size);
    let color = if checkbox.enabled {
        checkbox.text_color
    } else {
        color::Neutral::tone_146_152_160()
    };

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        checkbox.font_size,
        color,
    );
}

/// Computes checkbox indicator square size from widget height with clamped bounds.
fn indicator_size(height: f64) -> f64 {
    (height - INDICATOR_SIZE_OFFSET).clamp(INDICATOR_SIZE_MIN, INDICATOR_SIZE_MAX)
}

/// Resolves indicator background color from enabled/pressed/hovered/checked state priority.
fn indicator_bg_color(checkbox: &CheckboxNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        checkbox.enabled,
        checkbox.pressed,
        checkbox.hovered,
        color::Neutral::tone_43_47_53(),
        color::AccentBlue::tone_45_129_205(),
        color::AccentBlue::tone_53_141_221(),
        if checkbox.checked {
            color::AccentBlue::tone_50_144_229()
        } else {
            color::Neutral::tone_36_42_50()
        },
    )
}
