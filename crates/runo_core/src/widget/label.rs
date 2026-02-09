use vello::peniko::Color;

use crate::Ui;
use crate::widget::text::{draw_text_run, estimate_text_width, layout_text};

pub struct LabelBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    text: String,
    font_size: f32,
    text_color: Color,
}

impl<'ui, 'a> LabelBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, text: String) -> Self {
        Self {
            ui,
            text,
            font_size: 18.0,
            text_color: Color::from_rgb8(245, 248, 252),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn show(self) {
        let height = self.font_size as f64 * 1.35;

        let (glyphs, width) = if let Some(font) = self.ui.font.as_ref() {
            layout_text(font, &self.text, self.font_size)
                .unwrap_or_else(|| (Vec::new(), estimate_text_width(&self.text, self.font_size)))
        } else {
            (Vec::new(), estimate_text_width(&self.text, self.font_size))
        };

        let (x, y) = self.ui.allocate_rect(width as f64, height);

        if let Some(font) = self.ui.font.as_ref() {
            let baseline_y = y + self.font_size as f64;
            draw_text_run(
                self.ui.scene,
                font,
                glyphs,
                x,
                baseline_y,
                self.font_size,
                self.text_color,
            );
        }
    }
}
