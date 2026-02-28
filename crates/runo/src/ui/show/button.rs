use vello::kurbo::Rect;

use crate::ButtonResponse;
use crate::Color;
use crate::ui::Ui;

pub(crate) struct ShowButtonArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) text: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_button(&mut self, args: ShowButtonArgs) -> ButtonResponse {
        let ShowButtonArgs {
            id,
            width,
            height,
            text,
            font_size,
            text_color,
            enabled,
        } = args;

        let (x, y) = self.allocate_rect(width, height);
        let rect = Rect::new(x, y, x + width, y + height);
        self.retained.upsert_button(
            id,
            rect,
            text,
            font_size,
            text_color,
            enabled && self.current_enabled(),
        )
    }
}
