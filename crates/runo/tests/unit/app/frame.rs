    use super::*;
    use crate::RunOptions;

    #[derive(Default)]
    struct DummyApp {
        build_calls: usize,
        event_calls: usize,
    }

    impl RunoApplication for DummyApp {
        type Event = ();

        fn event_bindings(&self) -> crate::EventBindings<Self::Event> {
            crate::EventBindings::new()
        }

        fn build(&mut self, _ui: &mut Ui<'_>) {
            self.build_calls += 1;
        }

        fn on_event(&mut self, _ui: &mut Ui<'_>, _event: Self::Event) {
            self.event_calls += 1;
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
        assert_eq!(runner.user_app.build_calls, 1);
        assert_eq!(runner.user_app.event_calls, 0);
    }
