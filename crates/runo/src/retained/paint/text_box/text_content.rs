//! Text and placeholder painter for `TextBox`.
//!
//! This module handles line layout, clipping, and draw calls for text content.

use super::caret::Caret;
use super::metrics::TextMetrics;
use crate::retained::node::TextBoxNode;
use crate::widget::text;
use vello::Glyph;
use vello::Scene;
use vello::peniko::{Color, FontData};

pub(super) struct TextContentPainter;

struct TextLinePainter<'a> {
    scene: &'a mut Scene,
    font: &'a FontData,
    text_box: &'a TextBoxNode,
    text_color: Color,
    metrics: TextMetrics,
}

impl TextContentPainter {
    /// Draws text content or placeholder and updates cached text advance.
    ///
    /// Lays out multi-line text per line and applies vertical/horizontal clipping
    /// when required.
    pub(super) fn draw_text_content(
        scene: &mut Scene,
        font: &FontData,
        text_box: &mut TextBoxNode,
        text_color: Color,
        metrics: TextMetrics,
    ) {
        if text_box.text.is_empty() {
            Self::draw_placeholder_text(scene, font, text_box, text_color, metrics);
            text_box.text_advance = 0.0;
            return;
        }

        text_box.text_advance =
            Self::draw_multiline_text(scene, font, text_box, text_color, metrics);
    }

    /// Draws placeholder text for an empty text box.
    fn draw_placeholder_text(
        scene: &mut Scene,
        font: &FontData,
        text_box: &TextBoxNode,
        text_color: Color,
        metrics: TextMetrics,
    ) {
        let placeholder: &str = text_box.placeholder.as_deref().unwrap_or("");
        let mut line_painter: TextLinePainter =
            TextLinePainter::new(scene, font, text_box, text_color, metrics);
        line_painter.draw_visible_text_line(placeholder, metrics.first_line_baseline);
    }

    /// Draws multi-line text content and returns the maximum line advance.
    fn draw_multiline_text(
        scene: &mut Scene,
        font: &FontData,
        text_box: &TextBoxNode,
        text_color: Color,
        metrics: TextMetrics,
    ) -> f64 {
        let mut line_painter: TextLinePainter =
            TextLinePainter::new(scene, font, text_box, text_color, metrics);
        let mut max_advance: f64 = 0.0;
        for (line_index, line_text) in text_box.text.split('\n').enumerate() {
            let baseline_y: f64 =
                metrics.first_line_baseline + line_index as f64 * metrics.line_height;
            let Some(line_advance) = line_painter.draw_visible_text_line(line_text, baseline_y)
            else {
                continue;
            };

            max_advance = max_advance.max(line_advance);
        }

        max_advance
    }

    /// Returns only glyphs that intersect the horizontal clip region.
    ///
    /// Visibility is computed from each glyph start and the next glyph position
    /// (or `total_advance` for the last glyph).
    pub(super) fn clip_glyphs_horizontally(
        glyphs: Vec<Glyph>,
        total_advance: f64,
        draw_origin_x: f64,
        clip_left: f64,
        clip_right: f64,
    ) -> Vec<Glyph> {
        if glyphs.is_empty() || clip_right <= clip_left {
            return Vec::new();
        }

        let mut out: Vec<Glyph> = Vec::new();
        for (index, glyph) in glyphs.iter().enumerate() {
            let x0: f64 = draw_origin_x + f64::from(glyph.x);
            let next_x: f64 = if let Some(next) = glyphs.get(index + 1) {
                draw_origin_x + f64::from(next.x)
            } else {
                draw_origin_x + total_advance
            };
            if next_x >= clip_left && x0 <= clip_right {
                out.push(*glyph);
            }
        }

        out
    }
}

impl<'a> TextLinePainter<'a> {
    fn new(
        scene: &'a mut Scene,
        font: &'a FontData,
        text_box: &'a TextBoxNode,
        text_color: Color,
        metrics: TextMetrics,
    ) -> Self {
        Self {
            scene,
            font,
            text_box,
            text_color,
            metrics,
        }
    }

    /// Draws one logical line if visible and returns its layout advance.
    fn draw_visible_text_line(&mut self, line_text: &str, baseline_y: f64) -> Option<f64> {
        if !self.line_is_visible(baseline_y) {
            return None;
        }

        let (glyphs, advance) = text::layout_text(self.font, line_text, self.text_box.font_size)?;
        let advance_f64: f64 = f64::from(advance);
        let visible_glyphs: Vec<Glyph> = self.maybe_clip_glyphs_horizontally(glyphs, advance_f64);

        if !visible_glyphs.is_empty() {
            text::draw_text_run(
                self.scene,
                self.font,
                visible_glyphs,
                self.metrics.text_x,
                baseline_y,
                self.text_box.font_size,
                self.text_color,
            );
        }

        Some(advance_f64)
    }

    /// Returns whether a baseline line intersects the current vertical clip area.
    fn line_is_visible(&self, baseline_y: f64) -> bool {
        !self.text_box.overflow_y.clips()
            || Caret::line_intersects_vertical_clip(
                baseline_y,
                f64::from(self.text_box.font_size),
                self.metrics.inner_top,
                self.metrics.inner_bottom,
            )
    }

    /// Applies horizontal clipping only when the text box overflows on X.
    fn maybe_clip_glyphs_horizontally(&self, glyphs: Vec<Glyph>, total_advance: f64) -> Vec<Glyph> {
        if self.text_box.overflow_x.clips() {
            TextContentPainter::clip_glyphs_horizontally(
                glyphs,
                total_advance,
                self.metrics.text_x,
                self.metrics.inner_left,
                self.metrics.inner_right,
            )
        } else {
            glyphs
        }
    }
}
