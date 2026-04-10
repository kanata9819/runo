use super::Ui;
use crate::layout::div::DivBuilder;
use crate::widget::button::ButtonBuilder;
use crate::widget::checkbox::CheckboxBuilder;
use crate::widget::combo_box::ComboBoxBuilder;
use crate::widget::label::LabelBuilder;
use crate::widget::radio_button::RadioButtonBuilder;
use crate::widget::slider::SliderBuilder;
use crate::widget::text_box::TextBoxBuilder;

#[cfg(test)]
#[path = "../../tests/unit/ui/widgets.rs"]
mod tests;

pub struct UiWidgets<'ui, 'a> {
    pub(super) ui: &'ui mut Ui<'a>,
}

impl<'ui, 'a> UiWidgets<'ui, 'a> {
    #[must_use] 
    pub fn button(self) -> ButtonBuilder<'ui, 'a> {
        self.ui.button()
    }

    #[must_use] 
    pub fn label(self) -> LabelBuilder<'ui, 'a> {
        self.ui.label()
    }

    #[must_use] 
    pub fn checkbox(self) -> CheckboxBuilder<'ui, 'a> {
        self.ui.checkbox()
    }

    #[must_use] 
    pub fn text_box(self) -> TextBoxBuilder<'ui, 'a> {
        self.ui.text_box()
    }

    #[must_use] 
    pub fn combo_box(self) -> ComboBoxBuilder<'ui, 'a> {
        self.ui.combo_box()
    }

    #[must_use] 
    pub fn radio_button(self) -> RadioButtonBuilder<'ui, 'a> {
        self.ui.radio_button()
    }

    #[must_use] 
    pub fn slider(self) -> SliderBuilder<'ui, 'a> {
        self.ui.slider()
    }

    #[must_use] 
    pub fn div(self) -> DivBuilder<'ui, 'a> {
        self.ui.div()
    }
}
