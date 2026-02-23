use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowCheckboxArgs;

#[derive(Clone, Debug, Default)]
pub struct CheckboxResponse {
    pub checked: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
}

pub struct CheckboxBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    checked: Option<bool>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

impl<'ui, 'a> CheckboxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 260.0,
            height: 36.0,
            text: None,
            checked: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
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

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        // Initial checked state at first creation.
        self.checked = Some(checked);
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

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    pub fn show(self) -> CheckboxResponse {
        self.ui.show_checkbox(ShowCheckboxArgs {
            id: self.id,
            width: self.width,
            height: self.height,
            text: self.text,
            checked: self.checked,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CheckboxResponse;

    #[test]
    fn checkbox_response_default_is_unchecked_and_idle() {
        let response = CheckboxResponse::default();
        assert!(!response.checked);
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
    }
}
