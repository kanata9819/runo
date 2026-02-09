use vello::peniko::Color;

use crate::Ui;
use crate::widget::text::{estimate_text_width, layout_text};

pub struct LabelBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    text: String,
    font_size: f32,
    text_color: Color,
}

impl<'ui, 'a> LabelBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String, text: String) -> Self {
        Self {
            ui,
            id,
            text,
            font_size: 18.0,
            text_color: Color::from_rgb8(245, 248, 252),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn size(mut self, px: u32) -> Self {
        self.font_size = px as f32;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn show(self) {
        let height = self.font_size as f64 * 1.35;
        let width = if let Some(font) = self.ui.font.as_ref() {
            layout_text(font, &self.text, self.font_size)
                .map(|(_, width)| width)
                .unwrap_or_else(|| estimate_text_width(&self.text, self.font_size))
        } else {
            estimate_text_width(&self.text, self.font_size)
        };

        self.ui.show_label(
            self.id,
            width as f64,
            height,
            self.text,
            self.font_size,
            self.text_color,
        );
    }
}
