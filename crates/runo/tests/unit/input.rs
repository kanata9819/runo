use super::*;

#[test]
fn mouse_press_sets_pressed_flags() {
    let mut input = InputState::default();

    input.on_mouse_input(ElementState::Pressed);
    let frame = input.snapshot();
    assert!(frame.mouse_down);
    assert!(frame.mouse_pressed);
    assert!(!frame.mouse_released);
}

#[test]
fn mouse_release_sets_released_flags() {
    let mut input = InputState::default();

    input.on_mouse_input(ElementState::Pressed);
    input.end_frame();
    input.on_mouse_input(ElementState::Released);
    let frame = input.snapshot();
    assert!(!frame.mouse_down);
    assert!(!frame.mouse_pressed);
    assert!(frame.mouse_released);
}

#[test]
fn end_frame_clears_transient_flags() {
    let mut input = InputState::default();

    input.push_text_input("abc");
    input.on_named_key_pressed(NamedKey::Enter);
    input.on_copy_pressed();
    input.on_paste_pressed();
    input.on_mouse_wheel(3.0, -2.0);
    input.end_frame();

    let frame = input.snapshot();
    assert_eq!(frame.text_input, "");
    assert!(!frame.enter_pressed);
    assert!(!frame.copy_pressed);
    assert!(!frame.paste_pressed);
    assert_eq!(frame.scroll_x, 0.0);
    assert_eq!(frame.scroll_y, 0.0);
}

#[test]
fn mouse_wheel_values_accumulate_until_end_frame() {
    let mut input = InputState::default();

    input.on_mouse_wheel(1.0, 2.0);
    input.on_mouse_wheel(-0.5, 0.5);
    let frame = input.snapshot();
    assert_eq!(frame.scroll_x, 0.5);
    assert_eq!(frame.scroll_y, 2.5);

    input.end_frame();
    let frame = input.snapshot();
    assert_eq!(frame.scroll_x, 0.0);
    assert_eq!(frame.scroll_y, 0.0);
}
