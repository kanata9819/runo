mod events;
mod show;
mod state;
mod widgets;

use std::marker::PhantomData;

pub use events::{ActionBindings, EventBindings, EventBindingsBuilder, UiEvents};
pub use state::{
    UiButtonState, UiCheckboxState, UiComboBoxState, UiDivState, UiLabelState, UiRadioButtonState,
    UiSliderState, UiState, UiTextBoxState,
};
pub use widgets::UiWidgets;

pub(crate) use show::button::ShowButtonArgs;
pub(crate) use show::checkbox::ShowCheckboxArgs;
pub(crate) use show::combo_box::ShowComboBoxArgs;
pub(crate) use show::div::ShowDivArgs;
pub(crate) use show::label::ShowLabelArgs;
pub(crate) use show::radio_button::ShowRadioButtonArgs;
pub(crate) use show::slider::ShowSliderArgs;
pub(crate) use show::text_box::ShowTextBoxArgs;

use vello::Scene;
use vello::kurbo::{Affine, Rect};
use vello::peniko::{Fill, FontData};

use crate::Color;
use crate::hooks::use_state::StateStore;
use crate::hooks::use_effect::{EffectCleanup, EffectStore};
use crate::layout::LayoutDirection;
use crate::layout::stack::LayoutStack;
use crate::retained::RetainedState;

pub struct Ui<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) font: Option<FontData>,
    effects: &'a mut EffectStore,
    states: &'a mut StateStore,
    retained: &'a mut RetainedState,
    layout_stack: LayoutStack,
    enabled_stack: Vec<bool>,
    auto_id_counter: u64,
}

pub struct UiStateSetter<T> {
    id: String,
    marker: PhantomData<T>,
}

impl<T> UiStateSetter<T> {
    fn new(id: String) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}

impl<T> UiStateSetter<T>
where
    T: Clone + PartialEq + 'static,
{
    pub fn set(&self, ui: &mut Ui<'_>, value: T) -> bool {
        ui.set_state(self.id.clone(), value)
    }
}

impl<'a> Ui<'a> {
    pub(crate) fn new(
        scene: &'a mut Scene,
        font: Option<FontData>,
        effects: &'a mut EffectStore,
        states: &'a mut StateStore,
        retained: &'a mut RetainedState,
    ) -> Self {
        Self {
            scene,
            font,
            effects,
            states,
            retained,
            layout_stack: LayoutStack::new((24.0, 24.0), LayoutDirection::Vertical, 12.0),
            enabled_stack: vec![true],
            auto_id_counter: 0,
        }
    }

    pub fn widgets(&mut self) -> UiWidgets<'_, 'a> {
        UiWidgets { ui: self }
    }

    pub fn state(&mut self) -> UiState<'_, 'a> {
        UiState { ui: self }
    }

    pub fn events(&mut self) -> UiEvents<'_, 'a> {
        UiEvents { ui: self }
    }

    pub fn drain_bound_events<E>(&mut self, bindings: &EventBindings<E>) -> Vec<E> {
        self.events().drain_bound_events(bindings)
    }

    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.with_layout(LayoutDirection::Vertical, 12.0, f)
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.with_layout(LayoutDirection::Horizontal, 12.0, f)
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: Color) {
        let rect = Rect::new(x, y, x + w, y + h);
        self.scene
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rect);
    }

    pub fn use_effect<D, F>(&mut self, id: impl Into<String>, deps: D, effect: F)
    where
        D: std::hash::Hash,
        F: FnOnce() -> Option<EffectCleanup>,
    {
        self.effects.use_effect(id, deps, effect);
    }

    pub fn use_state<T, F>(&mut self, id: impl Into<String>, init: F) -> (T, UiStateSetter<T>)
    where
        T: Clone + 'static,
        F: FnOnce() -> T,
    {
        let id = id.into();
        let value = self.states.use_state(id.clone(), init);
        (value, UiStateSetter::new(id))
    }

    pub fn set_state<T>(&mut self, id: impl Into<String>, value: T) -> bool
    where
        T: Clone + PartialEq + 'static,
    {
        self.states.set_state(id, value)
    }

    pub(crate) fn button(&mut self) -> crate::widget::button::ButtonBuilder<'_, 'a> {
        let id = format!("__auto_button_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::button::ButtonBuilder::new(self, id)
    }

    pub(crate) fn label(&mut self) -> crate::widget::label::LabelBuilder<'_, 'a> {
        let id = format!("__auto_label_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::label::LabelBuilder::new(self, id)
    }

    pub(crate) fn checkbox(&mut self) -> crate::widget::checkbox::CheckboxBuilder<'_, 'a> {
        let id = format!("__auto_checkbox_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::checkbox::CheckboxBuilder::new(self, id)
    }

    pub(crate) fn text_box(&mut self) -> crate::widget::text_box::TextBoxBuilder<'_, 'a> {
        let id = format!("__auto_text_box_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::text_box::TextBoxBuilder::new(self, id)
    }

    pub(crate) fn combo_box(&mut self) -> crate::widget::combo_box::ComboBoxBuilder<'_, 'a> {
        let id = format!("__auto_combo_box_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::combo_box::ComboBoxBuilder::new(self, id)
    }

    pub(crate) fn slider(&mut self) -> crate::widget::slider::SliderBuilder<'_, 'a> {
        let id = format!("__auto_slider_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::slider::SliderBuilder::new(self, id)
    }

    pub(crate) fn radio_button(
        &mut self,
    ) -> crate::widget::radio_button::RadioButtonBuilder<'_, 'a> {
        let id = format!("__auto_radio_button_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::widget::radio_button::RadioButtonBuilder::new(self, id)
    }

    pub(crate) fn div(&mut self) -> crate::layout::div::DivBuilder<'_, 'a> {
        let id = format!("__auto_div_{}", self.auto_id_counter);
        self.auto_id_counter += 1;
        crate::layout::div::DivBuilder::new(self, id)
    }

    pub(crate) fn button_response(&self, id: impl AsRef<str>) -> crate::ButtonResponse {
        self.retained.button_response(id)
    }

    pub(crate) fn set_button_text(&mut self, id: impl AsRef<str>, text: Option<String>) {
        self.retained.set_button_text(id, text);
    }

    pub(crate) fn set_button_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.retained.set_button_enabled(id, enabled);
    }

    fn with_layout<R>(
        &mut self,
        direction: LayoutDirection,
        spacing: f64,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.layout_stack.push_layout(direction, spacing);
        let result = f(self);
        self.layout_stack.pop_layout_and_advance_parent();
        result
    }

    pub(crate) fn allocate_rect(&mut self, width: f64, height: f64) -> (f64, f64) {
        self.layout_stack.allocate_rect(width, height)
    }

    fn current_enabled(&self) -> bool {
        self.enabled_stack.last().copied().unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use vello::Scene;

    use crate::Color;
    use crate::hooks::use_state::StateStore;
    use crate::hooks::use_effect::EffectStore;
    use crate::retained::RetainedState;

    use super::Ui;

    #[test]
    fn ui_layout_helpers_and_fill_rect_are_callable() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        ui.fill_rect(0.0, 0.0, 20.0, 10.0, Color::from_rgb8(10, 20, 30));

        ui.vertical(|ui| {
            ui.widgets().label().id("v1").text("v1").show();
            ui.widgets().label().id("v2").text("v2").show();
        });
        ui.horizontal(|ui| {
            ui.widgets().label().id("h1").text("h1").show();
            ui.widgets().label().id("h2").text("h2").show();
        });
    }

    #[test]
    fn ui_use_effect_delegates_to_effect_store() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);
        let calls = Rc::new(RefCell::new(0usize));

        ui.use_effect("e", 1_u32, {
            let calls = Rc::clone(&calls);
            move || {
                *calls.borrow_mut() += 1;
                None
            }
        });
        // unchanged deps for same id should not invoke again.
        ui.use_effect("e", 1_u32, {
            let calls = Rc::clone(&calls);
            move || {
                *calls.borrow_mut() += 1;
                None
            }
        });
        assert_eq!(*calls.borrow(), 1);
    }

    #[test]
    fn ui_use_state_and_setter_roundtrip() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let (value, set_value) = ui.use_state("counter", || 7_u32);
        assert_eq!(value, 7);
        assert!(set_value.set(&mut ui, 9_u32));
        let (value2, _) = ui.use_state("counter", || 0_u32);
        assert_eq!(value2, 9);
    }
}
