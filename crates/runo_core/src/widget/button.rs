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

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
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
