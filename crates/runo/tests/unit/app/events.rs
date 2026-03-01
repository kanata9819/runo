
use super::*;
use winit::dpi::PhysicalPosition;

#[test]
fn scroll_delta_to_pixels_handles_line_and_pixel_modes() {
    let (dx, dy) = scroll_delta_to_pixels(MouseScrollDelta::LineDelta(1.5, -2.0));
    assert_eq!(dx, 30.0);
    assert_eq!(dy, -40.0);

    let (dx, dy) = scroll_delta_to_pixels(MouseScrollDelta::PixelDelta(PhysicalPosition::new(
        3.0, -4.0,
    )));
    assert_eq!(dx, 3.0);
    assert_eq!(dy, -4.0);
}

#[test]
fn clipboard_shortcut_detects_copy_and_paste_keys() {
    assert_eq!(
        clipboard_shortcut(&Key::Character("c".into())),
        Some(ClipboardShortcut::Copy)
    );
    assert_eq!(
        clipboard_shortcut(&Key::Character("C".into())),
        Some(ClipboardShortcut::Copy)
    );
    assert_eq!(
        clipboard_shortcut(&Key::Character("v".into())),
        Some(ClipboardShortcut::Paste)
    );
    assert_eq!(
        clipboard_shortcut(&Key::Character("V".into())),
        Some(ClipboardShortcut::Paste)
    );
    assert_eq!(clipboard_shortcut(&Key::Character("x".into())), None);
    assert_eq!(
        clipboard_shortcut(&Key::Named(winit::keyboard::NamedKey::Enter)),
        None
    );
}

#[test]
fn keyboard_actions_for_character_input_without_modifiers() {
    let actions = keyboard_actions(&Key::Character("a".into()), Some("a"), true, false, false);
    assert_eq!(actions.push_text.as_deref(), Some("a"));
    assert_eq!(actions.named_key, None);
    assert_eq!(actions.clipboard, None);
    assert!(actions.request_redraw);
}

#[test]
fn keyboard_actions_for_named_key_and_clipboard_shortcut() {
    let enter_actions = keyboard_actions(&Key::Named(NamedKey::Enter), None, true, false, false);
    assert_eq!(enter_actions.named_key, Some(NamedKey::Enter));
    assert_eq!(enter_actions.push_text, None);
    assert_eq!(enter_actions.clipboard, None);
    assert!(enter_actions.request_redraw);

    let copy_actions = keyboard_actions(&Key::Character("c".into()), Some("c"), true, true, false);
    assert_eq!(copy_actions.push_text, None);
    assert_eq!(copy_actions.clipboard, Some(ClipboardShortcut::Copy));
}

#[test]
fn keyboard_actions_ignore_text_when_ime_active_or_key_released() {
    let ime_actions = keyboard_actions(&Key::Character("x".into()), Some("x"), true, false, true);
    assert_eq!(ime_actions.push_text, None);
    assert!(ime_actions.request_redraw);

    let released_actions =
        keyboard_actions(&Key::Character("x".into()), Some("x"), false, false, false);
    assert_eq!(released_actions, KeyboardActions::default());
}
