use vello::Scene;
use vello::kurbo::{Affine, Circle, Stroke};
use vello::peniko::color::{AlphaColor, Srgb};
use vello::peniko::{Fill, FontData};

use super::interaction_color;
use super::text_baseline;
use crate::retained::node::RadioButtonNode;
use crate::theme::color;
use crate::widget::text;

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/radio_button.rs"]
mod tests;

const INDICATOR_X_OFFSET: f64 = 2.0;
const INDICATOR_BORDER_WIDTH: f64 = 1.0;
const INDICATOR_SIZE_OFFSET: f64 = 8.0;
const INDICATOR_SIZE_MIN: f64 = 14.0;
const INDICATOR_SIZE_MAX: f64 = 24.0;
const OUTER_RADIUS_RATIO: f64 = 0.5;
const INNER_RADIUS_RATIO: f64 = 0.45;
const BASELINE_VERTICAL_RATIO: f64 = 0.5;
const LABEL_TEXT_SPACING: f64 = 10.0;

/// Renders radio button indicator, selected dot, and optional label text.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, radio_button: &RadioButtonNode) {
    let indicator_size = indicator_size(radio_button.rect.height());
    let indicator_radius = indicator_size * OUTER_RADIUS_RATIO;
    let center_x = radio_button.rect.x0 + INDICATOR_X_OFFSET + indicator_radius;
    let center_y = radio_button.rect.y0 + radio_button.rect.height() * BASELINE_VERTICAL_RATIO;
    let outer_circle = Circle::new((center_x, center_y), indicator_radius);

    let outer_bg = outer_bg_color(radio_button);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        outer_bg,
        None,
        &outer_circle,
    );

    scene.stroke(
        &Stroke::new(INDICATOR_BORDER_WIDTH),
        Affine::IDENTITY,
        if radio_button.enabled {
            color::Neutral::tone_130_145_163()
        } else {
            color::Neutral::tone_88_94_102()
        },
        None,
        &outer_circle,
    );

    if radio_button.selected {
        let inner_radius = indicator_radius * INNER_RADIUS_RATIO;
        let inner_circle = Circle::new((center_x, center_y), inner_radius);

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            if radio_button.enabled {
                color::SoftWhite::tone_240_246_255()
            } else {
                color::Neutral::tone_167_173_181()
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

    let Some((glyphs, _)) = text::layout_text(font, text, radio_button.font_size) else {
        return;
    };

    let text_x = center_x + indicator_radius + LABEL_TEXT_SPACING;
    let baseline_y = text_baseline::centered(radio_button.rect, radio_button.font_size);

    text::draw_text_run(
        scene,
        font,
        glyphs,
        text_x,
        baseline_y,
        radio_button.font_size,
        if radio_button.enabled {
            radio_button.text_color
        } else {
            color::Neutral::tone_146_152_160()
        },
    );
}

/// Computes radio indicator diameter from widget height with clamped bounds.
fn indicator_size(height: f64) -> f64 {
    (height - INDICATOR_SIZE_OFFSET).clamp(INDICATOR_SIZE_MIN, INDICATOR_SIZE_MAX)
}

/// Resolves radio outer indicator color from enabled/pressed/hovered state priority.
fn outer_bg_color(radio_button: &RadioButtonNode) -> AlphaColor<Srgb> {
    interaction_color::resolve_interaction_color(
        radio_button.enabled,
        radio_button.pressed,
        radio_button.hovered,
        color::Neutral::tone_43_47_53(),
        color::AccentBlue::tone_45_129_205(),
        color::AccentBlue::tone_53_141_221(),
        color::Neutral::tone_36_42_50(),
    )
}
