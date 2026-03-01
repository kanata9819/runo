use super::Ui;
use crate::event::UiEvent;
use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;
use std::collections::HashMap;

#[cfg(test)]
#[path = "../../tests/unit/ui/events.rs"]
mod tests;

pub struct ActionBindings<A> {
    button: HashMap<ButtonHandle, A>,
    checkbox: HashMap<CheckboxHandle, A>,
    radio_button: HashMap<RadioButtonHandle, A>,
    slider: HashMap<SliderHandle, A>,
    text_box: HashMap<TextBoxHandle, A>,
    combo_box: HashMap<ComboBoxHandle, A>,
}

pub struct EventBindings<E> {
    button: HashMap<ButtonHandle, Box<dyn Fn() -> E>>,
    checkbox: HashMap<CheckboxHandle, Box<dyn Fn(bool) -> E>>,
    radio_button: HashMap<RadioButtonHandle, Box<dyn Fn(bool) -> E>>,
    slider: HashMap<SliderHandle, Box<dyn Fn(f64) -> E>>,
    text_box: HashMap<TextBoxHandle, Box<dyn Fn(String) -> E>>,
    combo_box: HashMap<ComboBoxHandle, Box<dyn Fn(usize, String) -> E>>,
}

pub struct EventBindingsBuilder<E> {
    bindings: EventBindings<E>,
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

impl<E> EventBindings<E> {
    pub fn builder() -> EventBindingsBuilder<E> {
        EventBindingsBuilder::new()
    }

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

    pub fn bind_button(&mut self, handle: ButtonHandle, event: E)
    where
        E: Clone + 'static,
    {
        self.bind_button_with(handle, move || event.clone());
    }

    pub fn bind_button_with(&mut self, handle: ButtonHandle, f: impl Fn() -> E + 'static) {
        self.button.insert(handle, Box::new(f));
    }

    pub fn bind_checkbox(&mut self, handle: CheckboxHandle, f: impl Fn(bool) -> E + 'static) {
        self.checkbox.insert(handle, Box::new(f));
    }

    pub fn bind_radio_button(
        &mut self,
        handle: RadioButtonHandle,
        f: impl Fn(bool) -> E + 'static,
    ) {
        self.radio_button.insert(handle, Box::new(f));
    }

    pub fn bind_slider(&mut self, handle: SliderHandle, f: impl Fn(f64) -> E + 'static) {
        self.slider.insert(handle, Box::new(f));
    }

    pub fn bind_text_box(&mut self, handle: TextBoxHandle, f: impl Fn(String) -> E + 'static) {
        self.text_box.insert(handle, Box::new(f));
    }

    pub fn bind_combo_box(
        &mut self,
        handle: ComboBoxHandle,
        f: impl Fn(usize, String) -> E + 'static,
    ) {
        self.combo_box.insert(handle, Box::new(f));
    }
}

impl<E> EventBindingsBuilder<E> {
    pub fn new() -> Self {
        Self {
            bindings: EventBindings::new(),
        }
    }

    pub fn button(mut self, handle: ButtonHandle, event: E) -> Self
    where
        E: Clone + 'static,
    {
        self.bindings.bind_button(handle, event);
        self
    }

    pub fn button_with(mut self, handle: ButtonHandle, f: impl Fn() -> E + 'static) -> Self {
        self.bindings.bind_button_with(handle, f);
        self
    }

    pub fn checkbox(mut self, handle: CheckboxHandle, f: impl Fn(bool) -> E + 'static) -> Self {
        self.bindings.bind_checkbox(handle, f);
        self
    }

    pub fn radio_button(
        mut self,
        handle: RadioButtonHandle,
        f: impl Fn(bool) -> E + 'static,
    ) -> Self {
        self.bindings.bind_radio_button(handle, f);
        self
    }

    pub fn slider(mut self, handle: SliderHandle, f: impl Fn(f64) -> E + 'static) -> Self {
        self.bindings.bind_slider(handle, f);
        self
    }

    pub fn text_box(mut self, handle: TextBoxHandle, f: impl Fn(String) -> E + 'static) -> Self {
        self.bindings.bind_text_box(handle, f);
        self
    }

    pub fn combo_box(
        mut self,
        handle: ComboBoxHandle,
        f: impl Fn(usize, String) -> E + 'static,
    ) -> Self {
        self.bindings.bind_combo_box(handle, f);
        self
    }

    pub fn extend(mut self, other: EventBindings<E>) -> Self {
        self.bindings.button.extend(other.button);
        self.bindings.checkbox.extend(other.checkbox);
        self.bindings.radio_button.extend(other.radio_button);
        self.bindings.slider.extend(other.slider);
        self.bindings.text_box.extend(other.text_box);
        self.bindings.combo_box.extend(other.combo_box);
        self
    }

    pub fn build(self) -> EventBindings<E> {
        self.bindings
    }
}

impl<A> Default for ActionBindings<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Default for EventBindings<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Default for EventBindingsBuilder<E> {
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

    pub fn drain_bound_events<E>(&mut self, bindings: &EventBindings<E>) -> Vec<E> {
        self.drain_events()
            .into_iter()
            .filter_map(|event| match event {
                UiEvent::ButtonClicked { button } => bindings.button.get(&button).map(|f| f()),
                UiEvent::CheckboxChanged { checkbox, checked } => {
                    bindings.checkbox.get(&checkbox).map(|f| f(checked))
                }
                UiEvent::RadioButtonChanged {
                    radio_button,
                    selected,
                    ..
                } => bindings
                    .radio_button
                    .get(&radio_button)
                    .map(|f| f(selected)),
                UiEvent::SliderChanged { slider, value } => {
                    bindings.slider.get(&slider).map(|f| f(value))
                }
                UiEvent::TextBoxChanged { text_box, text } => {
                    bindings.text_box.get(&text_box).map(|f| f(text))
                }
                UiEvent::ComboBoxChanged {
                    combo_box,
                    selected_index,
                    selected_text,
                } => bindings
                    .combo_box
                    .get(&combo_box)
                    .map(|f| f(selected_index, selected_text)),
            })
            .collect()
    }

    pub fn on_button_clicked(&mut self, handle: &ButtonHandle, f: impl FnOnce()) {
        if self.take_button_clicked(handle) {
            f();
        }
    }

    pub fn button_clicked(&mut self, handle: &ButtonHandle) -> bool {
        self.take_button_clicked(handle)
    }

    pub fn on_button_clicked_with_ui(
        &mut self,
        handle: &ButtonHandle,
        f: impl FnOnce(&mut Ui<'a>),
    ) {
        if self.take_button_clicked(handle) {
            f(self.ui);
        }
    }

    pub fn on_text_box_changed(&mut self, handle: &TextBoxHandle, f: impl FnOnce(String)) {
        let changed = self.take_text_box_changed(handle);
        self.on_some(changed, f);
    }

    pub fn text_box_changed(&mut self, handle: &TextBoxHandle) -> Option<String> {
        self.take_text_box_changed(handle)
    }

    pub fn on_text_box_changed_with_ui(
        &mut self,
        handle: &TextBoxHandle,
        f: impl FnOnce(&mut Ui<'a>, String),
    ) {
        let changed = self.take_text_box_changed(handle);
        self.on_some_with_ui(changed, f);
    }

    pub fn on_checkbox_changed(&mut self, handle: &CheckboxHandle, f: impl FnOnce(bool)) {
        let changed = self.take_checkbox_changed(handle);
        self.on_some(changed, f);
    }

    pub fn checkbox_changed(&mut self, handle: &CheckboxHandle) -> Option<bool> {
        self.take_checkbox_changed(handle)
    }

    pub fn on_checkbox_changed_with_ui(
        &mut self,
        handle: &CheckboxHandle,
        f: impl FnOnce(&mut Ui<'a>, bool),
    ) {
        let changed = self.take_checkbox_changed(handle);
        self.on_some_with_ui(changed, f);
    }

    pub fn on_slider_changed(&mut self, handle: &SliderHandle, f: impl FnOnce(f64)) {
        let changed = self.take_slider_changed(handle);
        self.on_some(changed, f);
    }

    pub fn slider_changed(&mut self, handle: &SliderHandle) -> Option<f64> {
        self.take_slider_changed(handle)
    }

    pub fn on_slider_changed_with_ui(
        &mut self,
        handle: &SliderHandle,
        f: impl FnOnce(&mut Ui<'a>, f64),
    ) {
        let changed = self.take_slider_changed(handle);
        self.on_some_with_ui(changed, f);
    }

    pub fn on_radio_button_changed(&mut self, handle: &RadioButtonHandle, f: impl FnOnce(bool)) {
        let changed = self.take_radio_button_changed(handle);
        self.on_some(changed, f);
    }

    pub fn radio_button_changed(&mut self, handle: &RadioButtonHandle) -> Option<bool> {
        self.take_radio_button_changed(handle)
    }

    pub fn on_radio_button_changed_with_ui(
        &mut self,
        handle: &RadioButtonHandle,
        f: impl FnOnce(&mut Ui<'a>, bool),
    ) {
        let changed = self.take_radio_button_changed(handle);
        self.on_some_with_ui(changed, f);
    }

    pub fn on_combo_box_changed(&mut self, handle: &ComboBoxHandle, f: impl FnOnce(usize, String)) {
        if let Some((selected_index, selected_text)) = self.take_combo_box_changed(handle) {
            f(selected_index, selected_text);
        }
    }

    pub fn combo_box_changed(&mut self, handle: &ComboBoxHandle) -> Option<(usize, String)> {
        self.take_combo_box_changed(handle)
    }

    pub fn on_combo_box_changed_with_ui(
        &mut self,
        handle: &ComboBoxHandle,
        f: impl FnOnce(&mut Ui<'a>, usize, String),
    ) {
        if let Some((selected_index, selected_text)) = self.take_combo_box_changed(handle) {
            f(self.ui, selected_index, selected_text);
        }
    }

    fn take_button_clicked(&mut self, handle: &ButtonHandle) -> bool {
        self.ui.retained.take_button_clicked(handle)
    }

    fn take_text_box_changed(&mut self, handle: &TextBoxHandle) -> Option<String> {
        self.ui.retained.take_text_box_changed(handle)
    }

    fn take_checkbox_changed(&mut self, handle: &CheckboxHandle) -> Option<bool> {
        self.ui.retained.take_checkbox_changed(handle)
    }

    fn take_slider_changed(&mut self, handle: &SliderHandle) -> Option<f64> {
        self.ui.retained.take_slider_changed(handle)
    }

    fn take_radio_button_changed(&mut self, handle: &RadioButtonHandle) -> Option<bool> {
        self.ui.retained.take_radio_button_changed(handle)
    }

    fn take_combo_box_changed(&mut self, handle: &ComboBoxHandle) -> Option<(usize, String)> {
        self.ui.retained.take_combo_box_changed(handle)
    }

    fn on_some<T>(&mut self, value: Option<T>, f: impl FnOnce(T)) {
        if let Some(value) = value {
            f(value);
        }
    }

    fn on_some_with_ui<T>(&mut self, value: Option<T>, f: impl FnOnce(&mut Ui<'a>, T)) {
        if let Some(value) = value {
            f(self.ui, value);
        }
    }
}
