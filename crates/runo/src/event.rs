use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

#[cfg(test)]
#[path = "../tests/unit/event.rs"]
mod tests;

#[derive(Clone, Debug)]
pub enum UiEvent {
    ButtonClicked {
        button: ButtonHandle,
    },

    CheckboxChanged {
        checkbox: CheckboxHandle,
        checked: bool,
    },

    RadioButtonChanged {
        radio_button: RadioButtonHandle,
        group: String,
        selected: bool,
    },

    SliderChanged {
        slider: SliderHandle,
        value: f64,
    },

    TextBoxChanged {
        text_box: TextBoxHandle,
        text: String,
    },

    ComboBoxChanged {
        combo_box: ComboBoxHandle,
        selected_index: usize,
        selected_text: String,
    },
}
