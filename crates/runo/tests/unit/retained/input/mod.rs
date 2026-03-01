    use super::*;

    #[test]
    fn begin_frame_input_runs_with_default_input_and_no_widgets() {
        let mut state = RetainedState::new();
        let input = InputFrame {
            cursor_pos: (0.0, 0.0),
            mouse_down: false,
            mouse_pressed: false,
            mouse_released: false,
            text_input: String::new(),
            backspace_pressed: false,
            delete_pressed: false,
            enter_pressed: false,
            arrow_left_pressed: false,
            arrow_right_pressed: false,
            arrow_up_pressed: false,
            arrow_down_pressed: false,
            copy_pressed: false,
            paste_pressed: false,
            scroll_x: 0.0,
            scroll_y: 0.0,
        };

        state.begin_frame_input(input, None);
        assert!(state.drain_events().is_empty());
    }
