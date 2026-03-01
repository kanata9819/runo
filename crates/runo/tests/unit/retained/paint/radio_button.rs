    use super::*;
    use crate::font::load_default_font;
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    /// Creates a reusable radio button fixture for helper-function tests.
    fn sample_radio_button() -> RadioButtonNode {
        RadioButtonNode {
            rect: Rect::new(0.0, 0.0, 180.0, 24.0),
            group: "group".to_string(),
            text: Some("Radio".to_string()),
            selected: false,
            font_size: 14.0,
            text_color: Color::from_rgb8(255, 255, 255),
            enabled: true,
            hovered: false,
            pressed: false,
            changed: false,
        }
    }

    #[test]
    /// Clamps indicator size to minimum when control is short.
    fn indicator_size_clamps_to_min() {
        assert_eq!(indicator_size(10.0), 14.0);
    }

    #[test]
    /// Clamps indicator size to maximum when control is tall.
    fn indicator_size_clamps_to_max() {
        assert_eq!(indicator_size(100.0), 24.0);
    }

    #[test]
    /// Prioritizes pressed color over hovered color.
    fn outer_bg_color_prefers_pressed() {
        let mut radio_button = sample_radio_button();
        radio_button.pressed = true;
        radio_button.hovered = true;
        assert_eq!(
            outer_bg_color(&radio_button),
            Color::from_rgb8(45, 129, 205)
        );
    }

    #[test]
    /// Uses disabled color regardless of interaction states.
    fn outer_bg_color_uses_disabled_color() {
        let mut radio_button = sample_radio_button();
        radio_button.enabled = false;
        radio_button.pressed = true;
        radio_button.hovered = true;
        assert_eq!(outer_bg_color(&radio_button), Color::from_rgb8(43, 47, 53));
    }

    #[test]
    fn render_runs_for_selected_and_unselected_states() {
        let mut scene = Scene::new();
        let mut radio = sample_radio_button();
        render(&mut scene, None, &radio);

        if let Some(font) = load_default_font() {
            render(&mut scene, Some(&font), &radio);
            radio.selected = true;
            render(&mut scene, Some(&font), &radio);
        }
    }
