    use vello::Scene;
    use vello::peniko::Color;

    use super::Overflow;
    use crate::hooks::use_effect::EffectStore;
    use crate::hooks::use_state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn overflow_allows_scroll_only_for_scroll_and_auto() {
        assert!(!Overflow::Visible.allows_scroll());
        assert!(!Overflow::Hidden.allows_scroll());
        assert!(Overflow::Scroll.allows_scroll());
        assert!(Overflow::Auto.allows_scroll());
    }

    #[test]
    fn overflow_clips_except_visible() {
        assert!(!Overflow::Visible.clips());
        assert!(Overflow::Hidden.clips());
        assert!(Overflow::Scroll.clips());
        assert!(Overflow::Auto.clips());
    }

    #[test]
    fn text_box_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let text_box = ui
            .widgets()
            .text_box()
            .id("tb")
            .width(300)
            .height(48)
            .text("hello")
            .placeholder("type")
            .font_size(16)
            .text_color(Color::from_rgb8(220, 220, 220))
            .bg_color(Color::from_rgb8(30, 30, 30))
            .border_color(Color::from_rgb8(80, 80, 80))
            .enabled(false)
            .overflow_x(Overflow::Scroll)
            .overflow_y(Overflow::Auto)
            .show();
        assert_eq!(text_box.text(&mut ui), "hello");

        text_box.set_enabled(&mut ui, true);
        text_box.set_text(&mut ui, "updated");
        assert_eq!(text_box.text(&mut ui), "updated");
    }
