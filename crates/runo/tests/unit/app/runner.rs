use super::*;
use crate::app::RunOptions;

struct DummyApp;
impl RunoApplication for DummyApp {
    type Event = ();

    fn event_bindings(&self) -> crate::EventBindings<Self::Event> {
        crate::EventBindings::new()
    }
}

#[test]
fn sanitize_window_size_never_returns_zero() {
    assert_eq!(sanitize_window_size(0, 0), (1, 1));
    assert_eq!(sanitize_window_size(640, 0), (640, 1));
    assert_eq!(sanitize_window_size(0, 480), (1, 480));
    assert_eq!(sanitize_window_size(800, 600), (800, 600));
}

#[test]
fn new_clamps_window_dimensions_to_minimum_one() {
    let runner = AppRunner::new(
        DummyApp,
        RunOptions {
            window_title: "t".to_string(),
            window_width: 0,
            window_height: 0,
            window_resizable: true,
        },
    );
    assert_eq!(runner.window_options.window_width, 1);
    assert_eq!(runner.window_options.window_height, 1);
}

#[test]
fn resize_ignores_zero_sizes_without_surface() {
    let mut runner = AppRunner::new(DummyApp, RunOptions::default());
    runner.resize(0, 100);
    runner.resize(100, 0);
    runner.resize(0, 0);
    runner.resize(100, 100);
    // no panic and no surface mutation path is enough for this unit test
    assert!(runner.surface.is_none());
}

#[test]
fn window_attributes_follow_run_options() {
    let options = RunOptions {
        window_title: "My Window".to_string(),
        window_width: 123,
        window_height: 456,
        window_resizable: false,
    };
    let attrs = window_attributes_from_options(&options);
    assert_eq!(attrs.title, "My Window");
    assert!(!attrs.resizable);
    assert!(attrs.inner_size.is_some());
}
