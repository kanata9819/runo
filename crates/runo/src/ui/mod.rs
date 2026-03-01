//! Immediate-style facade over the retained core.
//!
//! Application code uses `Ui` builders each frame, while `Ui` writes into
//! `retained::state` so interaction and rendering remain stable across frames.
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

#[cfg(test)]
#[path = "../../tests/unit/ui/mod.rs"]
mod tests;

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
use crate::hooks::use_effect::{EffectCleanup, EffectStore};
use crate::hooks::use_state::StateStore;
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
    key_scope_stack: Vec<String>,
    auto_id_counter_stack: Vec<u64>,
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
            key_scope_stack: Vec::new(),
            auto_id_counter_stack: vec![0],
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

    pub fn with_stable_key<R, K>(&mut self, key: K, f: impl FnOnce(&mut Self) -> R) -> R
    where
        K: Into<String>,
    {
        self.key_scope_stack.push(key.into());
        self.auto_id_counter_stack.push(0);
        let result = f(self);
        let _ = self.auto_id_counter_stack.pop();
        let _ = self.key_scope_stack.pop();

        result
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
        let id = self.next_auto_id("button");
        crate::widget::button::ButtonBuilder::new(self, id)
    }

    pub(crate) fn label(&mut self) -> crate::widget::label::LabelBuilder<'_, 'a> {
        let id = self.next_auto_id("label");
        crate::widget::label::LabelBuilder::new(self, id)
    }

    pub(crate) fn checkbox(&mut self) -> crate::widget::checkbox::CheckboxBuilder<'_, 'a> {
        let id = self.next_auto_id("checkbox");
        crate::widget::checkbox::CheckboxBuilder::new(self, id)
    }

    pub(crate) fn text_box(&mut self) -> crate::widget::text_box::TextBoxBuilder<'_, 'a> {
        let id = self.next_auto_id("text_box");
        crate::widget::text_box::TextBoxBuilder::new(self, id)
    }

    pub(crate) fn combo_box(&mut self) -> crate::widget::combo_box::ComboBoxBuilder<'_, 'a> {
        let id = self.next_auto_id("combo_box");
        crate::widget::combo_box::ComboBoxBuilder::new(self, id)
    }

    pub(crate) fn slider(&mut self) -> crate::widget::slider::SliderBuilder<'_, 'a> {
        let id = self.next_auto_id("slider");
        crate::widget::slider::SliderBuilder::new(self, id)
    }

    pub(crate) fn radio_button(
        &mut self,
    ) -> crate::widget::radio_button::RadioButtonBuilder<'_, 'a> {
        let id = self.next_auto_id("radio_button");
        crate::widget::radio_button::RadioButtonBuilder::new(self, id)
    }

    pub(crate) fn div(&mut self) -> crate::layout::div::DivBuilder<'_, 'a> {
        let id = self.next_auto_id("div");
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

    pub(crate) fn allocate_widget_rect(&mut self, width: f64, height: f64) -> Rect {
        let (x, y) = self.allocate_rect(width, height);
        Rect::new(x, y, x + width, y + height)
    }

    fn current_enabled(&self) -> bool {
        self.enabled_stack.last().copied().unwrap_or(true)
    }

    pub(crate) fn resolve_enabled(&self, enabled: bool) -> bool {
        enabled && self.current_enabled()
    }

    fn next_auto_id(&mut self, kind: &str) -> String {
        let counter = self
            .auto_id_counter_stack
            .last_mut()
            .expect("auto id counter stack should never be empty");

        let index = *counter;
        *counter += 1;

        if self.key_scope_stack.is_empty() {
            format!("__auto_{kind}_{index}")
        } else {
            let scope = self.key_scope_stack.join(".");
            format!("__auto_{kind}_{scope}_{index}")
        }
    }
}
