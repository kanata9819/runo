use super::*;

struct App;
impl RunoApplication for App {
    type Event = ();
}

#[test]
fn run_options_default_values() {
    let options = RunOptions::default();
    assert_eq!(options.window_title, "runo app");
    assert_eq!(options.window_width, 640);
    assert_eq!(options.window_height, 480);
}

#[test]
fn application_default_options_delegate_to_run_options_default() {
    let app = App;
    let options = app.options();
    assert_eq!(options.window_title, "runo app");
    assert_eq!(options.window_width, 640);
    assert_eq!(options.window_height, 480);
}

struct CustomApp;
impl RunoApplication for CustomApp {
    type Event = ();

    fn options(&self) -> RunOptions {
        RunOptions {
            window_title: "custom".to_string(),
            window_width: 111,
            window_height: 222,
            window_resizable: false,
        }
    }
}

#[test]
fn run_options_default_sets_resizable_true() {
    assert!(RunOptions::default().window_resizable);
}

#[test]
fn application_can_override_options() {
    let app = CustomApp;
    let options = app.options();
    assert_eq!(options.window_title, "custom");
    assert_eq!(options.window_width, 111);
    assert_eq!(options.window_height, 222);
    assert!(!options.window_resizable);
}

#[test]
fn build_runner_uses_application_options() {
    let runner = build_runner(CustomApp);
    let options = runner.window_options();
    assert_eq!(options.window_title, "custom");
    assert_eq!(options.window_width, 111);
    assert_eq!(options.window_height, 222);
    assert!(!options.window_resizable);
}

#[test]
fn run_symbol_points_to_runtime_run() {
    let run_fn: fn(App) = run::<App>;
    let runtime_fn: fn(App) = runtime::run::<App>;
    let _ = (run_fn, runtime_fn);
}
