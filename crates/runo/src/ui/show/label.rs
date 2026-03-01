use crate::Color;
use crate::ui::Ui;

pub(crate) struct ShowLabelArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) text: String,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl<'a> Ui<'a> {
    pub(crate) fn show_label(&mut self, args: ShowLabelArgs) {
        let ShowLabelArgs {
            id,
            width,
            height,
            text,
            font_size,
            text_color,
            enabled,
        } = args;
        let rect = self.allocate_widget_rect(width, height);
        self.retained.upsert_label(
            id,
            rect,
            text,
            font_size,
            text_color,
            self.resolve_enabled(enabled),
        );
    }
}
