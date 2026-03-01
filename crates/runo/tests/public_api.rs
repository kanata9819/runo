use runo::{
    ButtonResponse, Color, Overflow, RunOptions, RunoApplication, TextBoxResponse, UiEvent, colors,
};

struct DefaultOptionsApp;

impl RunoApplication for DefaultOptionsApp {
    type Event = ();
}

struct CustomOptionsApp;

impl RunoApplication for CustomOptionsApp {
    type Event = ();

    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "custom".to_string(),
            window_width: 1280,
            window_height: 720,
            window_resizable: true,
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
    fn accepts_and_matches(event: UiEvent) {
        match event {
            UiEvent::ButtonClicked { button: _ } => {}
            UiEvent::CheckboxChanged {
                checkbox: _,
                checked: _,
            } => {}
            UiEvent::RadioButtonChanged {
                radio_button: _,
                group: _,
                selected: _,
            } => {}
            UiEvent::SliderChanged {
                slider: _,
                value: _,
            } => {}
            UiEvent::TextBoxChanged {
                text_box: _,
                text: _,
            } => {}
            UiEvent::ComboBoxChanged {
                combo_box: _,
                selected_index: _,
                selected_text: _,
            } => {}
        }
    }

    let _fn_ref: fn(UiEvent) = accepts_and_matches;
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
