#[derive(Clone, Debug)]
pub enum UiEvent {
    ButtonClicked {
        id: String,
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
