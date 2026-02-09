use std::collections::HashMap;

use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Color, Fill, FontData};
use vello::Scene;

use crate::ButtonResponse;
use crate::input::InputFrame;
use crate::widget::text::{draw_text_run, layout_text};

pub(crate) struct RetainedState {
    widgets: HashMap<String, WidgetNode>,
    order: Vec<String>,
    active_button: Option<String>,
}

impl RetainedState {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            order: Vec::new(),
            active_button: None,
        }
    }

    pub(crate) fn begin_frame_input(&mut self, input: InputFrame) {
        for node in self.widgets.values_mut() {
            if let WidgetNode::Button(button) = node {
                button.clicked = false;
                button.hovered = contains(button.rect, input.cursor_pos.0, input.cursor_pos.1);
            }
        }

        if input.mouse_pressed {
            self.active_button = self.order.iter().rev().find_map(|id| {
                let WidgetNode::Button(button) = self.widgets.get(id)? else {
                    return None;
                };
                if button.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });
        }

        for (id, node) in &mut self.widgets {
            if let WidgetNode::Button(button) = node {
                button.pressed = input.mouse_down
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false);
                if input.mouse_released
                    && button.hovered
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                {
                    button.clicked = true;
                }
            }
        }

        if input.mouse_released {
            self.active_button = None;
        }
    }

    pub(crate) fn upsert_button(
        &mut self,
        id: String,
        rect: Rect,
        text: Option<String>,
        text_color: Color,
    ) -> ButtonResponse {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::Button(ButtonNode {
                    rect,
                    text,
                    text_color,
                    hovered: false,
                    pressed: false,
                    clicked: false,
                }),
            );
            return ButtonResponse::default();
        }

        let entry = self.widgets.get_mut(&id).expect("button entry missing");
        match entry {
            WidgetNode::Button(button) => {
                button.rect = rect;
                button.text = text;
                button.text_color = text_color;
                ButtonResponse {
                    hovered: button.hovered,
                    pressed: button.pressed,
                    clicked: button.clicked,
                }
            }
            WidgetNode::Label(_) => {
                *entry = WidgetNode::Button(ButtonNode {
                    rect,
                    text,
                    text_color,
                    hovered: false,
                    pressed: false,
                    clicked: false,
                });
                ButtonResponse::default()
            }
        }
    }

    pub(crate) fn upsert_label(
        &mut self,
        id: String,
        rect: Rect,
        text: String,
        font_size: f32,
        text_color: Color,
    ) {
        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(
                id.clone(),
                WidgetNode::Label(LabelNode {
                    rect,
                    text,
                    font_size,
                    text_color,
                }),
            );
            return;
        }

        self.widgets.insert(
            id,
            WidgetNode::Label(LabelNode {
                rect,
                text,
                font_size,
                text_color,
            }),
        );
    }

    pub(crate) fn render(&self, scene: &mut Scene, font: Option<&FontData>) {
        for id in &self.order {
            let Some(node) = self.widgets.get(id) else {
                continue;
            };
            match node {
                WidgetNode::Button(button) => render_button(scene, font, button),
                WidgetNode::Label(label) => render_label(scene, font, label),
            }
        }
    }

    pub(crate) fn button_response(&self, id: &str) -> ButtonResponse {
        let Some(WidgetNode::Button(button)) = self.widgets.get(id) else {
            return ButtonResponse::default();
        };
        ButtonResponse {
            hovered: button.hovered,
            pressed: button.pressed,
            clicked: button.clicked,
        }
    }

    pub(crate) fn set_button_text(&mut self, id: &str, text: Option<String>) {
        let Some(WidgetNode::Button(button)) = self.widgets.get_mut(id) else {
            return;
        };
        button.text = text;
    }
}

enum WidgetNode {
    Button(ButtonNode),
    Label(LabelNode),
}

struct ButtonNode {
    rect: Rect,
    text: Option<String>,
    text_color: Color,
    hovered: bool,
    pressed: bool,
    clicked: bool,
}

struct LabelNode {
    rect: Rect,
    text: String,
    font_size: f32,
    text_color: Color,
}

fn render_button(scene: &mut Scene, font: Option<&FontData>, button: &ButtonNode) {
    let color = if button.pressed {
        Color::from_rgb8(31, 122, 205)
    } else if button.hovered {
        Color::from_rgb8(69, 160, 242)
    } else {
        Color::from_rgb8(50, 144, 229)
    };

    let rounded = RoundedRect::from_rect(button.rect, 10.0);
    scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rounded);

    let (Some(font), Some(text)) = (font, button.text.as_deref()) else {
        return;
    };
    let font_size = 18.0_f32;
    let Some((glyphs, total_advance)) = layout_text(font, text, font_size) else {
        return;
    };

    let text_x = button.rect.x0 + (button.rect.width() - total_advance as f64) * 0.5;
    let text_y = button.rect.y0 + button.rect.height() * 0.5 + font_size as f64 * 0.35;
    draw_text_run(scene, font, glyphs, text_x, text_y, font_size, button.text_color);
}

fn render_label(scene: &mut Scene, font: Option<&FontData>, label: &LabelNode) {
    let Some(font) = font else {
        return;
    };
    let Some((glyphs, _)) = layout_text(font, &label.text, label.font_size) else {
        return;
    };
    let baseline_y = label.rect.y0 + label.font_size as f64;
    draw_text_run(
        scene,
        font,
        glyphs,
        label.rect.x0,
        baseline_y,
        label.font_size,
        label.text_color,
    );
}

fn contains(rect: Rect, x: f64, y: f64) -> bool {
    x >= rect.x0 && x <= rect.x1 && y >= rect.y0 && y <= rect.y1
}
