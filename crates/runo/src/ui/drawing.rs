use vello::kurbo::{Affine, Rect};
use vello::peniko::Fill;

use super::Ui;
use crate::Color;
use crate::widget::text;

impl Ui<'_> {
    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: Color) {
        let rect: Rect = Rect::new(x, y, x + w, y + h);
        self.scene
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rect);
    }

    pub fn draw_text(
        &mut self,
        x: f64,
        baseline_y: f64,
        text_value: &str,
        font_size: f32,
        color: Color,
    ) {
        let Some(font) = self.font.as_ref() else {
            return;
        };
        let Some((glyphs, _)) = text::layout_text(font, text_value, font_size) else {
            return;
        };

        text::draw_text_run(self.scene, font, glyphs, x, baseline_y, font_size, color);
    }

    pub fn estimate_text_width(&self, text_value: &str, font_size: f32) -> f64 {
        f64::from(text::estimate_text_width(text_value, font_size))
    }
}
