
use vello::Scene;
use vello::peniko::Color;

use super::CheckboxResponse;
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
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
