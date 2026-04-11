use crate::retained::node::TextBoxNode;

/// Precomputed geometry used by `TextBox` painting routines.
///
/// Values here are shared between text rendering and caret rendering so
/// those paths stay consistent without duplicating coordinate math.
#[derive(Clone, Copy)]
pub(super) struct TextMetrics {
    /// Drawing origin on X axis for text runs (includes horizontal scroll).
    pub(super) text_x: f64,
    /// Baseline Y for the first line (includes vertical scroll).
    pub(super) first_line_baseline: f64,
    /// Distance between baselines of adjacent lines.
    pub(super) line_height: f64,
    /// Left edge of the inner content area.
    pub(super) inner_left: f64,
    /// Right edge of the inner content area.
    pub(super) inner_right: f64,
    /// Top edge of the inner content area.
    pub(super) inner_top: f64,
    /// Bottom edge of the inner content area.
    pub(super) inner_bottom: f64,
}

impl TextMetrics {
    /// Corner radius for the outer text box shape.
    pub(super) const BOX_CORNER_RADIUS: f64 = 8.0;
    /// Border stroke width for the text box outline.
    pub(super) const BORDER_STROKE_WIDTH: f64 = 1.0;
    /// Horizontal/vertical inset used by text and clip bounds.
    pub(super) const INNER_PADDING: f64 = 12.0;
    /// Line-height multiplier applied to font size.
    pub(super) const LINE_HEIGHT_RATIO: f64 = 1.35;
    /// Height of the horizontal scrollbar track.
    pub(super) const SCROLLBAR_TRACK_HEIGHT: f64 = 4.0;
    /// Offset from the bottom edge to scrollbar track baseline.
    pub(super) const SCROLLBAR_TRACK_BOTTOM_OFFSET: f64 = 6.0;
    /// Corner radius for scrollbar track and thumb.
    pub(super) const SCROLLBAR_CORNER_RADIUS: f64 = 2.0;
    /// Minimum width of scrollbar thumb for usability.
    pub(super) const SCROLLBAR_THUMB_MIN_WIDTH: f64 = 18.0;
    /// Minimum inner width used to avoid zero/negative geometry.
    pub(super) const MIN_INNER_WIDTH: f64 = 1.0;

    /// Computes shared coordinates and sizes for text and caret rendering.
    pub fn new(text_box: &TextBoxNode) -> Self {
        Self {
            text_x: text_box.rect.x0 + Self::INNER_PADDING - text_box.scroll_x,
            first_line_baseline: text_box.rect.y0
                + Self::INNER_PADDING
                + f64::from(text_box.font_size)
                - text_box.scroll_y,
            line_height: f64::from(text_box.font_size) * Self::LINE_HEIGHT_RATIO,
            inner_left: text_box.rect.x0 + Self::INNER_PADDING,
            inner_right: text_box.rect.x1 - Self::INNER_PADDING,
            inner_top: text_box.rect.y0 + Self::INNER_PADDING,
            inner_bottom: text_box.rect.y1 - Self::INNER_PADDING,
        }
    }
}
