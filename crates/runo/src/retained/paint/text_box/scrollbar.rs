//! Horizontal scrollbar painter for `TextBox`.
//!
//! Terminology used in this module:
//! - `inner`: Content area inside the text box after applying inner padding.
//!   Text layout and scrollbar geometry are both computed in this area.
//! - `track`: Scrollbar rail drawn at the bottom of the `inner` area.
//!   This is the background strip that represents the full scroll range.
//! - `thumb`: Draggable indicator drawn on top of the `track`.
//!   Its width represents visible ratio, and its x-position represents
//!   current horizontal scroll offset.
//!
//! Flow:
//! 1. Compute `inner` bounds and scrollable range.
//! 2. Draw `track` if horizontal scrolling is applicable.
//! 3. Draw `thumb` based on content/view ratio and current scroll position.

use super::metrics::TextMetrics;
use crate::retained::node::TextBoxNode;
use crate::theme::color;
use crate::widget::text;
use vello::Scene;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill};

pub(super) struct ScrollBar<'a> {
    scene: &'a mut Scene,
    text_box: &'a TextBoxNode,
}

impl<'a> ScrollBar<'a> {
    pub(super) fn new(scene: &'a mut Scene, text_box: &'a TextBoxNode) -> Self {
        Self { scene, text_box }
    }

    /// Renders a bottom horizontal scrollbar only when needed.
    ///
    /// Computes scrollable range from content/view width, then draws
    /// track and thumb with derived geometry.
    pub(super) fn render_horizontal_scrollbar(&mut self) {
        if !self.text_box.overflow_x.allows_scroll() {
            return;
        }

        let inner_left: f64 = self.text_box.rect.x0 + TextMetrics::INNER_PADDING;
        let inner_right: f64 = self.text_box.rect.x1 - TextMetrics::INNER_PADDING;
        let inner_width: f64 = (inner_right - inner_left).max(TextMetrics::MIN_INNER_WIDTH);
        let content_width: f64 = self.text_box_content_width();
        let max_scroll: f64 = (content_width - inner_width).max(0.0);
        if max_scroll <= 0.0 {
            return;
        }

        let track_height: f64 = TextMetrics::SCROLLBAR_TRACK_HEIGHT;
        let track_y: f64 = self.text_box.rect.y1 - TextMetrics::SCROLLBAR_TRACK_BOTTOM_OFFSET;
        let track: Rect = Rect::new(inner_left, track_y - track_height, inner_right, track_y);
        let track_shape: RoundedRect =
            RoundedRect::from_rect(track, TextMetrics::SCROLLBAR_CORNER_RADIUS);
        let track_color: Color = if self.text_box.enabled {
            color::WhiteAlpha::tone_255_255_255_35()
        } else {
            color::WhiteAlpha::tone_255_255_255_20()
        };

        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            track_color,
            None,
            &track_shape,
        );

        let thumb_w: f64 = ((inner_width / content_width) * inner_width)
            .clamp(TextMetrics::SCROLLBAR_THUMB_MIN_WIDTH, inner_width)
            .min(inner_width);
        let ratio: f64 = (self.text_box.scroll_x / max_scroll).clamp(0.0, 1.0);
        let thumb_x0: f64 = inner_left + ratio * (inner_width - thumb_w);
        let thumb: Rect = Rect::new(
            thumb_x0,
            track_y - track_height,
            thumb_x0 + thumb_w,
            track_y,
        );
        let thumb_shape: RoundedRect =
            RoundedRect::from_rect(thumb, TextMetrics::SCROLLBAR_CORNER_RADIUS);
        let thumb_color: Color = if self.text_box.enabled {
            color::WhiteAlpha::tone_255_255_255_150()
        } else {
            color::WhiteAlpha::tone_255_255_255_90()
        };

        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            thumb_color,
            None,
            &thumb_shape,
        );
    }

    /// Returns content width for the text box.
    ///
    /// Uses cached `text_advance` first, then falls back to estimated width.
    fn text_box_content_width(&self) -> f64 {
        if self.text_box.text_advance > 0.0 {
            self.text_box.text_advance
        } else {
            f64::from(text::estimate_text_width(
                &self.text_box.text,
                self.text_box.font_size,
            ))
        }
    }
}
