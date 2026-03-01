mod app;
mod cache;
mod event;
mod font;
mod hooks;
mod input;
mod layout;
mod option_ext;
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

pub mod prelude {
    pub use crate::option_ext::{
        OptionalButtonHandleExt, OptionalCheckboxHandleExt, OptionalComboBoxHandleExt,
        OptionalRadioButtonHandleExt, OptionalSliderHandleExt, OptionalTextBoxHandleExt,
    };
}
