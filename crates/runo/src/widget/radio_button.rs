use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowRadioButtonArgs;
use crate::ui::UiEvents;

#[cfg(test)]
#[path = "../../tests/unit/widget/radio_button.rs"]
mod tests;

#[derive(Clone, Debug, Default)]
pub struct RadioButtonResponse {
    pub selected: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
}

pub struct RadioButtonBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    group: String,
    width: f64,
    height: f64,
    text: Option<String>,
    selected: Option<bool>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RadioButtonHandle {
    id: String,
}

impl RadioButtonHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> RadioButtonResponse {
        ui.state().radio_button().response(self.id())
    }

    pub fn selected(&self, ui: &mut Ui<'_>) -> bool {
        self.response(ui).selected
    }

    pub fn set_selected(&self, ui: &mut Ui<'_>, selected: bool) {
        ui.state().radio_button().set_selected(self.id(), selected);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().radio_button().set_enabled(self.id(), enabled);
    }

    pub fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(bool)) {
        events.on_radio_button_changed(self, f);
    }

    pub fn take_change(&self, events: &mut UiEvents<'_, '_>) -> Option<bool> {
        events.radio_button_changed(self)
    }

    pub fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut Ui<'_>, bool),
    ) {
        events.on_radio_button_changed_with_ui(self, f);
    }
}

impl<'ui, 'a> RadioButtonBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            group: "default".to_string(),
            width: 260.0,
            height: 36.0,
            text: None,
            selected: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            enabled: true,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
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

    pub fn selected(mut self, selected: bool) -> Self {
        // Initial selected state at first creation.
        self.selected = Some(selected);
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

    pub fn show(self) -> RadioButtonHandle {
        let id = self.id;
        self.ui.show_radio_button(ShowRadioButtonArgs {
            id: id.clone(),
            group: self.group,
            width: self.width,
            height: self.height,
            text: self.text,
            selected: self.selected,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        });

        RadioButtonHandle::new(id)
    }
}
