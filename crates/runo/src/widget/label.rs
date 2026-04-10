use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowLabelArgs;
use crate::widget::text::{estimate_text_width, layout_text};

#[cfg(test)]
#[path = "../../tests/unit/widget/label.rs"]
mod tests;

pub struct LabelBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    text: String,
    width: Option<f64>,
    height: Option<f64>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabelHandle {
    id: String,
}

impl LabelHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().label().set_enabled(self.id(), enabled);
    }
}

impl<'ui, 'a> LabelBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            text: String::new(),
            width: None,
            height: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(245, 248, 252),
            enabled: true,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = Some(f64::from(px));
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = Some(f64::from(px));
        self
    }

    pub fn font_size(mut self, px: u32) -> Self {
        self.font_size = px as f32;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
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

    pub fn show(self) -> LabelHandle {
        let intrinsic_height = f64::from(self.font_size) * 1.35;
        let intrinsic_width = if let Some(font) = self.ui.font.as_ref() {
            layout_text(font, &self.text, self.font_size).map_or_else(|| estimate_text_width(&self.text, self.font_size), |(_, width)| width)
        } else {
            estimate_text_width(&self.text, self.font_size)
        };

        let width = self.width.unwrap_or(f64::from(intrinsic_width));
        let height = self.height.unwrap_or(intrinsic_height);
        let id = self.id;

        self.ui.show_label(ShowLabelArgs {
            id: id.clone(),
            width,
            height,
            text: self.text,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        });

        LabelHandle::new(id)
    }
}
