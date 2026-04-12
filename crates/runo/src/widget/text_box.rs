use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowTextBoxArgs;
use crate::ui::UiEvents;
use crate::widget_model::text_box::TextBoxCommon;

#[cfg(test)]
#[path = "../../tests/unit/widget/text_box.rs"]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl Overflow {
    pub(crate) fn allows_scroll(self) -> bool {
        matches!(self, Self::Scroll | Self::Auto)
    }

    pub(crate) fn clips(self) -> bool {
        !matches!(self, Self::Visible)
    }
}

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
    common: TextBoxCommon,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextBoxHandle {
    id: String,
}

impl TextBoxHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> TextBoxResponse {
        ui.state().text_box().response(self.id())
    }

    pub fn text(&self, ui: &mut Ui<'_>) -> String {
        self.response(ui).text
    }

    pub fn set_text(&self, ui: &mut Ui<'_>, text: impl Into<String>) {
        ui.state().text_box().set_text(self.id(), text);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().text_box().set_enabled(self.id(), enabled);
    }

    pub fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(String)) {
        events.on_text_box_changed(self, f);
    }

    pub fn take_change(&self, events: &mut UiEvents<'_, '_>) -> Option<String> {
        events.text_box_changed(self)
    }

    pub fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut Ui<'_>, String),
    ) {
        events.on_text_box_changed_with_ui(self, f);
    }
}

impl<'ui, 'a> TextBoxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 280.0,
            height: 44.0,
            common: TextBoxCommon::default(),
        }
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = f64::from(px);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = f64::from(px);
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.common.text = Some(text.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.common.placeholder = Some(text.into());
        self
    }

    pub fn font_size(mut self, px: u32) -> Self {
        self.common.font_size = px as f32;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.common.text_color = color;
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.common.bg_color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.common.border_color = color;
        self
    }

    pub fn disable_border(mut self, value: bool) -> Self {
        self.common.disable_border = value;
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.common.enabled = value;
        self
    }

    pub fn read_only(mut self, value: bool) -> Self {
        self.common.read_only = value;
        self
    }

    pub fn overflow_x(mut self, overflow: Overflow) -> Self {
        self.common.overflow_x = overflow;
        self
    }

    pub fn overflow_y(mut self, overflow: Overflow) -> Self {
        self.common.overflow_y = overflow;
        self
    }

    pub fn show(self) -> TextBoxHandle {
        let id: String = self.id;
        self.ui.show_text_box(ShowTextBoxArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            common: self.common,
        });

        TextBoxHandle::new(id)
    }
}
