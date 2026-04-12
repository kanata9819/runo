use super::*;
use crate::event::UiEvent;
use crate::input::InputFrame;
use crate::retained::{UpsertSliderArgs, UpsertTextBoxArgs};
use crate::widget::text_box::Overflow;
use vello::kurbo::Rect;
use vello::peniko::Color;

#[test]
fn begin_frame_input_runs_with_default_input_and_no_widgets() {
    let mut state = RetainedState::new();
    let input = InputFrame {
        cursor_pos: (0.0, 0.0),
        mouse_down: false,
        mouse_pressed: false,
        mouse_released: false,
        text_input: String::new(),
        backspace_pressed: false,
        delete_pressed: false,
        enter_pressed: false,
        arrow_left_pressed: false,
        arrow_right_pressed: false,
        arrow_up_pressed: false,
        arrow_down_pressed: false,
        copy_pressed: false,
        paste_pressed: false,
        scroll_x: 0.0,
        scroll_y: 0.0,
    };

    state.begin_frame_input(input, None);
    assert!(state.drain_events().is_empty());
}

fn input_frame_at(x: f64, y: f64) -> InputFrame {
    InputFrame {
        cursor_pos: (x, y),
        mouse_down: false,
        mouse_pressed: false,
        mouse_released: false,
        text_input: String::new(),
        backspace_pressed: false,
        delete_pressed: false,
        enter_pressed: false,
        arrow_left_pressed: false,
        arrow_right_pressed: false,
        arrow_up_pressed: false,
        arrow_down_pressed: false,
        copy_pressed: false,
        paste_pressed: false,
        scroll_x: 0.0,
        scroll_y: 0.0,
    }
}

#[test]
fn begin_frame_input_emits_button_clicked_event_on_press_over_button() {
    let mut state = RetainedState::new();
    state.upsert_button(
        "btn".to_string(),
        Rect::new(0.0, 0.0, 100.0, 40.0),
        Some("ok".to_string()),
        16.0,
        Color::from_rgb8(240, 240, 240),
        true,
    );

    let mut input = input_frame_at(10.0, 10.0);
    input.mouse_pressed = true;
    input.mouse_down = true;
    state.begin_frame_input(input, None);

    let events = state.drain_events();
    assert!(
        events.iter().any(
            |event| matches!(event, UiEvent::ButtonClicked { button } if button.id() == "btn")
        )
    );
}

#[test]
fn begin_frame_input_applies_text_to_newly_focused_text_box() {
    let mut state = RetainedState::new();
    state.upsert_text_box(UpsertTextBoxArgs {
        id: "tb".to_string(),
        rect: Rect::new(0.0, 0.0, 180.0, 44.0),
        text: Some("ab".to_string()),
        placeholder: Some("p".to_string()),
        font_size: 16.0,
        text_color: Color::from_rgb8(240, 240, 240),
        bg_color: Color::from_rgb8(30, 30, 30),
        border_color: Color::from_rgb8(80, 80, 80),
        disable_border: false,
        enabled: true,
        read_only: false,
        overflow_x: Overflow::Auto,
        overflow_y: Overflow::Auto,
    });

    let mut input = input_frame_at(10.0, 10.0);
    input.mouse_pressed = true;
    input.text_input = "Z".to_string();
    state.begin_frame_input(input, None);

    let response = state.text_box_response("tb");
    assert_eq!(response.text, "abZ");

    let events = state.drain_events();
    assert!(events.iter().any(|event| {
        matches!(event, UiEvent::TextBoxChanged { text_box, text } if text_box.id() == "tb" && text == "abZ")
    }));
}

#[test]
fn begin_frame_input_updates_slider_value_while_dragging() {
    let mut state = RetainedState::new();
    state.upsert_slider(UpsertSliderArgs {
        id: "slider".to_string(),
        rect: Rect::new(0.0, 0.0, 212.0, 48.0),
        min: 0.0,
        max: 10.0,
        value: Some(0.0),
        step: Some(1.0),
        text: Some("s".to_string()),
        font_size: 14.0,
        text_color: Color::from_rgb8(240, 240, 240),
        enabled: true,
    });

    let mut press = input_frame_at(20.0, 24.0);
    press.mouse_pressed = true;
    press.mouse_down = true;
    state.begin_frame_input(press, None);

    let mut drag = input_frame_at(190.0, 24.0);
    drag.mouse_down = true;
    state.begin_frame_input(drag, None);

    let response = state.slider_response("slider");
    assert!(response.value >= 9.0);

    let events = state.drain_events();
    assert!(events.iter().any(
        |event| matches!(event, UiEvent::SliderChanged { slider, .. } if slider.id() == "slider")
    ));
}


