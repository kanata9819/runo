#[derive(Clone, Debug)]
pub enum UiEvent {
    ButtonClicked { id: String },
    TextBoxChanged { id: String, text: String },
}
