use vello::Scene;
use vello::kurbo::{Affine, Circle};
use vello::peniko::Fill;

use super::constants;

pub(super) struct IndicatorGeometry {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub outer_circle: Circle,
}

pub(super) fn indicator_size(height: f64) -> f64 {
    (height - constants::INDICATOR_SIZE_OFFSET)
        .clamp(constants::INDICATOR_SIZE_MIN, constants::INDICATOR_SIZE_MAX)
}

pub(super) fn indicator_geometry(rect: vello::kurbo::Rect) -> IndicatorGeometry {
    let indicator_size: f64 = indicator_size(rect.height());
    let radius: f64 = indicator_size * constants::OUTER_RADIUS_RATIO;
    let center_x: f64 = rect.x0 + constants::INDICATOR_X_OFFSET + radius;
    let center_y: f64 = rect.y0 + rect.height() * constants::BASELINE_VERTICAL_RATIO;
    let outer_circle: Circle = Circle::new((center_x, center_y), radius);

    IndicatorGeometry {
        center_x,
        center_y,
        radius,
        outer_circle,
    }
}

pub(super) fn draw_selected_dot(
    scene: &mut Scene,
    radio_button: &crate::retained::node::RadioButtonNode,
    geometry: &IndicatorGeometry,
) {
    let inner_radius: f64 = geometry.radius * constants::INNER_RADIUS_RATIO;
    let inner_circle: Circle = Circle::new((geometry.center_x, geometry.center_y), inner_radius);

    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        super::colors::inner_dot(radio_button),
        None,
        &inner_circle,
    );
}
