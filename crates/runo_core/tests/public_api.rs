use runo_core::{
    ButtonResponse, Color, Overflow, RunOptions, RunoApplication, TextBoxResponse, UiEvent, colors,
};

struct DefaultOptionsApp;

impl RunoApplication for DefaultOptionsApp {}

struct CustomOptionsApp;

impl RunoApplication for CustomOptionsApp {
    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "custom".to_string(),
            window_width: 1280,
            window_height: 720,
        }
    }
}

#[test]
fn run_options_default_values_are_stable() {
    let options = RunOptions::default();
    assert_eq!(options.window_title, "runo app");
    assert_eq!(options.window_width, 640);
    assert_eq!(options.window_height, 480);
}

#[test]
fn runo_application_uses_default_options_without_override() {
    let app = DefaultOptionsApp;
    let options = app.options();
    assert_eq!(options.window_title, "runo app");
    assert_eq!(options.window_width, 640);
    assert_eq!(options.window_height, 480);
}

#[test]
fn runo_application_can_override_options() {
    let app = CustomOptionsApp;
    let options = app.options();
    assert_eq!(options.window_title, "custom");
    assert_eq!(options.window_width, 1280);
    assert_eq!(options.window_height, 720);
}

#[test]
fn ui_event_payloads_are_accessible_through_pattern_matching() {
    let events = vec![
        UiEvent::ButtonClicked {
            id: "btn".to_string(),
        },
        UiEvent::CheckboxChanged {
            id: "check".to_string(),
            checked: true,
        },
        UiEvent::RadioButtonChanged {
            id: "radio.a".to_string(),
            group: "group".to_string(),
            selected: true,
        },
        UiEvent::SliderChanged {
            id: "slider".to_string(),
            value: 0.75,
        },
        UiEvent::TextBoxChanged {
            id: "text".to_string(),
            text: "hello".to_string(),
        },
        UiEvent::ComboBoxChanged {
            id: "combo".to_string(),
            selected_index: 2,
            selected_text: "Manager".to_string(),
        },
    ];

    match &events[0] {
        UiEvent::ButtonClicked { id } => assert_eq!(id, "btn"),
        _ => panic!("unexpected event variant"),
    }
    match &events[1] {
        UiEvent::CheckboxChanged { id, checked } => {
            assert_eq!(id, "check");
            assert!(*checked);
        }
        _ => panic!("unexpected event variant"),
    }
    match &events[2] {
        UiEvent::RadioButtonChanged {
            id,
            group,
            selected,
        } => {
            assert_eq!(id, "radio.a");
            assert_eq!(group, "group");
            assert!(*selected);
        }
        _ => panic!("unexpected event variant"),
    }
    match &events[3] {
        UiEvent::SliderChanged { id, value } => {
            assert_eq!(id, "slider");
            assert!((*value - 0.75).abs() < f64::EPSILON);
        }
        _ => panic!("unexpected event variant"),
    }
    match &events[4] {
        UiEvent::TextBoxChanged { id, text } => {
            assert_eq!(id, "text");
            assert_eq!(text, "hello");
        }
        _ => panic!("unexpected event variant"),
    }
    match &events[5] {
        UiEvent::ComboBoxChanged {
            id,
            selected_index,
            selected_text,
        } => {
            assert_eq!(id, "combo");
            assert_eq!(*selected_index, 2);
            assert_eq!(selected_text, "Manager");
        }
        _ => panic!("unexpected event variant"),
    }
}

#[test]
fn widget_responses_have_expected_defaults() {
    let button = ButtonResponse::default();
    assert!(!button.hovered);
    assert!(!button.pressed);
    assert!(!button.clicked);

    let text_box = TextBoxResponse::default();
    assert_eq!(text_box.text, "");
    assert!(!text_box.hovered);
    assert!(!text_box.focused);
    assert!(!text_box.changed);
}

#[test]
fn overflow_is_copyable_and_comparable() {
    let overflow = Overflow::Auto;
    let copied = overflow;
    assert_eq!(copied, Overflow::Auto);
    assert_ne!(Overflow::Visible, Overflow::Hidden);
}

#[test]
fn rgb_matches_direct_color_constructor() {
    let from_helper = colors::rgb((12, 34, 56));
    let from_constructor = Color::from_rgb8(12, 34, 56);
    assert_eq!(format!("{from_helper:?}"), format!("{from_constructor:?}"));
}
