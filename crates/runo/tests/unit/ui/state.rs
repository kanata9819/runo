    use vello::Scene;
    use vello::peniko::Color;

    use crate::hooks::use_effect::EffectStore;
    use crate::hooks::use_state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn ui_state_accessors_cover_all_widget_state_apis() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let button = ui.widgets().button().id("btn").text("b").show();
        ui.widgets().text_box().id("tb").text("x").show();
        ui.widgets().checkbox().id("cb").checked(false).show();
        ui.widgets()
            .radio_button()
            .id("rb")
            .group("g")
            .selected(false)
            .show();
        ui.widgets()
            .slider()
            .id("sl")
            .range(0.0, 1.0)
            .value(0.2)
            .show();
        ui.widgets()
            .combo_box()
            .id("co")
            .items(["a", "b"])
            .selected_index(0)
            .show();
        ui.widgets().label().id("lb").text("label").show();
        ui.widgets()
            .div()
            .id("dv")
            .background(Color::from_rgb8(20, 20, 20))
            .show(|_| ());

        let _ = ui.state().button().response_handle(&button);
        assert!(!ui.state().button().clicked_handle(&button));
        ui.state().button().set_text("btn", "next");
        ui.state().button().set_enabled("btn", true);
        ui.state().button().set_text_handle(&button, "next2");
        ui.state().button().set_enabled_handle(&button, true);

        let _ = ui.state().text_box().response("tb");
        assert_eq!(ui.state().text_box().text("tb"), "x");
        ui.state().text_box().set_text("tb", "xx");
        ui.state().text_box().set_enabled("tb", true);

        let _ = ui.state().checkbox().response("cb");
        assert!(!ui.state().checkbox().checked("cb"));
        ui.state().checkbox().set_checked("cb", true);
        ui.state().checkbox().set_enabled("cb", true);

        let _ = ui.state().radio_button().response("rb");
        assert!(!ui.state().radio_button().selected("rb"));
        ui.state().radio_button().set_selected("rb", true);
        ui.state().radio_button().set_enabled("rb", true);

        let _ = ui.state().slider().response("sl");
        let _ = ui.state().slider().value("sl");
        ui.state().slider().set_value("sl", 0.7);
        ui.state().slider().set_enabled("sl", true);

        let _ = ui.state().combo_box().response("co");
        let _ = ui.state().combo_box().selected_text("co");
        let _ = ui.state().combo_box().selected_index("co");
        ui.state().combo_box().set_selected_index("co", 1);
        ui.state().combo_box().set_items("co", ["x", "y", "z"]);
        ui.state().combo_box().set_enabled("co", true);

        ui.state().label().set_enabled("lb", true);
        ui.state().div().set_visible("dv", true);
        ui.state().div().set_enabled("dv", true);
        ui.state()
            .div()
            .set_background("dv", Color::from_rgb8(30, 30, 30));
        ui.state().div().clear_background("dv");
    }
