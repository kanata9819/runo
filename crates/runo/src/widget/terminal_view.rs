use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowTerminalViewArgs;

#[cfg(test)]
#[path = "../../tests/unit/widget/terminal_view.rs"]
mod tests;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TerminalCell {
    pub ch: char,
    pub faint: bool,
}

impl TerminalCell {
    pub fn new(ch: char) -> Self {
        Self { ch, faint: false }
    }

    pub fn faint(mut self, value: bool) -> Self {
        self.faint = value;
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TerminalBuffer {
    pub lines: Vec<Vec<TerminalCell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

#[derive(Clone, Debug, Default)]
pub struct TerminalViewResponse {
    pub hovered: bool,
}

pub struct TerminalViewBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    buffer: TerminalBuffer,
    font_size: f32,
    text_color: Color,
    bg_color: Color,
    border_color: Color,
    disable_border: bool,
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TerminalViewHandle {
    id: String,
}

impl TerminalViewHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> TerminalViewResponse {
        ui.state().terminal_view().response(self.id())
    }

    pub fn set_buffer(&self, ui: &mut Ui<'_>, buffer: TerminalBuffer) {
        ui.state().terminal_view().set_buffer(self.id(), buffer);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().terminal_view().set_enabled(self.id(), enabled);
    }
}

impl<'ui, 'a> TerminalViewBuilder<'ui, 'a> {
    pub(crate) fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 280.0,
            height: 44.0,
            buffer: TerminalBuffer::default(),
            font_size: 16.0,
            text_color: Color::from_rgb8(236, 241, 247),
            bg_color: Color::from_rgb8(21, 24, 28),
            border_color: Color::from_rgb8(48, 52, 58),
            disable_border: false,
            enabled: true,
        }
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = f64::from(px);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = f64::from(px);
        self
    }

    pub fn buffer(mut self, buffer: TerminalBuffer) -> Self {
        self.buffer = buffer;
        self
    }

    pub fn font_size(mut self, px: u32) -> Self {
        self.font_size = px as f32;
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn disable_border(mut self, value: bool) -> Self {
        self.disable_border = value;
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    pub fn show(self) -> TerminalViewHandle {
        let id: String = self.id;
        self.ui.show_terminal_view(ShowTerminalViewArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            buffer: self.buffer,
            font_size: self.font_size,
            text_color: self.text_color,
            bg_color: self.bg_color,
            border_color: self.border_color,
            disable_border: self.disable_border,
            enabled: self.enabled,
        });

        TerminalViewHandle::new(id)
    }
}
