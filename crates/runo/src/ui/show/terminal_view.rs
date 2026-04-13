use vello::kurbo::Rect;

use crate::ui::Ui;
use crate::widget::terminal_view::{TerminalBuffer, TerminalViewResponse};

pub(crate) struct ShowTerminalViewArgs {
    pub(crate) id: String,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) buffer: TerminalBuffer,
    pub(crate) font_size: f32,
    pub(crate) text_color: vello::peniko::Color,
    pub(crate) bg_color: vello::peniko::Color,
    pub(crate) border_color: vello::peniko::Color,
    pub(crate) disable_border: bool,
    pub(crate) enabled: bool,
}

impl Ui<'_> {
    pub(crate) fn show_terminal_view(
        &mut self,
        args: ShowTerminalViewArgs,
    ) -> TerminalViewResponse {
        let ShowTerminalViewArgs {
            id,
            width,
            height,
            buffer,
            font_size,
            text_color,
            bg_color,
            border_color,
            disable_border,
            enabled,
        } = args;
        let rect: Rect = self.allocate_widget_rect(width, height);
        self.retained
            .upsert_terminal_view(crate::retained::UpsertTerminalViewArgs {
                id,
                rect,
                buffer,
                font_size,
                text_color,
                bg_color,
                border_color,
                disable_border,
                enabled: self.resolve_enabled(enabled),
            })
    }
}
