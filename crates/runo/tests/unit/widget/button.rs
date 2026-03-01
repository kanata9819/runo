use vello::Scene;
use vello::peniko::Color;

use super::ButtonResponse;
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
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

    let button = ui
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
    assert!(!button.clicked(&mut ui));

    button.set_enabled(&mut ui, true);
    button.set_text(&mut ui, "ok");
    assert!(!button.clicked(&mut ui));
}
