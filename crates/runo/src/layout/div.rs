use vello::peniko::Color;

use crate::Ui;
use crate::layout::LayoutDirection;
use crate::ui::ShowDivArgs;

#[derive(Clone)]
struct DivConfig {
    id: String,
    direction: LayoutDirection,
    gap: f64,
    width: Option<f64>,
    height: Option<f64>,
    padding_left: f64,
    padding_top: f64,
    padding_right: f64,
    padding_bottom: f64,
    bg_color: Option<Color>,
    border_color: Option<Color>,
    border_width: f64,
    radius: f64,
}

impl DivConfig {
    fn new(id: String) -> Self {
        Self {
            id,
            direction: LayoutDirection::Vertical,
            gap: 12.0,
            width: None,
            height: None,
            padding_left: 0.0,
            padding_top: 0.0,
            padding_right: 0.0,
            padding_bottom: 0.0,
            bg_color: None,
            border_color: None,
            border_width: 1.0,
            radius: 0.0,
        }
    }

    fn set_padding_all(&mut self, px: u32) {
        let px = px as f64;
        self.padding_left = px;
        self.padding_top = px;
        self.padding_right = px;
        self.padding_bottom = px;
    }

    fn set_padding_x(&mut self, px: u32) {
        let px = px as f64;
        self.padding_left = px;
        self.padding_right = px;
    }

    fn set_padding_y(&mut self, px: u32) {
        let px = px as f64;
        self.padding_top = px;
        self.padding_bottom = px;
    }

    fn into_show_args(self) -> ShowDivArgs {
        ShowDivArgs {
            id: self.id,
            direction: self.direction,
            gap: self.gap,
            width: self.width,
            height: self.height,
            padding_left: self.padding_left,
            padding_top: self.padding_top,
            padding_right: self.padding_right,
            padding_bottom: self.padding_bottom,
            bg_color: self.bg_color,
            border_color: self.border_color,
            border_width: self.border_width,
            radius: self.radius,
        }
    }
}

pub struct DivBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    config: DivConfig,
}

impl<'ui, 'a> DivBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            config: DivConfig::new(id),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.config.id = id.into();
        self
    }

    pub fn vertical(mut self) -> Self {
        self.config.direction = LayoutDirection::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.config.direction = LayoutDirection::Horizontal;
        self
    }

    pub fn gap(mut self, px: u32) -> Self {
        self.config.gap = px as f64;
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.config.width = Some(px as f64);
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.config.height = Some(px as f64);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.config.width = Some(width as f64);
        self.config.height = Some(height as f64);
        self
    }

    pub fn padding(mut self, px: u32) -> Self {
        self.config.set_padding_all(px);
        self
    }

    pub fn padding_x(mut self, px: u32) -> Self {
        self.config.set_padding_x(px);
        self
    }

    pub fn padding_y(mut self, px: u32) -> Self {
        self.config.set_padding_y(px);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.config.bg_color = Some(color);
        self
    }

    pub fn border(mut self, color: Color, width: u32) -> Self {
        self.config.border_color = Some(color);
        self.config.border_width = width as f64;
        self
    }

    pub fn radius(mut self, px: u32) -> Self {
        self.config.radius = px as f64;
        self
    }

    pub fn show<R>(self, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        self.ui.show_div(self.config.into_show_args(), f)
    }
}

#[cfg(test)]
mod tests {
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
}
