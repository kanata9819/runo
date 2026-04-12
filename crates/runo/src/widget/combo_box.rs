use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowComboBoxArgs;
use crate::ui::UiEvents;
use crate::widget_model::combo_box::ComboBoxCommon;

#[cfg(test)]
#[path = "../../tests/unit/widget/combo_box.rs"]
mod tests;

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
    common: ComboBoxCommon,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComboBoxHandle {
    id: String,
}

impl ComboBoxHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> ComboBoxResponse {
        ui.state().combo_box().response(self.id())
    }

    pub fn selected_text(&self, ui: &mut Ui<'_>) -> String {
        self.response(ui).selected_text
    }

    pub fn selected_index(&self, ui: &mut Ui<'_>) -> usize {
        self.response(ui).selected_index
    }

    pub fn set_selected_index(&self, ui: &mut Ui<'_>, index: usize) {
        ui.state().combo_box().set_selected_index(self.id(), index);
    }

    pub fn set_items<I, T>(&self, ui: &mut Ui<'_>, items: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        ui.state().combo_box().set_items(self.id(), items);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().combo_box().set_enabled(self.id(), enabled);
    }

    pub fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(usize, String)) {
        events.on_combo_box_changed(self, f);
    }

    pub fn take_change(&self, events: &mut UiEvents<'_, '_>) -> Option<(usize, String)> {
        events.combo_box_changed(self)
    }

    pub fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut Ui<'_>, usize, String),
    ) {
        events.on_combo_box_changed_with_ui(self, f);
    }
}

impl<'ui, 'a> ComboBoxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 280.0,
            height: 44.0,
            common: ComboBoxCommon::default(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
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

    pub fn items<I, T>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.common.items = items.into_iter().map(Into::into).collect();
        self
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.common.selected_index = Some(index);
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

    pub fn enabled(mut self, value: bool) -> Self {
        self.common.enabled = value;
        self
    }

    pub fn show(self) -> ComboBoxHandle {
        let id: String = self.id;
        self.ui.show_combo_box(ShowComboBoxArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            common: self.common,
        });

        ComboBoxHandle::new(id)
    }
}
