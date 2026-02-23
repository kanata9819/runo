use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowComboBoxArgs;

#[derive(Clone, Debug, Default)]
pub struct ComboBoxResponse {
    pub selected_index: usize,
    pub selected_text: String,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
    pub is_open: bool,
}

pub struct ComboBoxBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    items: Vec<String>,
    selected_index: Option<usize>,
    font_size: f32,
    text_color: Color,
    bg_color: Color,
    border_color: Color,
    enabled: bool,
}

impl<'ui, 'a> ComboBoxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 280.0,
            height: 44.0,
            items: Vec::new(),
            selected_index: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            bg_color: Color::from_rgb8(33, 38, 46),
            border_color: Color::from_rgb8(78, 89, 104),
            enabled: true,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = px as f64;
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = px as f64;
        self
    }

    pub fn items<I, T>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
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

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    pub fn show(self) -> ComboBoxResponse {
        self.ui.show_combo_box(ShowComboBoxArgs {
            id: self.id,
            width: self.width,
            height: self.height,
            items: self.items,
            selected_index: self.selected_index,
            font_size: self.font_size,
            text_color: self.text_color,
            bg_color: self.bg_color,
            border_color: self.border_color,
            enabled: self.enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ComboBoxResponse;

    #[test]
    fn combo_box_response_default_is_empty_and_closed() {
        let response = ComboBoxResponse::default();
        assert_eq!(response.selected_index, 0);
        assert_eq!(response.selected_text, "");
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
        assert!(!response.is_open);
    }
}
