    use super::*;

    #[test]
    fn div_config_defaults_are_expected() {
        let config = DivConfig::new("div".to_string());
        assert!(matches!(config.direction, LayoutDirection::Vertical));
        assert_eq!(config.gap, 12.0);
        assert_eq!(config.width, None);
        assert_eq!(config.height, None);
        assert_eq!(config.padding_left, 0.0);
        assert_eq!(config.padding_top, 0.0);
        assert_eq!(config.padding_right, 0.0);
        assert_eq!(config.padding_bottom, 0.0);
        assert_eq!(config.border_width, 1.0);
        assert_eq!(config.radius, 0.0);
    }

    #[test]
    fn padding_helpers_update_expected_axes() {
        let mut config = DivConfig::new("div".to_string());
        config.set_padding_all(8);
        assert_eq!(config.padding_left, 8.0);
        assert_eq!(config.padding_top, 8.0);
        assert_eq!(config.padding_right, 8.0);
        assert_eq!(config.padding_bottom, 8.0);

        config.set_padding_x(12);
        assert_eq!(config.padding_left, 12.0);
        assert_eq!(config.padding_right, 12.0);
        assert_eq!(config.padding_top, 8.0);
        assert_eq!(config.padding_bottom, 8.0);

        config.set_padding_y(16);
        assert_eq!(config.padding_top, 16.0);
        assert_eq!(config.padding_bottom, 16.0);
        assert_eq!(config.padding_left, 12.0);
        assert_eq!(config.padding_right, 12.0);
    }

    #[test]
    fn into_show_args_preserves_values() {
        let mut config = DivConfig::new("main".to_string());
        config.direction = LayoutDirection::Horizontal;
        config.gap = 10.0;
        config.width = Some(320.0);
        config.height = Some(120.0);
        config.set_padding_all(6);
        config.border_width = 2.0;
        config.radius = 4.0;

        let args = config.into_show_args();
        assert_eq!(args.id, "main");
        assert!(matches!(args.direction, LayoutDirection::Horizontal));
        assert_eq!(args.gap, 10.0);
        assert_eq!(args.width, Some(320.0));
        assert_eq!(args.height, Some(120.0));
        assert_eq!(args.padding_left, 6.0);
        assert_eq!(args.padding_top, 6.0);
        assert_eq!(args.padding_right, 6.0);
        assert_eq!(args.padding_bottom, 6.0);
        assert_eq!(args.border_width, 2.0);
        assert_eq!(args.radius, 4.0);
    }
