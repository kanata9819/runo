    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use crate::retained::node::WidgetNode;
    use crate::retained::state::{RetainedState, UpsertComboBoxArgs};

    fn rect() -> Rect {
        Rect::new(0.0, 0.0, 200.0, 40.0)
    }

    #[test]
    fn upsert_combo_box_clamps_selected_index_on_insert() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        let response = state.upsert_combo_box(UpsertComboBoxArgs {
            id: "co".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string()],
            selected_index: Some(99),
            font_size: 16.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });
        assert_eq!(response.selected_index, 1);
        assert_eq!(response.selected_text, "b");
    }

    #[test]
    fn set_combo_box_enabled_false_clears_flags_and_active_id() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_combo_box(UpsertComboBoxArgs {
            id: "co".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string()],
            selected_index: Some(0),
            font_size: 16.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });
        state.active_combo_box = Some("co".to_string());
        if let Some(WidgetNode::ComboBox(co)) = state.widgets.get_mut("co") {
            co.hovered = true;
            co.hovered_item = Some(1);
            co.pressed = true;
            co.changed = true;
            co.is_open = true;
        }

        state.set_combo_box_enabled("co", false);
        if let Some(WidgetNode::ComboBox(co)) = state.widgets.get("co") {
            assert!(!co.enabled);
            assert!(!co.hovered);
            assert!(co.hovered_item.is_none());
            assert!(!co.pressed);
            assert!(!co.changed);
            assert!(!co.is_open);
        } else {
            panic!("combo box missing");
        }
        assert!(state.active_combo_box.is_none());
    }

    #[test]
    fn upsert_existing_combo_box_clamps_selected_when_items_shrink() {
        let mut state = RetainedState::new();
        let color = Color::from_rgb8(240, 240, 240);
        state.upsert_combo_box(UpsertComboBoxArgs {
            id: "co".to_string(),
            rect: rect(),
            items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            selected_index: Some(2),
            font_size: 16.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });

        let response = state.upsert_combo_box(UpsertComboBoxArgs {
            id: "co".to_string(),
            rect: rect(),
            items: vec!["x".to_string()],
            selected_index: None,
            font_size: 16.0,
            text_color: color,
            bg_color: color,
            border_color: color,
            enabled: true,
        });
        assert_eq!(response.selected_index, 0);
        assert_eq!(response.selected_text, "x");
    }
