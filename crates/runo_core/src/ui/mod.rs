pub mod colors;
mod events;
mod show;
mod state;
mod widgets;

pub use events::UiEvents;
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
use crate::hooks::effect::{EffectCleanup, EffectStore};
use crate::layout::LayoutDirection;
use crate::layout::stack::LayoutStack;
use crate::retained::RetainedState;

pub struct Ui<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) font: Option<FontData>,
    effects: &'a mut EffectStore,
    retained: &'a mut RetainedState,
    layout_stack: LayoutStack,
    enabled_stack: Vec<bool>,
    auto_id_counter: u64,
}

impl<'a> Ui<'a> {
    pub(crate) fn new(
        scene: &'a mut Scene,
        font: Option<FontData>,
        effects: &'a mut EffectStore,
        retained: &'a mut RetainedState,
    ) -> Self {
        Self {
            scene,
            font,
            effects,
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
