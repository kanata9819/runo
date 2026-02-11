use vello::peniko::Color;

use crate::Ui;
use crate::layout::LayoutDirection;
use crate::ui::ShowDivArgs;

pub struct DivBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
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

impl<'ui, 'a> DivBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
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

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = LayoutDirection::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.direction = LayoutDirection::Horizontal;
        self
    }

    pub fn gap(mut self, px: u32) -> Self {
        self.gap = px as f64;
        self
    }

    pub fn width(mut self, px: u32) -> Self {
        self.width = Some(px as f64);
        self
    }

    pub fn height(mut self, px: u32) -> Self {
        self.height = Some(px as f64);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width as f64);
        self.height = Some(height as f64);
        self
    }

    pub fn padding(mut self, px: u32) -> Self {
        let px = px as f64;
        self.padding_left = px;
        self.padding_top = px;
        self.padding_right = px;
        self.padding_bottom = px;
        self
    }

    pub fn padding_x(mut self, px: u32) -> Self {
        let px = px as f64;
        self.padding_left = px;
        self.padding_right = px;
        self
    }

    pub fn padding_y(mut self, px: u32) -> Self {
        let px = px as f64;
        self.padding_top = px;
        self.padding_bottom = px;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn border(mut self, color: Color, width: u32) -> Self {
        self.border_color = Some(color);
        self.border_width = width as f64;
        self
    }

    pub fn radius(mut self, px: u32) -> Self {
        self.radius = px as f64;
        self
    }

    pub fn show<R>(self, f: impl FnOnce(&mut Ui<'a>) -> R) -> R {
        self.ui.show_div(
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
            },
            f,
        )
    }
}
