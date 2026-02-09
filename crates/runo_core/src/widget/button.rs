use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill};

use crate::Ui;
use crate::widget::text::{draw_text_run, layout_text};

#[derive(Clone, Copy)]
pub struct ButtonResponse {
    pub hovered: bool,
    pub pressed: bool,
    pub clicked: bool,
}

pub struct ButtonBuilder<'ui, 'a> {
    ui: &'ui mut Ui<'a>,
    id: String,
    width: f64,
    height: f64,
    text: Option<String>,
    text_color: Color,
}

impl<'ui, 'a> ButtonBuilder<'ui, 'a> {
    pub fn new(ui: &'ui mut Ui<'a>, id: String) -> Self {
        Self {
            ui,
            id,
            width: 180.0,
            height: 56.0,
            text: None,
            text_color: Color::from_rgb8(245, 248, 252),
        }
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn show(self) -> ButtonResponse {
        let (x, y) = self.ui.allocate_rect(self.width, self.height);
        let rect = Rect::new(x, y, x + self.width, y + self.height);
        let hovered = contains(rect, self.ui.input.cursor_pos.0, self.ui.input.cursor_pos.1);
        let widget_id = hash_id(&self.id);

        if self.ui.input.mouse_pressed && hovered {
            *self.ui.active_button = Some(widget_id);
        }

        let pressed = self.ui.input.mouse_down && *self.ui.active_button == Some(widget_id);
        let clicked =
            self.ui.input.mouse_released && hovered && *self.ui.active_button == Some(widget_id);

        let color = if pressed {
            Color::from_rgb8(31, 122, 205)
        } else if hovered {
            Color::from_rgb8(69, 160, 242)
        } else {
            Color::from_rgb8(50, 144, 229)
        };

        let rounded = RoundedRect::from_rect(rect, 10.0);
        self.ui
            .scene
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);

        if let Some(text) = self.text {
            draw_button_text(
                self.ui,
                &text,
                x,
                y,
                self.width,
                self.height,
                self.text_color,
            );
        }

        ButtonResponse {
            hovered,
            pressed,
            clicked,
        }
    }
}

fn draw_button_text(ui: &mut Ui<'_>, text: &str, x: f64, y: f64, w: f64, h: f64, color: Color) {
    let Some(font) = ui.font.as_ref() else {
        return;
    };
    let font_size = 18.0_f32;
    let Some((glyphs, total_advance)) = layout_text(font, text, font_size) else {
        return;
    };
    let text_x = x + (w - total_advance as f64) * 0.5;
    let text_y = y + h * 0.5 + font_size as f64 * 0.35;
    draw_text_run(ui.scene, font, glyphs, text_x, text_y, font_size, color);
}

fn contains(rect: Rect, x: f64, y: f64) -> bool {
    x >= rect.x0 && x <= rect.x1 && y >= rect.y0 && y <= rect.y1
}

fn hash_id(id: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}
