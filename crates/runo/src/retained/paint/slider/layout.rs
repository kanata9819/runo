use vello::kurbo::RoundedRect;

use crate::retained::node::SliderNode;

pub(super) const TRACK_HEIGHT: f64 = 6.0;
pub(super) const TRACK_HORIZONTAL_PADDING: f64 = 12.0;
pub(super) const TRACK_VERTICAL_RATIO: f64 = 0.62;
pub(super) const HALF_RATIO: f64 = 0.5;
pub(super) const TRACK_CORNER_RADIUS: f64 = 3.0;
pub(super) const THUMB_RADIUS: f64 = 8.0;
pub(super) const THUMB_BORDER_WIDTH: f64 = 1.0;
pub(super) const VALUE_DECIMALS: usize = 2;

pub(super) struct SliderGeometry {
    pub pad_x: f64,
    pub track_y: f64,
    pub track_height: f64,
    pub track_x0: f64,
    pub track_rect: RoundedRect,
    pub thumb_x: f64,
}

pub(super) fn geometry(slider: &SliderNode) -> SliderGeometry {
    let track_height: f64 = TRACK_HEIGHT;
    let pad_x: f64 = TRACK_HORIZONTAL_PADDING;
    let track_x0: f64 = slider.rect.x0 + pad_x;
    let track_x1: f64 = slider.rect.x1 - pad_x;
    let track_y: f64 = slider.rect.y0 + slider.rect.height() * TRACK_VERTICAL_RATIO;
    let track_rect: RoundedRect = RoundedRect::new(
        track_x0,
        track_y - track_height * HALF_RATIO,
        track_x1,
        track_y + track_height * HALF_RATIO,
        TRACK_CORNER_RADIUS,
    );

    let ratio: f64 = value_ratio(slider.value, slider.min, slider.max);
    let thumb_x: f64 = track_x0 + (track_x1 - track_x0) * ratio;

    SliderGeometry {
        pad_x,
        track_y,
        track_height,
        track_x0,
        track_rect,
        thumb_x,
    }
}

/// Converts slider value in `[min, max]` into a clamped ratio in `[0.0, 1.0]`.
pub(super) fn value_ratio(value: f64, min: f64, max: f64) -> f64 {
    let span: f64 = (max - min).abs();

    if span <= f64::EPSILON {
        return 0.0;
    }

    ((value - min) / (max - min)).clamp(0.0, 1.0)
}
