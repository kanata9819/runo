
use std::cell::RefCell;
use std::rc::Rc;

use vello::Scene;

use crate::Color;
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
use crate::retained::RetainedState;

use super::Ui;

#[test]
fn ui_layout_helpers_and_fill_rect_are_callable() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    ui.fill_rect(0.0, 0.0, 20.0, 10.0, Color::from_rgb8(10, 20, 30));

    ui.vertical(|ui| {
        ui.widgets().label().id("v1").text("v1").show();
        ui.widgets().label().id("v2").text("v2").show();
    });
    ui.horizontal(|ui| {
        ui.widgets().label().id("h1").text("h1").show();
        ui.widgets().label().id("h2").text("h2").show();
    });
}

#[test]
fn ui_use_effect_delegates_to_effect_store() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);
    let calls = Rc::new(RefCell::new(0usize));

    ui.use_effect("e", 1_u32, {
        let calls = Rc::clone(&calls);
        move || {
            *calls.borrow_mut() += 1;
            None
        }
    });
    // unchanged deps for same id should not invoke again.
    ui.use_effect("e", 1_u32, {
        let calls = Rc::clone(&calls);
        move || {
            *calls.borrow_mut() += 1;
            None
        }
    });
    assert_eq!(*calls.borrow(), 1);
}

#[test]
fn ui_use_state_and_setter_roundtrip() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let (value, set_value) = ui.use_state("counter", || 7_u32);
    assert_eq!(value, 7);
    assert!(set_value.set(&mut ui, 9_u32));
    let (value2, _) = ui.use_state("counter", || 0_u32);
    assert_eq!(value2, 9);
}
