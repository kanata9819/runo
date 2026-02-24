use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowTextBoxArgs;
use crate::ui::UiEvents;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl Overflow {
    pub(crate) fn allows_scroll(self) -> bool {
        matches!(self, Self::Scroll | Self::Auto)
    }

    pub(crate) fn clips(self) -> bool {
        !matches!(self, Self::Visible)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextBoxResponse {
    pub text: String,
    pub hovered: bool,
    pub focused: bool,
    pub changed: bool,
}

pub struct TextBoxBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    placeholder: Option<String>,
    font_size: f32,
    text_color: Color,
    bg_color: Color,
    border_color: Color,
    enabled: bool,
    overflow_x: Overflow,
    overflow_y: Overflow,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextBoxHandle {
    id: String,
}

impl TextBoxHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> TextBoxResponse {
        ui.state().text_box().response(self.id())
    }

    pub fn text(&self, ui: &mut Ui<'_>) -> String {
        self.response(ui).text
    }

    pub fn set_text(&self, ui: &mut Ui<'_>, text: impl Into<String>) {
        ui.state().text_box().set_text(self.id(), text);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().text_box().set_enabled(self.id(), enabled);
    }

    pub fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(String)) {
        events.on_text_box_changed(self, f);
    }

    pub fn take_change(&self, events: &mut UiEvents<'_, '_>) -> Option<String> {
        events.text_box_changed(self)
    }

    pub fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut Ui<'_>, String),
    ) {
        events.on_text_box_changed_with_ui(self, f);
    }
}

impl<'ui, 'a> TextBoxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 280.0,
            height: 44.0,
            text: None,
            placeholder: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            bg_color: Color::from_rgb8(33, 38, 46),
            border_color: Color::from_rgb8(78, 89, 104),
            enabled: true,
            overflow_x: Overflow::Auto,
            overflow_y: Overflow::Hidden,
        }
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = px as f64;
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = px as f64;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
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

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    pub fn overflow_x(mut self, overflow: Overflow) -> Self {
        self.overflow_x = overflow;
        self
    }

    pub fn overflow_y(mut self, overflow: Overflow) -> Self {
        self.overflow_y = overflow;
        self
    }

    pub fn show(self) -> TextBoxHandle {
        let id = self.id;
        self.ui.show_text_box(ShowTextBoxArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            text: self.text,
            placeholder: self.placeholder,
            font_size: self.font_size,
            text_color: self.text_color,
            bg_color: self.bg_color,
            border_color: self.border_color,
            enabled: self.enabled,
            overflow_x: self.overflow_x,
            overflow_y: self.overflow_y,
        });
        TextBoxHandle::new(id)
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::peniko::Color;

    use super::Overflow;
    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn overflow_allows_scroll_only_for_scroll_and_auto() {
        assert!(!Overflow::Visible.allows_scroll());
        assert!(!Overflow::Hidden.allows_scroll());
        assert!(Overflow::Scroll.allows_scroll());
        assert!(Overflow::Auto.allows_scroll());
    }

    #[test]
    fn overflow_clips_except_visible() {
        assert!(!Overflow::Visible.clips());
        assert!(Overflow::Hidden.clips());
        assert!(Overflow::Scroll.clips());
        assert!(Overflow::Auto.clips());
    }

    #[test]
    fn text_box_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let text_box = ui
            .widgets()
            .text_box()
            .id("tb")
            .width(300)
            .height(48)
            .text("hello")
            .placeholder("type")
            .font_size(16)
            .text_color(Color::from_rgb8(220, 220, 220))
            .bg_color(Color::from_rgb8(30, 30, 30))
            .border_color(Color::from_rgb8(80, 80, 80))
            .enabled(false)
            .overflow_x(Overflow::Scroll)
            .overflow_y(Overflow::Auto)
            .show();
        assert_eq!(text_box.text(&mut ui), "hello");

        text_box.set_enabled(&mut ui, true);
        text_box.set_text(&mut ui, "updated");
        assert_eq!(text_box.text(&mut ui), "updated");
    }
}
