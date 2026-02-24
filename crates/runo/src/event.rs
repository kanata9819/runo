use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

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

#[cfg(test)]
mod tests {
    use super::UiEvent;
    use crate::widget::slider::SliderHandle;

    #[test]
    fn ui_event_is_cloneable() {
        let event = UiEvent::SliderChanged {
            slider: SliderHandle::new("s".to_string()),
            value: 0.5,
        };
        let cloned = event.clone();
        match cloned {
            UiEvent::SliderChanged { slider, value } => {
                assert_eq!(slider.id(), "s");
                assert!((value - 0.5).abs() < f64::EPSILON);
            }
            _ => panic!("unexpected variant"),
        }
    }
}
