use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowComboBoxArgs;
use crate::ui::UiEvents;

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

    pub fn show(self) -> ComboBoxHandle {
        let id = self.id;
        self.ui.show_combo_box(ShowComboBoxArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            items: self.items,
            selected_index: self.selected_index,
            font_size: self.font_size,
            text_color: self.text_color,
            bg_color: self.bg_color,
            border_color: self.border_color,
            enabled: self.enabled,
        });
        ComboBoxHandle::new(id)
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::peniko::Color;

    use super::ComboBoxResponse;
    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

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

    #[test]
    fn combo_box_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let combo = ui
            .widgets()
            .combo_box()
            .id("combo")
            .width(280)
            .height(44)
            .items(["a", "b", "c"])
            .selected_index(1)
            .font_size(19)
            .text_color(Color::from_rgb8(240, 240, 240))
            .bg_color(Color::from_rgb8(30, 30, 30))
            .border_color(Color::from_rgb8(90, 90, 90))
            .enabled(false)
            .show();
        assert_eq!(combo.selected_text(&mut ui), "b");

        combo.set_enabled(&mut ui, true);
        combo.set_selected_index(&mut ui, 2);
        assert_eq!(combo.selected_text(&mut ui), "c");
        combo.set_items(&mut ui, ["x", "y"]);
        assert_eq!(combo.selected_index(&mut ui), 1);
    }
}
