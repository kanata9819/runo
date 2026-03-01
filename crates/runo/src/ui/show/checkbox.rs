use crate::CheckboxResponse;
use crate::Color;
use crate::retained::UpsertCheckboxArgs;
use crate::ui::Ui;

pub(crate) struct ShowCheckboxArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) text: Option<String>,
    pub(crate) checked: Option<bool>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_checkbox(&mut self, args: ShowCheckboxArgs) -> CheckboxResponse {
        let ShowCheckboxArgs {
            id,
            width,
            height,
            text,
            checked,
            font_size,
            text_color,
            enabled,
        } = args;
        let rect = self.allocate_widget_rect(width, height);
        self.retained.upsert_checkbox(UpsertCheckboxArgs {
            id,
            rect,
            text,
            checked,
            font_size,
            text_color,
            enabled: self.resolve_enabled(enabled),
        })
    }
}
