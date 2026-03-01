use vello::kurbo::Rect;

const CENTER_RATIO: f64 = 0.5;
const BASELINE_FONT_OFFSET_RATIO: f64 = 0.35;

/// Baseline for vertically centered control labels (button/checkbox/radio/combo).
#[inline]
pub(super) fn centered(rect: Rect, font_size: f32) -> f64 {
    rect.y0 + rect.height() * CENTER_RATIO + font_size as f64 * BASELINE_FONT_OFFSET_RATIO
}

/// Baseline for top-aligned text blocks (label/slider caption/value).
#[inline]
pub(super) fn top_aligned(rect: Rect, font_size: f32) -> f64 {
    rect.y0 + font_size as f64
}
