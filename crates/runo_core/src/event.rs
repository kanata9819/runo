#[derive(Clone, Debug)]
pub enum UiEvent {
    ButtonClicked {
        id: String,
    },
    CheckboxChanged {
        id: String,
        checked: bool,
    },
    RadioButtonChanged {
        id: String,
        group: String,
        selected: bool,
    },
    SliderChanged {
        id: String,
        value: f64,
    },
    TextBoxChanged {
        id: String,
        text: String,
    },
    ComboBoxChanged {
        id: String,
        selected_index: usize,
        selected_text: String,
    },
}

#[cfg(test)]
mod tests {
    use super::UiEvent;

    #[test]
    fn ui_event_is_cloneable() {
        let event = UiEvent::SliderChanged {
            id: "s".to_string(),
            value: 0.5,
        };
        let cloned = event.clone();
        match cloned {
            UiEvent::SliderChanged { id, value } => {
                assert_eq!(id, "s");
                assert!((value - 0.5).abs() < f64::EPSILON);
            }
            _ => panic!("unexpected variant"),
        }
    }
}
