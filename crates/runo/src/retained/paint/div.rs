use vello::Scene;
use vello::kurbo::{Affine, RoundedRect, Stroke};
use vello::peniko::Fill;

use crate::retained::node::DivNode;

/// Renders div background and border using retained geometry plus state overrides.
pub(super) fn render(scene: &mut Scene, div: &DivNode) {
    if !div.visible {
        return;
    }

    let rounded = RoundedRect::from_rect(div.rect, div.radius);

    if let Some(color) = div.bg_color {
        scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);
    }

    if let Some(color) = div.border_color {
        scene.stroke(
            &Stroke::new(div.border_width.max(0.0)),
            Affine::IDENTITY,
            color,
            None,
            &rounded,
        );
    }
}
