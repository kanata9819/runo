use vello::kurbo::Rect;

use crate::CheckboxResponse;
use crate::retained::UpsertCheckboxArgs;
use crate::ui::Ui;
use crate::widget_model::checkbox::CheckboxCommon;

pub(crate) struct ShowCheckboxArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) common: CheckboxCommon,
}

impl Ui<'_> {
    pub(crate) fn show_checkbox(&mut self, args: ShowCheckboxArgs) -> CheckboxResponse {
        let ShowCheckboxArgs {
            id,
            width,
            height,
            mut common,
        } = args;
        let rect: Rect = self.allocate_widget_rect(width, height);
        common.enabled = self.resolve_enabled(common.enabled);
        let CheckboxCommon {
            text,
            checked,
            font_size,
            text_color,
            enabled,
        } = common;
        self.retained.upsert_checkbox(UpsertCheckboxArgs {
            id,
            rect,
            text,
            checked,
            font_size,
            text_color,
            enabled,
        })
    }
}
