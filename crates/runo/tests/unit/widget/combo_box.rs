    use vello::Scene;
    use vello::peniko::Color;

    use super::ComboBoxResponse;
    use crate::hooks::use_effect::EffectStore;
    use crate::hooks::use_state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn combo_box_response_default_is_empty_and_closed() {
        let response = ComboBoxResponse::default();
        assert_eq!(response.selected_index, 0);
        assert_eq!(response.selected_text, "");
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
        assert!(!response.is_open);
    }

    #[test]
    fn combo_box_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let combo = ui
            .widgets()
            .combo_box()
            .id("combo")
            .width(280)
            .height(44)
            .items(["a", "b", "c"])
            .selected_index(1)
            .font_size(19)
            .text_color(Color::from_rgb8(240, 240, 240))
            .bg_color(Color::from_rgb8(30, 30, 30))
            .border_color(Color::from_rgb8(90, 90, 90))
            .enabled(false)
            .show();
        assert_eq!(combo.selected_text(&mut ui), "b");

        combo.set_enabled(&mut ui, true);
        combo.set_selected_index(&mut ui, 2);
        assert_eq!(combo.selected_text(&mut ui), "c");
        combo.set_items(&mut ui, ["x", "y"]);
        assert_eq!(combo.selected_index(&mut ui), 1);
    }
