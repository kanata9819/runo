use vello::kurbo::Rect;

use crate::Color;
use crate::RadioButtonResponse;
use crate::retained::UpsertRadioButtonArgs;
use crate::ui::Ui;

pub(crate) struct ShowRadioButtonArgs {
    pub(crate) id: String,
    pub(crate) group: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) text: Option<String>,
    pub(crate) selected: Option<bool>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_radio_button(&mut self, args: ShowRadioButtonArgs) -> RadioButtonResponse {
        let ShowRadioButtonArgs {
            id,
            group,
            width,
            height,
            text,
            selected,
            font_size,
            text_color,
            enabled,
        } = args;
        let (x, y) = self.allocate_rect(width, height);
        let rect = Rect::new(x, y, x + width, y + height);
        self.retained.upsert_radio_button(UpsertRadioButtonArgs {
            id,
            group,
            rect,
            text,
            selected,
            font_size,
            text_color,
            enabled: enabled && self.current_enabled(),
        })
    }
}
