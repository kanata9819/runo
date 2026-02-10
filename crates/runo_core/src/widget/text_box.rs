use vello::peniko::Color;

use crate::Ui;

#[derive(Clone, Debug, Default)]
pub struct TextBoxResponse {
    pub text: String,
    pub hovered: bool,
    pub focused: bool,
    pub changed: bool,
}

pub struct TextBoxBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    placeholder: Option<String>,
    font_size: f32,
    text_color: Color,
    bg_color: Color,
    border_color: Color,
}

impl<'ui, 'a> TextBoxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 280.0,
            height: 44.0,
            text: None,
            placeholder: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            bg_color: Color::from_rgb8(33, 38, 46),
            border_color: Color::from_rgb8(78, 89, 104),
        }
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = px as f64;
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = px as f64;
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width as f64;
        self.height = height as f64;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn font_size(mut self, px: u32) -> Self {
        self.font_size = px as f32;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn show(self) -> TextBoxResponse {
        self.ui.show_text_box(
            self.id,
            self.width,
            self.height,
            self.text,
            self.placeholder,
            self.font_size,
            self.text_color,
            self.bg_color,
            self.border_color,
        )
    }
}
