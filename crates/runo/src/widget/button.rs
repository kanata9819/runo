use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowButtonArgs;

#[derive(Clone, Copy, Debug, Default)]
pub struct ButtonResponse {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

pub struct ButtonBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

impl<'ui, 'a> ButtonBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 180.0,
            height: 56.0,
            text: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(245, 248, 252),
            enabled: true,
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

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width as f64;
        self.height = height as f64;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
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

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }

    pub fn show(self) -> ButtonResponse {
        self.ui.show_button(ShowButtonArgs {
            id: self.id,
            width: self.width,
            height: self.height,
            text: self.text,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::peniko::Color;

    use super::ButtonResponse;
    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn button_response_default_is_all_false() {
        let response = ButtonResponse::default();
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.clicked);
    }

    #[test]
    fn button_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let response = ui
            .widgets()
            .button()
            .id("btn")
            .width(120)
            .height(36)
            .size(140, 40)
            .text("press")
            .font_size(20)
            .text_color(Color::from_rgb8(220, 220, 220))
            .enabled(false)
            .show();
        assert!(!response.clicked);

        ui.state().button().set_enabled("btn", true);
        ui.state().button().set_text("btn", "ok");
        assert!(!ui.state().button().clicked("btn"));
    }
}
