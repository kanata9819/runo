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
