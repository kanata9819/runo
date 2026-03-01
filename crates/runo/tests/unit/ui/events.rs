
use vello::Scene;

use super::{ActionBindings, EventBindings};
use crate::event::UiEvent;
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
use crate::retained::RetainedState;
use crate::ui::Ui;
use crate::widget::button::ButtonHandle;
use crate::widget::checkbox::CheckboxHandle;
use crate::widget::combo_box::ComboBoxHandle;
use crate::widget::radio_button::RadioButtonHandle;
use crate::widget::slider::SliderHandle;
use crate::widget::text_box::TextBoxHandle;

#[test]
fn ui_events_empty_queue_returns_none_and_empty_vec() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    assert!(ui.events().next_event().is_none());
    assert!(ui.events().drain_events().is_empty());
}

#[test]
fn ui_events_can_drain_mapped_actions() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let button = ButtonHandle::new("btn".to_string());
    ui.retained.push_event(UiEvent::ButtonClicked {
        button: button.clone(),
    });

    let mut bindings = ActionBindings::new();
    bindings.bind_button(button, "do".to_string());

    let actions = ui.events().drain_actions(&bindings);
    assert_eq!(actions, vec!["do".to_string()]);
}

#[test]
fn ui_events_can_drain_bound_events_with_payloads() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let button = ButtonHandle::new("btn".to_string());
    let text_box = TextBoxHandle::new("tb".to_string());
    let checkbox = CheckboxHandle::new("cb".to_string());
    ui.retained.push_event(UiEvent::ButtonClicked {
        button: button.clone(),
    });
    ui.retained.push_event(UiEvent::TextBoxChanged {
        text_box: text_box.clone(),
        text: "hello".to_string(),
    });
    ui.retained.push_event(UiEvent::CheckboxChanged {
        checkbox: checkbox.clone(),
        checked: true,
    });

    let mut bindings = EventBindings::new();
    bindings.bind_button(button, "clicked".to_string());
    bindings.bind_text_box(text_box, |text| format!("text={text}"));
    bindings.bind_checkbox(checkbox, |checked| format!("checked={checked}"));

    let events = ui.events().drain_bound_events(&bindings);
    assert_eq!(
        events,
        vec![
            "clicked".to_string(),
            "text=hello".to_string(),
            "checked=true".to_string(),
        ]
    );
}

#[test]
fn event_bindings_builder_chains_and_builds() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let button = ButtonHandle::new("btn".to_string());
    let text_box = TextBoxHandle::new("tb".to_string());
    ui.retained.push_event(UiEvent::ButtonClicked {
        button: button.clone(),
    });
    ui.retained.push_event(UiEvent::TextBoxChanged {
        text_box: text_box.clone(),
        text: "hello".to_string(),
    });

    let bindings = EventBindings::builder()
        .button(button, "clicked".to_string())
        .text_box(text_box, |text| format!("text={text}"))
        .build();

    let events = ui.events().drain_bound_events(&bindings);
    assert_eq!(
        events,
        vec!["clicked".to_string(), "text=hello".to_string()]
    );
}

#[test]
fn event_bindings_builder_extend_merges_bindings() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let button = ButtonHandle::new("btn".to_string());
    let checkbox = CheckboxHandle::new("cb".to_string());
    ui.retained.push_event(UiEvent::ButtonClicked {
        button: button.clone(),
    });
    ui.retained.push_event(UiEvent::CheckboxChanged {
        checkbox: checkbox.clone(),
        checked: true,
    });

    let extra = EventBindings::builder()
        .checkbox(checkbox, |checked| format!("checked={checked}"))
        .build();
    let bindings = EventBindings::builder()
        .button(button, "clicked".to_string())
        .extend(extra)
        .build();

    let events = ui.events().drain_bound_events(&bindings);
    assert_eq!(
        events,
        vec!["clicked".to_string(), "checked=true".to_string()]
    );
}

#[test]
fn ui_events_callback_helpers_consume_matching_events() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let button = ButtonHandle::new("btn".to_string());
    let text_box = TextBoxHandle::new("tb".to_string());
    let checkbox = CheckboxHandle::new("cb".to_string());
    ui.retained.push_event(UiEvent::ButtonClicked {
        button: button.clone(),
    });
    ui.retained.push_event(UiEvent::TextBoxChanged {
        text_box: text_box.clone(),
        text: "hello".to_string(),
    });
    ui.retained.push_event(UiEvent::CheckboxChanged {
        checkbox: checkbox.clone(),
        checked: true,
    });

    let mut clicked = false;
    let mut text = String::new();
    let mut checked = false;
    {
        let mut events = ui.events();
        events.on_button_clicked(&button, || clicked = true);
        events.on_text_box_changed(&text_box, |value| text = value);
        events.on_checkbox_changed(&checkbox, |value| checked = value);
    }

    assert!(clicked);
    assert_eq!(text, "hello");
    assert!(checked);
    assert!(ui.events().drain_events().is_empty());
}

#[test]
fn ui_events_callback_helpers_consume_slider_radio_and_combo_events() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let slider = SliderHandle::new("sl".to_string());
    let radio_button = RadioButtonHandle::new("rb".to_string());
    let combo_box = ComboBoxHandle::new("cb".to_string());
    ui.retained.push_event(UiEvent::SliderChanged {
        slider: slider.clone(),
        value: 0.75,
    });
    ui.retained.push_event(UiEvent::RadioButtonChanged {
        radio_button: radio_button.clone(),
        group: "g".to_string(),
        selected: true,
    });
    ui.retained.push_event(UiEvent::ComboBoxChanged {
        combo_box: combo_box.clone(),
        selected_index: 1,
        selected_text: "b".to_string(),
    });

    let mut slider_value = 0.0;
    let mut selected = false;
    let mut combo = (0usize, String::new());
    {
        let mut events = ui.events();
        events.on_slider_changed(&slider, |value| slider_value = value);
        events.on_radio_button_changed(&radio_button, |value| selected = value);
        events.on_combo_box_changed(&combo_box, |index, text| combo = (index, text));
    }

    assert!((slider_value - 0.75).abs() < f64::EPSILON);
    assert!(selected);
    assert_eq!(combo, (1, "b".to_string()));
    assert!(ui.events().drain_events().is_empty());
}
