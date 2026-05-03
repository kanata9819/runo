use super::Ui;

impl<'a> Ui<'a> {
    pub(crate) fn button(&mut self) -> crate::widget::button::ButtonBuilder<'_, 'a> {
        let id: String = self.next_auto_id("button");
        crate::widget::button::ButtonBuilder::new(self, id)
    }

    pub(crate) fn label(&mut self) -> crate::widget::label::LabelBuilder<'_, 'a> {
        let id: String = self.next_auto_id("label");
        crate::widget::label::LabelBuilder::new(self, id)
    }

    pub(crate) fn checkbox(&mut self) -> crate::widget::checkbox::CheckboxBuilder<'_, 'a> {
        let id: String = self.next_auto_id("checkbox");
        crate::widget::checkbox::CheckboxBuilder::new(self, id)
    }

    pub(crate) fn text_box(&mut self) -> crate::widget::text_box::TextBoxBuilder<'_, 'a> {
        let id: String = self.next_auto_id("text_box");
        crate::widget::text_box::TextBoxBuilder::new(self, id)
    }

    pub(crate) fn terminal_view(
        &mut self,
    ) -> crate::widget::terminal_view::TerminalViewBuilder<'_, 'a> {
        let id: String = self.next_auto_id("terminal_view");
        crate::widget::terminal_view::TerminalViewBuilder::new(self, id)
    }

    pub(crate) fn combo_box(&mut self) -> crate::widget::combo_box::ComboBoxBuilder<'_, 'a> {
        let id: String = self.next_auto_id("combo_box");
        crate::widget::combo_box::ComboBoxBuilder::new(self, id)
    }

    pub(crate) fn slider(&mut self) -> crate::widget::slider::SliderBuilder<'_, 'a> {
        let id: String = self.next_auto_id("slider");
        crate::widget::slider::SliderBuilder::new(self, id)
    }

    pub(crate) fn radio_button(
        &mut self,
    ) -> crate::widget::radio_button::RadioButtonBuilder<'_, 'a> {
        let id: String = self.next_auto_id("radio_button");
        crate::widget::radio_button::RadioButtonBuilder::new(self, id)
    }

    pub(crate) fn div(&mut self) -> crate::layout::div::DivBuilder<'_, 'a> {
        let id: String = self.next_auto_id("div");
        crate::layout::div::DivBuilder::new(self, id)
    }

    pub(crate) fn button_response(&self, id: impl AsRef<str>) -> crate::ButtonResponse {
        self.retained.button_response(id)
    }

    pub(crate) fn set_button_text(&mut self, id: impl AsRef<str>, text: Option<String>) {
        self.retained.set_button_text(id, text);
    }

    pub(crate) fn set_button_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        self.retained.set_button_enabled(id, enabled);
    }

    fn next_auto_id(&mut self, kind: &str) -> String {
        let counter: &mut u64 = self
            .auto_id_counter_stack
            .last_mut()
            .expect("auto id counter stack should never be empty");

        let index: u64 = *counter;
        *counter += 1;

        if self.key_scope_stack.is_empty() {
            format!("__auto_{kind}_{index}")
        } else {
            let scope: String = self.key_scope_stack.join(".");
            format!("__auto_{kind}_{scope}_{index}")
        }
    }
}
