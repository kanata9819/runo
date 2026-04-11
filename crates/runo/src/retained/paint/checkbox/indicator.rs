use vello::Scene;
use vello::kurbo::{Affine, Line, RoundedRect, Stroke};

use super::constants;

pub(super) struct IndicatorGeometry {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub rect: RoundedRect,
}

pub(super) fn indicator_size(height: f64) -> f64 {
    (height - constants::INDICATOR_SIZE_OFFSET)
        .clamp(constants::INDICATOR_SIZE_MIN, constants::INDICATOR_SIZE_MAX)
}

pub(super) fn indicator_geometry(rect: vello::kurbo::Rect) -> IndicatorGeometry {
    let size: f64 = indicator_size(rect.height());
    let x: f64 = rect.x0 + constants::INDICATOR_X_OFFSET;
    let y: f64 = rect.y0 + (rect.height() - size) * constants::BASELINE_VERTICAL_RATIO;
    let rounded: RoundedRect =
        RoundedRect::new(x, y, x + size, y + size, constants::INDICATOR_CORNER_RADIUS);

    IndicatorGeometry {
        x,
        y,
        size,
        rect: rounded,
    }
}

pub(super) fn draw_check_mark(
    scene: &mut Scene,
    checkbox: &crate::retained::node::CheckboxNode,
    geometry: &IndicatorGeometry,
) {
    let x0: f64 = geometry.x + geometry.size * constants::CHECK_X0_RATIO;
    let y0: f64 = geometry.y + geometry.size * constants::CHECK_Y0_RATIO;
    let x1: f64 = geometry.x + geometry.size * constants::CHECK_X1_RATIO;
    let y1: f64 = geometry.y + geometry.size * constants::CHECK_Y1_RATIO;
    let x2: f64 = geometry.x + geometry.size * constants::CHECK_X2_RATIO;
    let y2: f64 = geometry.y + geometry.size * constants::CHECK_Y2_RATIO;

    scene.stroke(
        &Stroke::new(constants::CHECK_STROKE_WIDTH),
        Affine::IDENTITY,
        super::colors::check_mark(checkbox),
        None,
        &Line::new((x0, y0), (x1, y1)),
    );

    scene.stroke(
        &Stroke::new(constants::CHECK_STROKE_WIDTH),
        Affine::IDENTITY,
        super::colors::check_mark(checkbox),
        None,
        &Line::new((x1, y1), (x2, y2)),
    );
}
