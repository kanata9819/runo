use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowSliderArgs;

#[derive(Clone, Debug, Default)]
pub struct SliderResponse {
    pub value: f64,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
}

pub struct SliderBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    min: f64,
    max: f64,
    value: Option<f64>,
    step: Option<f64>,
    text: Option<String>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

impl<'ui, 'a> SliderBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 320.0,
            height: 40.0,
            min: 0.0,
            max: 100.0,
            value: None,
            step: None,
            text: None,
            font_size: 16.0,
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

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        // Initial value at first creation.
        self.value = Some(value);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
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

    pub fn show(self) -> SliderResponse {
        self.ui.show_slider(ShowSliderArgs {
            id: self.id,
            width: self.width,
            height: self.height,
            min: self.min,
            max: self.max,
            value: self.value,
            step: self.step,
            text: self.text,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SliderResponse;

    #[test]
    fn slider_response_default_is_zero_and_idle() {
        let response = SliderResponse::default();
        assert_eq!(response.value, 0.0);
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
    }
}
