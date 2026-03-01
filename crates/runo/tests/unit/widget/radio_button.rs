
use vello::Scene;
use vello::peniko::Color;

use super::RadioButtonResponse;
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
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

    let radio = ui
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
    assert!(radio.selected(&mut ui));

    radio.set_enabled(&mut ui, true);
    radio.set_selected(&mut ui, false);
    assert!(!radio.selected(&mut ui));
}
