use vello::peniko::Color;

use crate::Ui;

#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonResponse {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

pub struct ButtonBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    text_color: Color,
}

impl<'ui, 'a> ButtonBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 180.0,
            height: 56.0,
            text: None,
            text_color: Color::from_rgb8(245, 248, 252),
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

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn show(self) -> ButtonResponse {
        self.ui
            .show_button(self.id, self.width, self.height, self.text, self.text_color)
    }
}
