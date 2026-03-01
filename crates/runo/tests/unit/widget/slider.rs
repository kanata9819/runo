    use vello::Scene;
    use vello::peniko::Color;

    use super::SliderResponse;
    use crate::hooks::use_effect::EffectStore;
    use crate::hooks::use_state::StateStore;
    use crate::retained::RetainedState;
    use crate::ui::Ui;

    #[test]
    fn slider_response_default_is_zero_and_idle() {
        let response = SliderResponse::default();
        assert_eq!(response.value, 0.0);
        assert!(!response.hovered);
        assert!(!response.pressed);
        assert!(!response.changed);
    }

    #[test]
    fn slider_builder_methods_and_show_work() {
        let mut scene = Scene::new();
        let mut effects = EffectStore::new();
        let mut states = StateStore::new();
        let mut retained = RetainedState::new();
        let mut ui = Ui::new(&mut scene, None, &mut effects, &mut states, &mut retained);

        let slider = ui
            .widgets()
            .slider()
            .id("sl")
            .width(220)
            .height(30)
            .range(-1.0, 1.0)
            .value(0.5)
            .step(0.25)
            .text("volume")
            .font_size(13)
            .text_color(Color::from_rgb8(220, 220, 220))
            .enabled(false)
            .show();
        assert!((slider.value(&mut ui) - 0.5).abs() < f64::EPSILON);

        slider.set_enabled(&mut ui, true);
        slider.set_value(&mut ui, -0.5);
        assert!((slider.value(&mut ui) + 0.5).abs() < f64::EPSILON);
    }
