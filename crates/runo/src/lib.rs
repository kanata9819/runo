//! `runo` is a retained-mode oriented GUI core on top of `winit + wgpu + vello`.
//!
//! Runtime flow (high level):
//! 1. `ui` builds widgets each frame and upserts them into `retained::state`.
//! 2. `retained::input` updates interaction flags and emits `UiEvent`s.
//! 3. `retained::paint` renders nodes in stable order (base layer -> overlay layer).
//! 4. `ui::events` exposes events/actions to application code.
//!
//! This crate keeps widget state in a central retained tree so builders stay lightweight.
mod app;
mod cache;
mod event;
mod font;
mod hooks;
mod input;
mod layout;
mod retained;
mod theme;
mod ui;
mod widget;

pub use app::{RunOptions, RunoApplication, run};
pub use event::UiEvent;
pub use hooks::use_effect::EffectCleanup;
pub use layout::div::DivHandle;
pub use theme::color as colors;
pub use ui::{
    ActionBindings, EventBindings, EventBindingsBuilder, Ui, UiButtonState, UiCheckboxState,
    UiComboBoxState, UiDivState, UiEvents, UiLabelState, UiRadioButtonState, UiSliderState,
    UiState, UiStateSetter, UiTextBoxState, UiWidgets,
};
#[cfg(test)]
#[path = "../tests/unit/lib.rs"]
mod tests;
pub use vello::peniko::Color;
pub use widget::button::ButtonHandle;
pub use widget::button::ButtonResponse;
pub use widget::checkbox::CheckboxHandle;
pub use widget::checkbox::CheckboxResponse;
pub use widget::combo_box::ComboBoxHandle;
pub use widget::combo_box::ComboBoxResponse;
pub use widget::label::LabelHandle;
pub use widget::radio_button::RadioButtonHandle;
pub use widget::radio_button::RadioButtonResponse;
pub use widget::slider::SliderHandle;
pub use widget::slider::SliderResponse;
pub use widget::text_box::TextBoxHandle;
pub use widget::text_box::{Overflow, TextBoxResponse};
