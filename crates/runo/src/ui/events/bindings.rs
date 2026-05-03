use std::collections::HashMap;

use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

pub struct ActionBindings<A> {
    pub(super) button: HashMap<ButtonHandle, A>,
    pub(super) checkbox: HashMap<CheckboxHandle, A>,
    pub(super) radio_button: HashMap<RadioButtonHandle, A>,
    pub(super) slider: HashMap<SliderHandle, A>,
    pub(super) text_box: HashMap<TextBoxHandle, A>,
    pub(super) combo_box: HashMap<ComboBoxHandle, A>,
}

pub struct EventBindings<E> {
    pub(super) button: HashMap<ButtonHandle, Box<dyn Fn() -> E>>,
    pub(super) checkbox: HashMap<CheckboxHandle, Box<dyn Fn(bool) -> E>>,
    pub(super) radio_button: HashMap<RadioButtonHandle, Box<dyn Fn(bool) -> E>>,
    pub(super) slider: HashMap<SliderHandle, Box<dyn Fn(f64) -> E>>,
    pub(super) text_box: HashMap<TextBoxHandle, Box<dyn Fn(String) -> E>>,
    pub(super) combo_box: HashMap<ComboBoxHandle, Box<dyn Fn(usize, String) -> E>>,
}

pub struct EventBindingsBuilder<E> {
    bindings: EventBindings<E>,
}

impl<A> ActionBindings<A> {
    #[must_use]
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
    #[must_use]
    pub fn builder() -> EventBindingsBuilder<E> {
        EventBindingsBuilder::new()
    }

    #[must_use]
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
    #[must_use]
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

    #[must_use]
    pub fn extend(mut self, other: EventBindings<E>) -> Self {
        self.bindings.button.extend(other.button);
        self.bindings.checkbox.extend(other.checkbox);
        self.bindings.radio_button.extend(other.radio_button);
        self.bindings.slider.extend(other.slider);
        self.bindings.text_box.extend(other.text_box);
        self.bindings.combo_box.extend(other.combo_box);
        self
    }

    #[must_use]
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
