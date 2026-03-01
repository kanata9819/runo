    use super::*;
    use crate::layout::LayoutDirection;

    #[test]
    fn allocate_rect_advances_vertical_cursor_with_spacing() {
        let mut stack = LayoutStack::new((10.0, 20.0), LayoutDirection::Vertical, 5.0);

        let first = stack.allocate_rect(50.0, 10.0);
        let second = stack.allocate_rect(30.0, 8.0);

        assert_eq!(first, (10.0, 20.0));
        assert_eq!(second, (10.0, 35.0));
    }

    #[test]
    fn push_layout_uses_parent_next_position() {
        let mut stack = LayoutStack::new((0.0, 0.0), LayoutDirection::Vertical, 4.0);
        let _ = stack.allocate_rect(20.0, 10.0);

        stack.push_layout(LayoutDirection::Horizontal, 2.0);
        let child_pos = stack.allocate_rect(5.0, 5.0);

        assert_eq!(child_pos, (0.0, 14.0));
    }

    #[test]
    fn pop_layout_and_advance_parent_moves_parent_cursor_by_child_consumed_size() {
        let mut stack = LayoutStack::new((0.0, 0.0), LayoutDirection::Vertical, 3.0);

        stack.push_layout(LayoutDirection::Vertical, 2.0);
        let _ = stack.allocate_rect(20.0, 10.0);
        let _ = stack.allocate_rect(30.0, 6.0);
        stack.pop_layout_and_advance_parent();

        let next_parent_pos = stack.peek_next_position();
        assert_eq!(next_parent_pos, (0.0, 21.0));
    }

    #[test]
    fn pop_layout_consumed_returns_cross_and_main_sizes() {
        let mut stack = LayoutStack::new((0.0, 0.0), LayoutDirection::Vertical, 0.0);
        stack.push_layout_at((10.0, 10.0), LayoutDirection::Horizontal, 1.0);
        let _ = stack.allocate_rect(5.0, 9.0);
        let _ = stack.allocate_rect(7.0, 4.0);

        let consumed = stack.pop_layout_consumed();
        assert_eq!(consumed, (13.0, 9.0));
    }
