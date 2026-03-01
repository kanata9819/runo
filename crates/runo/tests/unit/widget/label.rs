
use vello::Scene;
use vello::peniko::Color;

use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
use crate::retained::RetainedState;
use crate::ui::Ui;

#[test]
fn label_builder_setters_and_show_work_without_font() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let label = ui
        .widgets()
        .label()
        .id("lbl")
        .width(180)
        .height(32)
        .font_size(22)
        .text("hello")
        .text_color(Color::from_rgb8(210, 220, 230))
        .enabled(false)
        .show();

    label.set_enabled(&mut ui, true);
}
