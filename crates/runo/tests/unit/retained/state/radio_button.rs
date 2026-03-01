    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use crate::retained::node::WidgetNode;
    use crate::retained::state::{RetainedState, UpsertRadioButtonArgs};

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 180.0, 36.0)
    }

    #[test]
    fn selecting_true_clears_other_nodes_in_same_group() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r2".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });

        state.set_radio_button_selected("r2", true);
        assert!(!state.radio_button_response("r1").selected);
        assert!(state.radio_button_response("r2").selected);
    }

    #[test]
    fn set_radio_button_enabled_false_clears_flags_and_active_id() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.active_radio_button = Some("r1".to_string());
        if let Some(WidgetNode::RadioButton(r)) = state.widgets.get_mut("r1") {
            r.hovered = true;
            r.pressed = true;
            r.changed = true;
        }

        state.set_radio_button_enabled("r1", false);
        if let Some(WidgetNode::RadioButton(r)) = state.widgets.get("r1") {
            assert!(!r.enabled);
            assert!(!r.hovered);
            assert!(!r.pressed);
            assert!(!r.changed);
        } else {
            panic!("radio button missing");
        }
        assert!(state.active_radio_button.is_none());
    }

    #[test]
    fn set_radio_button_selected_false_updates_changed_flag() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: None,
            selected: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });

        state.set_radio_button_selected("r1", false);
        let response = state.radio_button_response("r1");
        assert!(!response.selected);
        assert!(response.changed);
    }

    #[test]
    fn upsert_radio_button_updates_existing_entry_fields() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g".to_string(),
            rect: rect(),
            text: Some("a".to_string()),
            selected: Some(false),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        let new_rect = Rect::new(5.0, 6.0, 55.0, 26.0);
        let response = state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "r1".to_string(),
            group: "g2".to_string(),
            rect: new_rect,
            text: Some("b".to_string()),
            selected: Some(true),
            font_size: 18.0,
            text_color: color,
            enabled: false,
        });
        assert!(!response.selected);
        if let Some(WidgetNode::RadioButton(rb)) = state.widgets.get("r1") {
            assert_eq!(rb.group, "g2");
            assert_eq!(rb.text.as_deref(), Some("b"));
            assert_eq!(rb.rect, new_rect);
            assert_eq!(rb.font_size, 18.0);
            assert!(!rb.enabled);
        } else {
            panic!("radio button missing");
        }
    }
