use std::marker::PhantomData;

use vello::Scene;
use vello::peniko::FontData;

use super::{EventBindings, UiEvents, UiState, UiWidgets};
use crate::hooks::use_effect::{EffectCleanup, EffectStore};
use crate::hooks::use_state::StateStore;
use crate::input::UiInputSnapshot;
use crate::layout::LayoutDirection;
use crate::layout::stack::LayoutStack;
use crate::retained::RetainedState;

#[cfg(test)]
#[path = "../../tests/unit/ui/mod.rs"]
mod tests;

pub struct Ui<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) font: Option<FontData>,
    pub(super) input: UiInputSnapshot,
    pub(super) effects: &'a mut EffectStore,
    pub(super) states: &'a mut StateStore,
    pub(super) retained: &'a mut RetainedState,
    pub(super) layout_stack: LayoutStack,
    pub(super) enabled_stack: Vec<bool>,
    pub(super) key_scope_stack: Vec<String>,
    pub(super) auto_id_counter_stack: Vec<u64>,
}

pub struct UiStateSetter<T> {
    id: String,
    marker: PhantomData<T>,
}

impl<T> UiStateSetter<T> {
    pub(super) fn new(id: String) -> Self {
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
    #[allow(dead_code)]
    pub(crate) fn new(
        scene: &'a mut Scene,
        font: Option<FontData>,
        effects: &'a mut EffectStore,
        states: &'a mut StateStore,
        retained: &'a mut RetainedState,
    ) -> Self {
        Self::with_input(
            scene,
            font,
            UiInputSnapshot::default(),
            effects,
            states,
            retained,
        )
    }

    pub(crate) fn with_input(
        scene: &'a mut Scene,
        font: Option<FontData>,
        input: UiInputSnapshot,
        effects: &'a mut EffectStore,
        states: &'a mut StateStore,
        retained: &'a mut RetainedState,
    ) -> Self {
        Self {
            scene,
            font,
            input,
            effects,
            states,
            retained,
            layout_stack: LayoutStack::new((0.0, 0.0), LayoutDirection::Vertical, 12.0),
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

    pub fn input(&self) -> &UiInputSnapshot {
        &self.input
    }

    pub fn drain_bound_events<E>(&mut self, bindings: &EventBindings<E>) -> Vec<E> {
        self.events().drain_bound_events(bindings)
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
        let id: String = id.into();
        let value: T = self.states.use_state(id.clone(), init);
        (value, UiStateSetter::new(id))
    }

    pub fn set_state<T>(&mut self, id: impl Into<String>, value: T) -> bool
    where
        T: Clone + PartialEq + 'static,
    {
        self.states.set_state(id, value)
    }
}
