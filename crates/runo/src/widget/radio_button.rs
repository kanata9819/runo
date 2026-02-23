use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowRadioButtonArgs;

#[derive(Clone, Debug, Default)]
pub struct RadioButtonResponse {
    pub selected: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
}

pub struct RadioButtonBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    group: String,
    width: f64,
    height: f64,
    text: Option<String>,
    selected: Option<bool>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

impl<'ui, 'a> RadioButtonBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            group: "default".to_string(),
            width: 260.0,
            height: 36.0,
            text: None,
            selected: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            enabled: true,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = px as f64;
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

    pub fn selected(mut self, selected: bool) -> Self {
        // Initial selected state at first creation.
        self.selected = Some(selected);
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

    pub fn show(self) -> RadioButtonResponse {
        self.ui.show_radio_button(ShowRadioButtonArgs {
            id: self.id,
            group: self.group,
            width: self.width,
            height: self.height,
            text: self.text,
            selected: self.selected,
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

    use super::RadioButtonResponse;
    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn radio_button_response_default_is_unselected_and_idle() {
        let response = RadioButtonResponse::default();
        assert!(!response.selected);
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
    }

    #[test]
    fn radio_button_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let response = ui
            .widgets()
            .radio_button()
            .id("r1")
            .group("g")
            .width(240)
            .height(32)
            .text("one")
            .selected(true)
            .font_size(17)
            .text_color(Color::from_rgb8(240, 240, 240))
            .enabled(false)
            .show();
        assert!(response.selected);

        ui.state().radio_button().set_enabled("r1", true);
        ui.state().radio_button().set_selected("r1", false);
        assert!(!ui.state().radio_button().selected("r1"));
    }
}
