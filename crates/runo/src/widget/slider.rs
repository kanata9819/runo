use vello::peniko::Color;

use crate::Ui;
use crate::ui::ShowSliderArgs;
use crate::ui::UiEvents;

#[derive(Clone, Debug, Default)]
pub struct SliderResponse {
    pub value: f64,
    pub hovered: bool,
    pub pressed: bool,
    pub changed: bool,
}

pub struct SliderBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    min: f64,
    max: f64,
    value: Option<f64>,
    step: Option<f64>,
    text: Option<String>,
    font_size: f32,
    text_color: Color,
    enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SliderHandle {
    id: String,
}

impl SliderHandle {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub fn response(&self, ui: &mut Ui<'_>) -> SliderResponse {
        ui.state().slider().response(self.id())
    }

    pub fn value(&self, ui: &mut Ui<'_>) -> f64 {
        self.response(ui).value
    }

    pub fn set_value(&self, ui: &mut Ui<'_>, value: f64) {
        ui.state().slider().set_value(self.id(), value);
    }

    pub fn set_enabled(&self, ui: &mut Ui<'_>, enabled: bool) {
        ui.state().slider().set_enabled(self.id(), enabled);
    }

    pub fn on_change(&self, events: &mut UiEvents<'_, '_>, f: impl FnOnce(f64)) {
        events.on_slider_changed(self, f);
    }

    pub fn take_change(&self, events: &mut UiEvents<'_, '_>) -> Option<f64> {
        events.slider_changed(self)
    }

    pub fn on_change_with_ui(
        &self,
        events: &mut UiEvents<'_, '_>,
        f: impl FnOnce(&mut Ui<'_>, f64),
    ) {
        events.on_slider_changed_with_ui(self, f);
    }
}

impl<'ui, 'a> SliderBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 320.0,
            height: 40.0,
            min: 0.0,
            max: 100.0,
            value: None,
            step: None,
            text: None,
            font_size: 16.0,
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

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        // Initial value at first creation.
        self.value = Some(value);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
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

    pub fn show(self) -> SliderHandle {
        let id = self.id;
        self.ui.show_slider(ShowSliderArgs {
            id: id.clone(),
            width: self.width,
            height: self.height,
            min: self.min,
            max: self.max,
            value: self.value,
            step: self.step,
            text: self.text,
            font_size: self.font_size,
            text_color: self.text_color,
            enabled: self.enabled,
        });
        SliderHandle::new(id)
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::peniko::Color;

    use super::SliderResponse;
    use crate::hooks::use_effect::EffectStore;
    use crate::hooks::use_state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn slider_response_default_is_zero_and_idle() {
        let response = SliderResponse::default();
        assert_eq!(response.value, 0.0);
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
    }

    #[test]
    fn slider_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let slider = ui
            .widgets()
            .slider()
            .id("sl")
            .width(220)
            .height(30)
            .range(-1.0, 1.0)
            .value(0.5)
            .step(0.25)
            .text("volume")
            .font_size(13)
            .text_color(Color::from_rgb8(220, 220, 220))
            .enabled(false)
            .show();
        assert!((slider.value(&mut ui) - 0.5).abs() < f64::EPSILON);

        slider.set_enabled(&mut ui, true);
        slider.set_value(&mut ui, -0.5);
        assert!((slider.value(&mut ui) + 0.5).abs() < f64::EPSILON);
    }
}
