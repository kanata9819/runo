use vello::kurbo::Rect;

use crate::RadioButtonResponse;
use crate::retained::UpsertRadioButtonArgs;
use crate::ui::Ui;
use crate::widget_model::radio_button::RadioButtonCommon;

pub(crate) struct ShowRadioButtonArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) common: RadioButtonCommon,
}

impl Ui<'_> {
    pub(crate) fn show_radio_button(&mut self, args: ShowRadioButtonArgs) -> RadioButtonResponse {
        let ShowRadioButtonArgs {
            id,
            width,
            height,
            mut common,
        } = args;
        let rect: Rect = self.allocate_widget_rect(width, height);
        common.enabled = self.resolve_enabled(common.enabled);
        let RadioButtonCommon {
            group,
            text,
            selected,
            font_size,
            text_color,
            enabled,
        } = common;
        self.retained.upsert_radio_button(UpsertRadioButtonArgs {
            id,
            rect,
            group,
            text,
            selected,
            font_size,
            text_color,
            enabled,
        })
    }
}
