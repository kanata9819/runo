use super::Ui;
use crate::event::UiEvent;
use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;
use std::collections::HashMap;

pub struct ActionBindings<A> {
    button: HashMap<ButtonHandle, A>,
    checkbox: HashMap<CheckboxHandle, A>,
    radio_button: HashMap<RadioButtonHandle, A>,
    slider: HashMap<SliderHandle, A>,
    text_box: HashMap<TextBoxHandle, A>,
    combo_box: HashMap<ComboBoxHandle, A>,
}

impl<A> ActionBindings<A> {
    pub fn new() -> Self {
        Self {
            button: HashMap::new(),
            checkbox: HashMap::new(),
            radio_button: HashMap::new(),
            slider: HashMap::new(),
            text_box: HashMap::new(),
            combo_box: HashMap::new(),
        }
    }

    pub fn bind_button(&mut self, handle: ButtonHandle, action: A) {
        self.button.insert(handle, action);
    }

    pub fn bind_checkbox(&mut self, handle: CheckboxHandle, action: A) {
        self.checkbox.insert(handle, action);
    }

    pub fn bind_radio_button(&mut self, handle: RadioButtonHandle, action: A) {
        self.radio_button.insert(handle, action);
    }

    pub fn bind_slider(&mut self, handle: SliderHandle, action: A) {
        self.slider.insert(handle, action);
    }

    pub fn bind_text_box(&mut self, handle: TextBoxHandle, action: A) {
        self.text_box.insert(handle, action);
    }

    pub fn bind_combo_box(&mut self, handle: ComboBoxHandle, action: A) {
        self.combo_box.insert(handle, action);
    }
}

impl<A> Default for ActionBindings<A> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UiEvents<'ui, 'a> {
    pub(super) ui: &'ui mut Ui<'a>,
}

impl<'ui, 'a> UiEvents<'ui, 'a> {
    pub fn next_event(&mut self) -> Option<UiEvent> {
        self.ui.retained.pop_event()
    }

    pub fn drain_events(&mut self) -> Vec<UiEvent> {
        self.ui.retained.drain_events()
    }

    pub fn drain_actions<A>(&mut self, bindings: &ActionBindings<A>) -> Vec<A>
    where
        A: Clone,
    {
        self.drain_events()
            .into_iter()
            .filter_map(|event| match event {
                UiEvent::ButtonClicked { button } => bindings.button.get(&button),
                UiEvent::CheckboxChanged { checkbox, .. } => bindings.checkbox.get(&checkbox),
                UiEvent::RadioButtonChanged { radio_button, .. } => {
                    bindings.radio_button.get(&radio_button)
                }
                UiEvent::SliderChanged { slider, .. } => bindings.slider.get(&slider),
                UiEvent::TextBoxChanged { text_box, .. } => bindings.text_box.get(&text_box),
                UiEvent::ComboBoxChanged { combo_box, .. } => bindings.combo_box.get(&combo_box),
            })
            .cloned()
            .collect()
    }

    pub fn on_button_clicked(&mut self, handle: &ButtonHandle, f: impl FnOnce()) {
        if self.ui.retained.take_button_clicked(handle) {
            f();
        }
    }

    pub fn on_button_clicked_with_ui(
        &mut self,
        handle: &ButtonHandle,
        f: impl FnOnce(&mut Ui<'a>),
    ) {
        if self.ui.retained.take_button_clicked(handle) {
            f(self.ui);
        }
    }

    pub fn on_text_box_changed(&mut self, handle: &TextBoxHandle, f: impl FnOnce(String)) {
        if let Some(text) = self.ui.retained.take_text_box_changed(handle) {
            f(text);
        }
    }

    pub fn on_text_box_changed_with_ui(
        &mut self,
        handle: &TextBoxHandle,
        f: impl FnOnce(&mut Ui<'a>, String),
    ) {
        if let Some(text) = self.ui.retained.take_text_box_changed(handle) {
            f(self.ui, text);
        }
    }

    pub fn on_checkbox_changed(&mut self, handle: &CheckboxHandle, f: impl FnOnce(bool)) {
        if let Some(checked) = self.ui.retained.take_checkbox_changed(handle) {
            f(checked);
        }
    }

    pub fn on_checkbox_changed_with_ui(
        &mut self,
        handle: &CheckboxHandle,
        f: impl FnOnce(&mut Ui<'a>, bool),
    ) {
        if let Some(checked) = self.ui.retained.take_checkbox_changed(handle) {
            f(self.ui, checked);
        }
    }

    pub fn on_slider_changed(&mut self, handle: &SliderHandle, f: impl FnOnce(f64)) {
        if let Some(value) = self.ui.retained.take_slider_changed(handle) {
            f(value);
        }
    }

    pub fn on_slider_changed_with_ui(
        &mut self,
        handle: &SliderHandle,
        f: impl FnOnce(&mut Ui<'a>, f64),
    ) {
        if let Some(value) = self.ui.retained.take_slider_changed(handle) {
            f(self.ui, value);
        }
    }

    pub fn on_radio_button_changed(&mut self, handle: &RadioButtonHandle, f: impl FnOnce(bool)) {
        if let Some(selected) = self.ui.retained.take_radio_button_changed(handle) {
            f(selected);
        }
    }

    pub fn on_radio_button_changed_with_ui(
        &mut self,
        handle: &RadioButtonHandle,
        f: impl FnOnce(&mut Ui<'a>, bool),
    ) {
        if let Some(selected) = self.ui.retained.take_radio_button_changed(handle) {
            f(self.ui, selected);
        }
    }

    pub fn on_combo_box_changed(&mut self, handle: &ComboBoxHandle, f: impl FnOnce(usize, String)) {
        if let Some((selected_index, selected_text)) =
            self.ui.retained.take_combo_box_changed(handle)
        {
            f(selected_index, selected_text);
        }
    }

    pub fn on_combo_box_changed_with_ui(
        &mut self,
        handle: &ComboBoxHandle,
        f: impl FnOnce(&mut Ui<'a>, usize, String),
    ) {
        if let Some((selected_index, selected_text)) =
            self.ui.retained.take_combo_box_changed(handle)
        {
            f(self.ui, selected_index, selected_text);
        }
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;

    use super::ActionBindings;
    use crate::event::UiEvent;
    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;
    use crate::widget::button::ButtonHandle;
    use crate::widget::checkbox::CheckboxHandle;
    use crate::widget::combo_box::ComboBoxHandle;
    use crate::widget::radio_button::RadioButtonHandle;
    use crate::widget::slider::SliderHandle;
    use crate::widget::text_box::TextBoxHandle;

    #[test]
    fn ui_events_empty_queue_returns_none_and_empty_vec() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        assert!(ui.events().next_event().is_none());
        assert!(ui.events().drain_events().is_empty());
    }

    #[test]
    fn ui_events_can_drain_mapped_actions() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let button = ButtonHandle::new("btn".to_string());
        ui.retained.push_event(UiEvent::ButtonClicked {
            button: button.clone(),
        });

        let mut bindings = ActionBindings::new();
        bindings.bind_button(button, "do".to_string());

        let actions = ui.events().drain_actions(&bindings);
        assert_eq!(actions, vec!["do".to_string()]);
    }

    #[test]
    fn ui_events_callback_helpers_consume_matching_events() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let button = ButtonHandle::new("btn".to_string());
        let text_box = TextBoxHandle::new("tb".to_string());
        let checkbox = CheckboxHandle::new("cb".to_string());
        ui.retained.push_event(UiEvent::ButtonClicked {
            button: button.clone(),
        });
        ui.retained.push_event(UiEvent::TextBoxChanged {
            text_box: text_box.clone(),
            text: "hello".to_string(),
        });
        ui.retained.push_event(UiEvent::CheckboxChanged {
            checkbox: checkbox.clone(),
            checked: true,
        });

        let mut clicked = false;
        let mut text = String::new();
        let mut checked = false;
        {
            let mut events = ui.events();
            events.on_button_clicked(&button, || clicked = true);
            events.on_text_box_changed(&text_box, |value| text = value);
            events.on_checkbox_changed(&checkbox, |value| checked = value);
        }

        assert!(clicked);
        assert_eq!(text, "hello");
        assert!(checked);
        assert!(ui.events().drain_events().is_empty());
    }

    #[test]
    fn ui_events_callback_helpers_consume_slider_radio_and_combo_events() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let slider = SliderHandle::new("sl".to_string());
        let radio_button = RadioButtonHandle::new("rb".to_string());
        let combo_box = ComboBoxHandle::new("cb".to_string());
        ui.retained.push_event(UiEvent::SliderChanged {
            slider: slider.clone(),
            value: 0.75,
        });
        ui.retained.push_event(UiEvent::RadioButtonChanged {
            radio_button: radio_button.clone(),
            group: "g".to_string(),
            selected: true,
        });
        ui.retained.push_event(UiEvent::ComboBoxChanged {
            combo_box: combo_box.clone(),
            selected_index: 1,
            selected_text: "b".to_string(),
        });

        let mut slider_value = 0.0;
        let mut selected = false;
        let mut combo = (0usize, String::new());
        {
            let mut events = ui.events();
            events.on_slider_changed(&slider, |value| slider_value = value);
            events.on_radio_button_changed(&radio_button, |value| selected = value);
            events.on_combo_box_changed(&combo_box, |index, text| combo = (index, text));
        }

        assert!((slider_value - 0.75).abs() < f64::EPSILON);
        assert!(selected);
        assert_eq!(combo, (1, "b".to_string()));
        assert!(ui.events().drain_events().is_empty());
    }
}
