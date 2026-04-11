use super::metrics::TextMetrics;
use crate::retained::node::TextBoxNode;
use crate::theme::color;
use crate::widget::text;
use vello::Scene;
use vello::kurbo::{Affine, Rect};
use vello::peniko::{Fill, FontData};

const CARET_X_OFFSET: f64 = 1.0;
const CARET_HEIGHT_RATIO: f64 = 1.1;
const CARET_TOP_OFFSET_RATIO: f64 = 0.9;
const CARET_WIDTH: f64 = 1.5;

pub struct Caret<'a> {
    scene: &'a mut Scene,
    font: &'a FontData,
    text_box: &'a TextBoxNode,
    metrics: TextMetrics,
}

impl<'a> Caret<'a> {
    pub(super) fn new(
        scene: &'a mut Scene,
        font: &'a FontData,
        text_box: &'a TextBoxNode,
        metrics: TextMetrics,
    ) -> Self {
        Self {
            scene,
            font,
            text_box,
            metrics,
        }
    }

    /// Draws caret when focused and enabled using the current text metrics.
    pub(super) fn draw_caret(&mut self) {
        if self.text_box.focused && self.text_box.enabled && !self.text_box.read_only {
            let (caret_line, caret_col) =
                Self::line_col_from_char_index(&self.text_box.text, self.text_box.caret_index);

            let caret_line_text: &str =
                self.text_box.text.split('\n').nth(caret_line).unwrap_or("");
            let prefix: String = caret_line_text.chars().take(caret_col).collect();
            let prefix_advance: f64 =
                text::layout_text(self.font, &prefix, self.text_box.font_size)
                    .map_or(0.0, |(_, advance)| f64::from(advance));

            let caret_x: f64 = self.metrics.text_x + prefix_advance + CARET_X_OFFSET;
            let caret_x: f64 = if self.text_box.overflow_x.clips() {
                caret_x.clamp(self.metrics.inner_left, self.metrics.inner_right)
            } else {
                caret_x
            };
            let baseline_y: f64 =
                self.metrics.first_line_baseline + caret_line as f64 * self.metrics.line_height;

            if self.text_box.overflow_y.clips()
                && !Self::line_intersects_vertical_clip(
                    baseline_y,
                    f64::from(self.text_box.font_size),
                    self.metrics.inner_top,
                    self.metrics.inner_bottom,
                )
            {
                return;
            }

            let caret_h: f64 = f64::from(self.text_box.font_size) * CARET_HEIGHT_RATIO;
            let caret_y0: f64 =
                baseline_y - f64::from(self.text_box.font_size) * CARET_TOP_OFFSET_RATIO;
            let caret: Rect =
                Rect::new(caret_x, caret_y0, caret_x + CARET_WIDTH, caret_y0 + caret_h);

            self.scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                color::SoftWhite::tone_220_228_240(),
                None,
                &caret,
            );
        }
    }
}

impl<'a> Caret<'a> {
    /// Converts a caret character index into `(line, column)`.
    ///
    /// Both returned coordinates are zero-based.
    pub(super) fn line_col_from_char_index(text: &str, caret_index: usize) -> (usize, usize) {
        let mut line: usize = 0;
        let mut col: usize = 0;
        for (i, ch) in text.chars().enumerate() {
            if i == caret_index {
                return (line, col);
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }

        (line, col)
    }

    /// Checks whether one line intersects the vertical clip region.
    ///
    /// Approximates line top/bottom from baseline and font size.
    pub(super) fn line_intersects_vertical_clip(
        baseline_y: f64,
        font_size: f64,
        clip_top: f64,
        clip_bottom: f64,
    ) -> bool {
        let line_top: f64 = baseline_y - font_size;
        let line_bottom: f64 = baseline_y + font_size * (TextMetrics::LINE_HEIGHT_RATIO - 1.0);
        line_bottom >= clip_top && line_top <= clip_bottom
    }
}
