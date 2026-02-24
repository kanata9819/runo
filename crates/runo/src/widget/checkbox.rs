use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowCheckboxArgs;

#[derive(Clone, Debug, Default)]
pub struct CheckboxResponse {
    pub checked: bool,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
}

pub struct CheckboxBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    checked: Option<bool>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CheckboxHandle {
    id: String,
}

impl CheckboxHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> CheckboxResponse {
        ui.state().checkbox().response(self.id())
    }

    pub fn checked(&self, ui: &mut Ui<'_>) -> bool {
        self.response(ui).checked
    }

    pub fn set_checked(&self, ui: &mut Ui<'_>, checked: bool) {
        ui.state().checkbox().set_checked(self.id(), checked);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().checkbox().set_enabled(self.id(), enabled);
    }
}

impl<'ui, 'a> CheckboxBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 260.0,
            height: 36.0,
            text: None,
            checked: None,
            font_size: 18.0,
            text_color: Color::from_rgb8(236, 241, 247),
            enabled: true,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
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

    pub fn checked(mut self, checked: bool) -> Self {
        // Initial checked state at first creation.
        self.checked = Some(checked);
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

    pub fn show(self) -> CheckboxHandle {
        let id = self.id;
        self.ui.show_checkbox(ShowCheckboxArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            text: self.text,
            checked: self.checked,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        });
        CheckboxHandle::new(id)
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::peniko::Color;

    use super::CheckboxResponse;
    use crate::hooks::effect::EffectStore;
    use crate::hooks::state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn checkbox_response_default_is_unchecked_and_idle() {
        let response = CheckboxResponse::default();
        assert!(!response.checked);
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
    }

    #[test]
    fn checkbox_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let checkbox = ui
            .widgets()
            .checkbox()
            .id("cb")
            .width(180)
            .height(28)
            .text("check")
            .checked(true)
            .font_size(14)
            .text_color(Color::from_rgb8(230, 230, 230))
            .enabled(false)
            .show();
        assert!(checkbox.checked(&mut ui));

        checkbox.set_enabled(&mut ui, true);
        checkbox.set_checked(&mut ui, false);
        assert!(!checkbox.checked(&mut ui));
    }
}
