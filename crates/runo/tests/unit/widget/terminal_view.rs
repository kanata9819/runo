use vello::Scene;
use vello::peniko::Color;

use super::{TerminalBuffer, TerminalCell, TerminalViewResponse};
use crate::hooks::use_effect::EffectStore;
use crate::hooks::use_state::StateStore;
use crate::retained::RetainedState;
use crate::ui::Ui;

#[test]
fn terminal_cell_builder_sets_faint_flag() {
    let cell = TerminalCell::new('x').faint(true);
    assert_eq!(cell.ch, 'x');
    assert!(cell.faint);
}

#[test]
fn terminal_view_response_default_is_not_hovered() {
    let response = TerminalViewResponse::default();
    assert!(!response.hovered);
}

#[test]
fn terminal_view_builder_methods_and_show_work() {
    let mut scene = Scene::new();
    let mut effects = EffectStore::new();
    let mut states = StateStore::new();
    let mut retained = RetainedState::new();
    let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

    let buffer = TerminalBuffer {
        lines: vec![vec![
            TerminalCell::new('n'),
            TerminalCell::new('u').faint(true),
        ]],
        cursor_row: 0,
        cursor_col: 2,
    };

    let terminal_view = ui
        .widgets()
        .terminal_view()
        .id("tv")
        .width(320)
        .height(120)
        .buffer(buffer.clone())
        .font_size(16)
        .text_color(Color::from_rgb8(220, 220, 220))
        .bg_color(Color::from_rgb8(20, 20, 20))
        .border_color(Color::from_rgb8(80, 80, 80))
        .enabled(false)
        .show();

    assert!(!terminal_view.response(&mut ui).hovered);

    terminal_view.set_enabled(&mut ui, true);
    terminal_view.set_buffer(&mut ui, buffer);
    assert!(!terminal_view.response(&mut ui).hovered);
}
