use vello::kurbo::Rect;

use crate::retained::UpsertTextBoxArgs;
use crate::ui::Ui;
use crate::widget::text_box::TextBoxResponse;
use crate::widget_model::text_box::TextBoxCommon;

pub(crate) struct ShowTextBoxArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) common: TextBoxCommon,
}

impl Ui<'_> {
    pub(crate) fn show_text_box(&mut self, args: ShowTextBoxArgs) -> TextBoxResponse {
        let ShowTextBoxArgs {
            id,
            width,
            height,
            mut common,
        } = args;
        let rect: Rect = self.allocate_widget_rect(width, height);
        common.enabled = self.resolve_enabled(common.enabled);
        let TextBoxCommon {
            text,
            placeholder,
            font_size,
            text_color,
            bg_color,
            border_color,
            disable_border,
            enabled,
            read_only,
            overflow_x,
            overflow_y,
        } = common;
        self.retained.upsert_text_box(UpsertTextBoxArgs {
            id,
            rect,
            text,
            placeholder,
            font_size,
            text_color,
            bg_color,
            border_color,
            disable_border,
            enabled,
            read_only,
            overflow_x,
            overflow_y,
        })
    }
}
