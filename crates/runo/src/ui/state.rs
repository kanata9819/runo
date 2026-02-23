use super::Ui;
use crate::widget::checkbox::CheckboxResponse;
use crate::widget::radio_button::RadioButtonResponse;
use crate::widget::slider::SliderResponse;
use crate::widget::text_box::TextBoxResponse;
use crate::{ButtonResponse, ComboBoxResponse};

pub struct UiState<'ui, 'a> {
    pub(super) ui: &'ui mut Ui<'a>,
}

pub struct UiButtonState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiTextBoxState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiCheckboxState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiRadioButtonState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiSliderState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiComboBoxState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiLabelState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

pub struct UiDivState<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
}

impl<'ui, 'a> UiState<'ui, 'a> {
    pub fn button(&mut self) -> UiButtonState<'_, 'a> {
        UiButtonState { ui: &mut *self.ui }
    }

    pub fn text_box(&mut self) -> UiTextBoxState<'_, 'a> {
        UiTextBoxState { ui: &mut *self.ui }
    }

    pub fn checkbox(&mut self) -> UiCheckboxState<'_, 'a> {
        UiCheckboxState { ui: &mut *self.ui }
    }

    pub fn radio_button(&mut self) -> UiRadioButtonState<'_, 'a> {
        UiRadioButtonState { ui: &mut *self.ui }
    }

    pub fn slider(&mut self) -> UiSliderState<'_, 'a> {
        UiSliderState { ui: &mut *self.ui }
    }

    pub fn combo_box(&mut self) -> UiComboBoxState<'_, 'a> {
        UiComboBoxState { ui: &mut *self.ui }
    }

    pub fn label(&mut self) -> UiLabelState<'_, 'a> {
        UiLabelState { ui: &mut *self.ui }
    }

    pub fn div(&mut self) -> UiDivState<'_, 'a> {
        UiDivState { ui: &mut *self.ui }
    }
}

impl<'ui, 'a> UiButtonState<'ui, 'a> {
    pub fn response(&self, id: impl AsRef<str>) -> ButtonResponse {
        self.ui.retained.button_response(id)
    }

    pub fn clicked(&self, id: impl AsRef<str>) -> bool {
        self.response(id).clicked
    }

    pub fn set_text(&mut self, id: impl AsRef<str>, text: impl Into<String>) {
        self.ui.retained.set_button_text(id, Some(text.into()));
    }

    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_button_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiTextBoxState<'ui, 'a> {
    pub fn response(&self, id: impl AsRef<str>) -> TextBoxResponse {
        self.ui.retained.text_box_response(id)
    }

    pub fn text(&self, id: impl AsRef<str>) -> String {
        self.response(id).text
    }

    pub fn set_text(&mut self, id: impl AsRef<str>, text: impl Into<String>) {
        self.ui.retained.set_text_box_text(id, text);
    }

    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_text_box_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiCheckboxState<'ui, 'a> {
    pub fn response(&self, id: impl AsRef<str>) -> CheckboxResponse {
        self.ui.retained.checkbox_response(id)
    }

    pub fn checked(&self, id: impl AsRef<str>) -> bool {
        self.response(id).checked
    }

    pub fn set_checked(&mut self, id: impl AsRef<str>, checked: bool) {
        self.ui.retained.set_checkbox_checked(id, checked);
    }

    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_checkbox_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiRadioButtonState<'ui, 'a> {
    pub fn response(&self, id: impl AsRef<str>) -> RadioButtonResponse {
        self.ui.retained.radio_button_response(id)
    }

    pub fn selected(&self, id: impl AsRef<str>) -> bool {
        self.response(id).selected
    }

    pub fn set_selected(&mut self, id: impl AsRef<str>, selected: bool) {
        self.ui.retained.set_radio_button_selected(id, selected);
    }

    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_radio_button_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiSliderState<'ui, 'a> {
    pub fn response(&self, id: impl AsRef<str>) -> SliderResponse {
        self.ui.retained.slider_response(id)
    }

    pub fn value(&self, id: impl AsRef<str>) -> f64 {
        self.response(id).value
    }

    pub fn set_value(&mut self, id: impl AsRef<str>, value: f64) {
        self.ui.retained.set_slider_value(id, value);
    }

    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_slider_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiComboBoxState<'ui, 'a> {
    pub fn response(&self, id: impl AsRef<str>) -> ComboBoxResponse {
        self.ui.retained.combo_box_response(id)
    }

    pub fn selected_text(&self, id: impl AsRef<str>) -> String {
        self.response(id).selected_text
    }

    pub fn selected_index(&self, id: impl AsRef<str>) -> usize {
        self.response(id).selected_index
    }

    pub fn set_selected_index(&mut self, id: impl AsRef<str>, index: usize) {
        self.ui.retained.set_combo_box_selected_index(id, index);
    }

    pub fn set_items<I, T>(&mut self, id: impl AsRef<str>, items: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.ui.retained.set_combo_box_items(id, items);
    }

    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_combo_box_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiLabelState<'ui, 'a> {
    pub fn set_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.ui.retained.set_label_enabled(id, enabled);
    }
}

impl<'ui, 'a> UiDivState<'ui, 'a> {
    pub fn set_visible(&mut self, id: impl Into<String>, visible: bool) {
        self.ui.retained.set_div_visible(id, visible);
    }

    pub fn set_enabled(&mut self, id: impl Into<String>, enabled: bool) {
        self.ui.retained.set_div_enabled(id, enabled);
    }

    pub fn set_background(&mut self, id: impl Into<String>, color: crate::Color) {
        self.ui.retained.set_div_background(id, color);
    }

    pub fn clear_background(&mut self, id: impl AsRef<str>) {
        self.ui.retained.clear_div_background(id);
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::peniko::Color;

    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn ui_state_accessors_cover_all_widget_state_apis() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        ui.widgets().button().id("btn").text("b").show();
        ui.widgets().text_box().id("tb").text("x").show();
        ui.widgets().checkbox().id("cb").checked(false).show();
        ui.widgets()
            .radio_button()
            .id("rb")
            .group("g")
            .selected(false)
            .show();
        ui.widgets()
            .slider()
            .id("sl")
            .range(0.0, 1.0)
            .value(0.2)
            .show();
        ui.widgets()
            .combo_box()
            .id("co")
            .items(["a", "b"])
            .selected_index(0)
            .show();
        ui.widgets().label().id("lb").text("label").show();
        ui.widgets()
            .div()
            .id("dv")
            .background(Color::from_rgb8(20, 20, 20))
            .show(|_| ());

        let _ = ui.state().button().response("btn");
        assert!(!ui.state().button().clicked("btn"));
        ui.state().button().set_text("btn", "next");
        ui.state().button().set_enabled("btn", true);

        let _ = ui.state().text_box().response("tb");
        assert_eq!(ui.state().text_box().text("tb"), "x");
        ui.state().text_box().set_text("tb", "xx");
        ui.state().text_box().set_enabled("tb", true);

        let _ = ui.state().checkbox().response("cb");
        assert!(!ui.state().checkbox().checked("cb"));
        ui.state().checkbox().set_checked("cb", true);
        ui.state().checkbox().set_enabled("cb", true);

        let _ = ui.state().radio_button().response("rb");
        assert!(!ui.state().radio_button().selected("rb"));
        ui.state().radio_button().set_selected("rb", true);
        ui.state().radio_button().set_enabled("rb", true);

        let _ = ui.state().slider().response("sl");
        let _ = ui.state().slider().value("sl");
        ui.state().slider().set_value("sl", 0.7);
        ui.state().slider().set_enabled("sl", true);

        let _ = ui.state().combo_box().response("co");
        let _ = ui.state().combo_box().selected_text("co");
        let _ = ui.state().combo_box().selected_index("co");
        ui.state().combo_box().set_selected_index("co", 1);
        ui.state().combo_box().set_items("co", ["x", "y", "z"]);
        ui.state().combo_box().set_enabled("co", true);

        ui.state().label().set_enabled("lb", true);
        ui.state().div().set_visible("dv", true);
        ui.state().div().set_enabled("dv", true);
        ui.state()
            .div()
            .set_background("dv", Color::from_rgb8(30, 30, 30));
        ui.state().div().clear_background("dv");
    }
}
