//! Immediate-style facade over the retained core.
//!
//! Application code talks to `Ui`; `Ui` delegates to small modules for
//! builders, events, state, layout, drawing, and hooks.
mod core;
mod drawing;
mod events;
mod layout_scope;
mod show;
mod state;
mod widget_factories;
mod widgets;

pub use core::{Ui, UiStateSetter};
pub use events::{ActionBindings, EventBindings, EventBindingsBuilder, UiEvents};
#[allow(unused_imports)]
pub use state::{
    UiButtonState, UiCheckboxState, UiComboBoxState, UiDivState, UiLabelState, UiRadioButtonState,
    UiSliderState, UiState, UiTerminalViewState, UiTextBoxState,
};
pub use widgets::UiWidgets;

pub(crate) use show::button::ShowButtonArgs;
pub(crate) use show::checkbox::ShowCheckboxArgs;
pub(crate) use show::combo_box::ShowComboBoxArgs;
pub(crate) use show::div::ShowDivArgs;
pub(crate) use show::label::ShowLabelArgs;
pub(crate) use show::radio_button::ShowRadioButtonArgs;
pub(crate) use show::slider::ShowSliderArgs;
pub(crate) use show::terminal_view::ShowTerminalViewArgs;
pub(crate) use show::text_box::ShowTextBoxArgs;
