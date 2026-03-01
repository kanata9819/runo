use vello::peniko::Color;

use crate::ComboBoxResponse;
use crate::retained::UpsertComboBoxArgs;
use crate::ui::Ui;

pub(crate) struct ShowComboBoxArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) items: Vec<String>,
    pub(crate) selected_index: Option<usize>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) enabled: bool,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_combo_box(&mut self, args: ShowComboBoxArgs) -> ComboBoxResponse {
        let ShowComboBoxArgs {
            id,
            width,
            height,
            items,
            selected_index,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled: enabled_arg,
        } = args;
        let rect = self.allocate_widget_rect(width, height);
        self.retained.upsert_combo_box(UpsertComboBoxArgs {
            id,
            rect,
            items,
            selected_index,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled: self.resolve_enabled(enabled_arg),
        })
    }
}
