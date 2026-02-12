use super::Ui;
use crate::layout::div::DivBuilder;
use crate::widget::button::ButtonBuilder;
use crate::widget::checkbox::CheckboxBuilder;
use crate::widget::combo_box::ComboBoxBuilder;
use crate::widget::label::LabelBuilder;
use crate::widget::radio_button::RadioButtonBuilder;
use crate::widget::slider::SliderBuilder;
use crate::widget::text_box::TextBoxBuilder;

pub struct UiWidgets<'ui, 'a> {
    pub(super) ui: &'ui mut Ui<'a>,
}

impl<'ui, 'a> UiWidgets<'ui, 'a> {
    pub fn button(self) -> ButtonBuilder<'ui, 'a> {
        self.ui.button()
    }

    pub fn label(self) -> LabelBuilder<'ui, 'a> {
        self.ui.label()
    }

    pub fn checkbox(self) -> CheckboxBuilder<'ui, 'a> {
        self.ui.checkbox()
    }

    pub fn text_box(self) -> TextBoxBuilder<'ui, 'a> {
        self.ui.text_box()
    }

    pub fn combo_box(self) -> ComboBoxBuilder<'ui, 'a> {
        self.ui.combo_box()
    }

    pub fn radio_button(self) -> RadioButtonBuilder<'ui, 'a> {
        self.ui.radio_button()
    }

    pub fn slider(self) -> SliderBuilder<'ui, 'a> {
        self.ui.slider()
    }

    pub fn div(self) -> DivBuilder<'ui, 'a> {
        self.ui.div()
    }
}
