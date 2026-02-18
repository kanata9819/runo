use vello::Scene;
use vello::kurbo::{Affine, Circle, RoundedRect, Stroke};
use vello::peniko::{Fill, FontData};

use crate::retained::node::SliderNode;
use crate::theme::color;
use crate::widget::text;

const TRACK_HEIGHT: f64 = 6.0;
const TRACK_HORIZONTAL_PADDING: f64 = 12.0;
const TRACK_VERTICAL_RATIO: f64 = 0.62;
const HALF_RATIO: f64 = 0.5;
const TRACK_CORNER_RADIUS: f64 = 3.0;
const THUMB_RADIUS: f64 = 8.0;
const THUMB_BORDER_WIDTH: f64 = 1.0;
const VALUE_DECIMALS: usize = 2;

/// Renders slider track, active fill, thumb, optional label, and current numeric value.
pub(super) fn render(scene: &mut Scene, font: Option<&FontData>, slider: &SliderNode) {
    let track_h = TRACK_HEIGHT;
    let pad_x = TRACK_HORIZONTAL_PADDING;
    let track_x0 = slider.rect.x0 + pad_x;
    let track_x1 = slider.rect.x1 - pad_x;
    let track_y = slider.rect.y0 + slider.rect.height() * TRACK_VERTICAL_RATIO;
    let track_rect = RoundedRect::new(
        track_x0,
        track_y - track_h * HALF_RATIO,
        track_x1,
        track_y + track_h * HALF_RATIO,
        TRACK_CORNER_RADIUS,
    );

    let ratio = value_ratio(slider.value, slider.min, slider.max);
    let thumb_x = track_x0 + (track_x1 - track_x0) * ratio;

    draw_track(scene, slider, &track_rect);
    draw_active_fill(scene, slider, track_x0, track_y, track_h, thumb_x);
    draw_thumb(scene, slider, thumb_x, track_y);

    let Some(font) = font else {
        return;
    };

    draw_optional_label(scene, font, slider, pad_x);
    draw_value_text(scene, font, slider, pad_x);
}

/// Draws the slider background track.
fn draw_track(scene: &mut Scene, slider: &SliderNode, track_rect: &RoundedRect) {
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if slider.enabled {
            color::rgb(color::widget::SLIDER_TRACK_ENABLED)
        } else {
            color::rgb(color::widget::SLIDER_TRACK_DISABLED)
        },
        None,
        track_rect,
    );
}

/// Draws the active portion of the slider up to the thumb position.
fn draw_active_fill(
    scene: &mut Scene,
    slider: &SliderNode,
    track_x0: f64,
    track_y: f64,
    track_h: f64,
    thumb_x: f64,
) {
    let active_rect = RoundedRect::new(
        track_x0,
        track_y - track_h * HALF_RATIO,
        thumb_x,
        track_y + track_h * HALF_RATIO,
        TRACK_CORNER_RADIUS,
    );
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if !slider.enabled {
            color::rgb(color::widget::SLIDER_ACTIVE_DISABLED)
        } else if slider.pressed {
            color::rgb(color::widget::SLIDER_ACTIVE_PRESSED)
        } else if slider.hovered {
            color::rgb(color::widget::SLIDER_ACTIVE_HOVER)
        } else {
            color::rgb(color::widget::SLIDER_ACTIVE_ENABLED)
        },
        None,
        &active_rect,
    );
}

/// Draws the slider thumb circle.
fn draw_thumb(scene: &mut Scene, slider: &SliderNode, thumb_x: f64, track_y: f64) {
    let thumb = Circle::new((thumb_x, track_y), THUMB_RADIUS);
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        if slider.enabled {
            color::rgb(color::widget::SLIDER_THUMB_ENABLED)
        } else {
            color::rgb(color::widget::SLIDER_THUMB_DISABLED)
        },
        None,
        &thumb,
    );
    scene.stroke(
        &Stroke::new(THUMB_BORDER_WIDTH),
        Affine::IDENTITY,
        color::rgb(color::widget::SLIDER_THUMB_BORDER),
        None,
        &thumb,
    );
}

/// Draws the optional left-aligned slider label text.
fn draw_optional_label(scene: &mut Scene, font: &FontData, slider: &SliderNode, pad_x: f64) {
    if let Some(text) = slider.text.as_deref()
        && let Some((glyphs, _)) = text::layout_text(font, text, slider.font_size)
    {
        let baseline_y = slider.rect.y0 + slider.font_size as f64;
        text::draw_text_run(
            scene,
            font,
            glyphs,
            slider.rect.x0 + pad_x,
            baseline_y,
            slider.font_size,
            if slider.enabled {
                slider.text_color
            } else {
                color::rgb(color::widget::SLIDER_TEXT_DISABLED)
            },
        );
    }
}

/// Draws the right-aligned numeric value text.
fn draw_value_text(scene: &mut Scene, font: &FontData, slider: &SliderNode, pad_x: f64) {
    let value_text = format!("{:.*}", VALUE_DECIMALS, slider.value);
    if let Some((glyphs, w)) = text::layout_text(font, &value_text, slider.font_size) {
        let baseline_y = slider.rect.y0 + slider.font_size as f64;
        text::draw_text_run(
            scene,
            font,
            glyphs,
            slider.rect.x1 - pad_x - w as f64,
            baseline_y,
            slider.font_size,
            if slider.enabled {
                slider.text_color
            } else {
                color::rgb(color::widget::SLIDER_TEXT_DISABLED)
            },
        );
    }
}

/// Converts slider value in `[min, max]` into a clamped ratio in `[0.0, 1.0]`.
fn value_ratio(value: f64, min: f64, max: f64) -> f64 {
    let span = (max - min).abs();
    if span <= f64::EPSILON {
        return 0.0;
    }
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::value_ratio;

    #[test]
    /// Maps midpoint value to midpoint ratio.
    fn value_ratio_maps_midpoint() {
        assert_eq!(value_ratio(50.0, 0.0, 100.0), 0.5);
    }

    #[test]
    /// Clamps values below minimum to zero.
    fn value_ratio_clamps_below_min() {
        assert_eq!(value_ratio(-10.0, 0.0, 100.0), 0.0);
    }

    #[test]
    /// Clamps values above maximum to one.
    fn value_ratio_clamps_above_max() {
        assert_eq!(value_ratio(110.0, 0.0, 100.0), 1.0);
    }

    #[test]
    /// Returns zero ratio when range span is effectively zero.
    fn value_ratio_returns_zero_for_degenerate_range() {
        assert_eq!(value_ratio(5.0, 1.0, 1.0), 0.0);
    }
}
