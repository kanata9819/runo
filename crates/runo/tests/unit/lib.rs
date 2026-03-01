    use vello::Scene;
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use crate::hooks::use_effect::EffectStore;
    use crate::hooks::use_state::StateStore;
    use crate::layout::{LayoutDirection, LayoutNode};
    use crate::retained::{
        RetainedState, UpsertCheckboxArgs, UpsertComboBoxArgs, UpsertRadioButtonArgs,
        UpsertSliderArgs, UpsertTextBoxArgs,
    };
    use crate::{Overflow, RunOptions, Ui};

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 200.0, 64.0)
    }

    fn white() -> Color {
        Color::from_rgb8(255, 255, 255)
    }

    #[test]
    fn run_options_default_includes_resizable() {
        let options = RunOptions::default();
        assert_eq!(options.window_title, "runo app");
        assert_eq!(options.window_width, 640);
        assert_eq!(options.window_height, 480);
        assert!(options.window_resizable);
    }

    #[test]
    fn layout_node_vertical_and_horizontal_consumption() {
        let mut vertical = LayoutNode::new((10.0, 20.0), LayoutDirection::Vertical, 8.0);
        assert_eq!(vertical.place(100.0, 40.0), (10.0, 20.0));
        vertical.advance(100.0, 40.0);
        assert_eq!(vertical.place(30.0, 15.0), (10.0, 68.0));
        vertical.advance(30.0, 15.0);
        assert_eq!(vertical.consumed_size(), (100.0, 63.0));

        let mut horizontal = LayoutNode::new((5.0, 7.0), LayoutDirection::Horizontal, 4.0);
        assert_eq!(horizontal.place(50.0, 11.0), (5.0, 7.0));
        horizontal.advance(50.0, 11.0);
        assert_eq!(horizontal.place(2.0, 30.0), (59.0, 7.0));
        horizontal.advance(2.0, 30.0);
        assert_eq!(horizontal.consumed_size(), (56.0, 30.0));
    }

    #[test]
    fn retained_checkbox_change_flag_behaves_as_expected() {
        let mut retained = RetainedState::new();
        retained.upsert_checkbox(UpsertCheckboxArgs {
            id: "check".to_string(),
            rect: rect(),
            text: Some("check".to_string()),
            checked: Some(false),
            font_size: 16.0,
            text_color: white(),
            enabled: true,
        });

        retained.set_checkbox_checked("check", true);
        let changed = retained.checkbox_response("check");
        assert!(changed.checked);
        assert!(changed.changed);

        retained.set_checkbox_checked("check", true);
        let unchanged = retained.checkbox_response("check");
        assert!(unchanged.checked);
        assert!(!unchanged.changed);
    }

    #[test]
    fn retained_radio_group_is_exclusive() {
        let mut retained = RetainedState::new();
        retained.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 16.0,
            text_color: white(),
            enabled: true,
        });
        retained.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r2".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 16.0,
            text_color: white(),
            enabled: true,
        });

        assert!(!retained.radio_button_response("r1").selected);
        assert!(retained.radio_button_response("r2").selected);

        retained.set_radio_button_selected("r1", true);
        assert!(retained.radio_button_response("r1").selected);
        assert!(!retained.radio_button_response("r2").selected);
    }

    #[test]
    fn retained_combo_box_selection_is_clamped_on_mutations() {
        let mut retained = RetainedState::new();
        retained.upsert_combo_box(UpsertComboBoxArgs {
            id: "combo".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            selected_index: Some(2),
            font_size: 14.0,
            text_color: white(),
            bg_color: white(),
            border_color: white(),
            enabled: true,
        });

        retained.set_combo_box_selected_index("combo", 50);
        let clamped = retained.combo_box_response("combo");
        assert_eq!(clamped.selected_index, 2);
        assert_eq!(clamped.selected_text, "c");

        retained.set_combo_box_items("combo", ["x"]);
        let shrunk = retained.combo_box_response("combo");
        assert_eq!(shrunk.selected_index, 0);
        assert_eq!(shrunk.selected_text, "x");
    }

    #[test]
    fn retained_slider_snaps_and_clamps() {
        let mut retained = RetainedState::new();
        retained.upsert_slider(UpsertSliderArgs {
            id: "slider".to_string(),
            rect: rect(),
            min: 0.0,
            max: 1.0,
            value: Some(0.0),
            step: Some(0.25),
            text: None,
            font_size: 14.0,
            text_color: white(),
            enabled: true,
        });

        retained.set_slider_value("slider", 0.87);
        let response = retained.slider_response("slider");
        assert!((response.value - 0.75).abs() < f64::EPSILON);
        assert!(response.changed);
    }

    #[test]
    fn retained_text_box_set_text_updates_response() {
        let mut retained = RetainedState::new();
        retained.upsert_text_box(UpsertTextBoxArgs {
            id: "tb".to_string(),
            rect: rect(),
            text: Some("hello".to_string()),
            placeholder: Some("type".to_string()),
            font_size: 16.0,
            text_color: white(),
            bg_color: Color::from_rgb8(20, 20, 20),
            border_color: white(),
            enabled: true,
            overflow_x: Overflow::Auto,
            overflow_y: Overflow::Hidden,
        });

        retained.set_text_box_text("tb", "updated");
        let response = retained.text_box_response("tb");
        assert_eq!(response.text, "updated");
        assert!(response.changed);
    }

    #[test]
    fn ui_state_accessors_mutate_retained_state() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();

        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);
        ui.widgets()
            .checkbox()
            .id("c")
            .text("Check")
            .checked(false)
            .show();
        ui.widgets().text_box().id("tb").text("before").show();
        ui.widgets()
            .slider()
            .id("s")
            .range(0.0, 1.0)
            .value(0.0)
            .show();

        ui.state().checkbox().set_checked("c", true);
        ui.state().text_box().set_text("tb", "after");
        ui.state().slider().set_value("s", 0.6);

        assert!(ui.state().checkbox().checked("c"));
        assert_eq!(ui.state().text_box().text("tb"), "after");
        assert!((ui.state().slider().value("s") - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn ui_use_state_returns_value_and_setter() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();

        states.begin_frame();
        {
            let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);
            let (count, set_count) = ui.use_state("counter", || 0_u32);
            assert_eq!(count, 0);
            assert!(set_count.set(&mut ui, 3));
            let (updated, _) = ui.use_state("counter", || 99_u32);
            assert_eq!(updated, 3);
        }
        states.end_frame();
    }

    #[test]
    fn ui_use_state_persists_across_ui_instances_with_frame_lifecycle() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();

        states.begin_frame();
        {
            let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);
            let (_, set_value) = ui.use_state("persist", || 1_u32);
            set_value.set(&mut ui, 8_u32);
        }
        states.end_frame();

        states.begin_frame();
        {
            let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);
            let (value, _) = ui.use_state("persist", || 0_u32);
            assert_eq!(value, 8);
        }
        states.end_frame();
    }
