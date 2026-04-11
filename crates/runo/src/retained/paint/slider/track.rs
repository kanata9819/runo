use vello::Scene;
use vello::kurbo::{Affine, Circle, RoundedRect, Stroke};
use vello::peniko::Fill;

use crate::retained::node::SliderNode;

pub(super) fn draw_track(scene: &mut Scene, slider: &SliderNode, track_rect: &RoundedRect) {
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        super::colors::track(slider),
        None,
        track_rect,
    );
}

pub(super) fn draw_active_fill(
    scene: &mut Scene,
    slider: &SliderNode,
    track_x0: f64,
    track_y: f64,
    track_h: f64,
    thumb_x: f64,
) {
    let active_rect: RoundedRect = RoundedRect::new(
        track_x0,
        track_y - track_h * super::layout::HALF_RATIO,
        thumb_x,
        track_y + track_h * super::layout::HALF_RATIO,
        super::layout::TRACK_CORNER_RADIUS,
    );

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        super::colors::active_fill(slider),
        None,
        &active_rect,
    );
}

pub(super) fn draw_thumb(scene: &mut Scene, slider: &SliderNode, thumb_x: f64, track_y: f64) {
    let thumb: Circle = Circle::new((thumb_x, track_y), super::layout::THUMB_RADIUS);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        super::colors::thumb_fill(slider),
        None,
        &thumb,
    );

    scene.stroke(
        &Stroke::new(super::layout::THUMB_BORDER_WIDTH),
        Affine::IDENTITY,
        crate::theme::color::Neutral::tone_78_89_104(),
        None,
        &thumb,
    );
}
