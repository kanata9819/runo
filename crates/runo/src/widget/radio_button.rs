use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowRadioButtonArgs;
use crate::ui::UiEvents;
use crate::widget_model::radio_button::RadioButtonCommon;

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
    width: f64,
    height: f64,
    common: RadioButtonCommon,
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
            width: 260.0,
            height: 36.0,
            common: RadioButtonCommon::default(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.common.group = group.into();
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = f64::from(px);
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

    pub fn selected(mut self, selected: bool) -> Self {
        // Initial selected state at first creation.
        self.common.selected = Some(selected);
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

    pub fn enabled(mut self, value: bool) -> Self {
        self.common.enabled = value;
        self
    }

    pub fn show(self) -> RadioButtonHandle {
        let id: String = self.id;
        self.ui.show_radio_button(ShowRadioButtonArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            common: self.common,
        });

        RadioButtonHandle::new(id)
    }
}
