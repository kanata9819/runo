use super::*;
use crate::RunOptions;

#[derive(Default)]
struct DummyApp {
    mount_calls: usize,
    event_calls: usize,
}

impl RunoApplication for DummyApp {
    type Event = ();

    fn build(&mut self, _ui: &mut Ui<'_>) -> crate::EventBindings<Self::Event> {
        self.mount_calls += 1;
        crate::EventBindings::new()
    }

    fn on_event(&mut self, _ui: &mut Ui<'_>, _event: Self::Event) -> bool {
        self.event_calls += 1;
        false
    }
}

#[test]
fn render_returns_false_when_surface_is_missing() {
    let mut runner = AppRunner::new(DummyApp::default(), RunOptions::default());
    assert!(!runner.render());
}

#[test]
fn build_scene_and_run_ui_frame_execute_without_gpu() {
    let mut runner = AppRunner::new(DummyApp::default(), RunOptions::default());
    runner.build_scene(320, 180);
    runner.run_ui_frame();
    assert_eq!(runner.user_app.mount_calls, 1);
    assert_eq!(runner.user_app.event_calls, 0);
}

#[test]
fn state_change_requests_remount_on_next_frame() {
    let mut runner = AppRunner::new(DummyApp::default(), RunOptions::default());
    runner.build_scene(320, 180);
    runner.run_ui_frame();
    assert_eq!(runner.user_app.mount_calls, 1);

    assert!(runner.states.set_state("counter", 1_u32));
    assert!(!runner.mount_required);

    runner.run_ui_frame();
    assert!(runner.mount_required);
    assert_eq!(runner.user_app.mount_calls, 1);

    runner.run_ui_frame();
    assert_eq!(runner.user_app.mount_calls, 2);
}
