use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::UpsertTextBoxArgs;
use crate::ui::Ui;
use crate::widget::text_box::{Overflow, TextBoxResponse};

pub(crate) struct ShowTextBoxArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) text: Option<String>,
    pub(crate) placeholder: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) enabled: bool,
    pub(crate) overflow_x: Overflow,
    pub(crate) overflow_y: Overflow,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_text_box(&mut self, args: ShowTextBoxArgs) -> TextBoxResponse {
        let ShowTextBoxArgs {
            id,
            width,
            height,
            text,
            placeholder,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled: enabled_arg,
            overflow_x,
            overflow_y,
        } = args;
        let (x, y) = self.allocate_rect(width, height);
        let rect = Rect::new(x, y, x + width, y + height);
        self.retained.upsert_text_box(UpsertTextBoxArgs {
            id,
            rect,
            text,
            placeholder,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled: enabled_arg && self.current_enabled(),
            overflow_x,
            overflow_y,
        })
    }
}
