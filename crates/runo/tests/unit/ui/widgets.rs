
use vello::Scene;
use vello::peniko::Color;

use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
use crate::retained::RetainedState;
use crate::ui::Ui;

#[test]
fn widgets_builders_can_show_all_widget_types() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let button = ui
        .widgets()
        .button()
        .id("btn")
        .size(140, 40)
        .text("ok")
        .show();
    assert!(!button.clicked(&mut ui));

    let checkbox = ui
        .widgets()
        .checkbox()
        .id("check")
        .checked(true)
        .text("check")
        .show();
    assert!(checkbox.checked(&mut ui));

    let text_box = ui
        .widgets()
        .text_box()
        .id("tb")
        .text("hello")
        .placeholder("type")
        .show();
    assert_eq!(text_box.text(&mut ui), "hello");

    let combo = ui
        .widgets()
        .combo_box()
        .id("combo")
        .items(["a", "b", "c"])
        .selected_index(1)
        .show();
    assert_eq!(combo.selected_text(&mut ui), "b");

    let radio = ui
        .widgets()
        .radio_button()
        .id("r1")
        .group("g")
        .selected(true)
        .text("radio")
        .show();
    assert!(radio.selected(&mut ui));

    let slider = ui
        .widgets()
        .slider()
        .id("s")
        .range(0.0, 10.0)
        .value(2.0)
        .step(1.0)
        .text("slider")
        .show();
    assert_eq!(slider.value(&mut ui), 2.0);

    ui.widgets()
        .label()
        .id("lbl")
        .text("label")
        .text_color(Color::from_rgb8(220, 220, 220))
        .show();

    let nested_result = ui
        .widgets()
        .div()
        .id("div")
        .horizontal()
        .padding(8)
        .gap(6)
        .background(Color::from_rgb8(30, 30, 30))
        .border(Color::from_rgb8(80, 80, 80), 1)
        .radius(4)
        .show(|ui| {
            ui.widgets().label().id("inside").text("inside").show();
            42
        });
    assert_eq!(nested_result, 42);
}
