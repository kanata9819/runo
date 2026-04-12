use vello::kurbo::Rect;

use crate::ComboBoxResponse;
use crate::retained::UpsertComboBoxArgs;
use crate::ui::Ui;
use crate::widget_model::combo_box::ComboBoxCommon;

pub(crate) struct ShowComboBoxArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) common: ComboBoxCommon,
}

impl Ui<'_> {
    pub(crate) fn show_combo_box(&mut self, args: ShowComboBoxArgs) -> ComboBoxResponse {
        let ShowComboBoxArgs {
            id,
            width,
            height,
            mut common,
        } = args;
        let rect: Rect = self.allocate_widget_rect(width, height);
        common.enabled = self.resolve_enabled(common.enabled);
        let ComboBoxCommon {
            items,
            selected_index,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled,
        } = common;
        self.retained.upsert_combo_box(UpsertComboBoxArgs {
            id,
            rect,
            items,
            selected_index,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled,
        })
    }
}
