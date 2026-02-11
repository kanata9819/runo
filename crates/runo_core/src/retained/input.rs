use vello::kurbo::Rect;
use vello::peniko::FontData;

use crate::event::UiEvent;
use crate::input::InputFrame;
use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;
use crate::widget::text::{estimate_text_width, layout_text};

impl RetainedState {
    pub(crate) fn begin_frame_input(&mut self, input: InputFrame, font: Option<&FontData>) {
        self.update_hover_flags(input.cursor_pos);
        self.handle_mouse_press(input.mouse_pressed);
        self.handle_text_box_scrollbar_input(
            input.mouse_pressed,
            input.mouse_down,
            input.mouse_released,
            input.cursor_pos,
        );
        self.update_button_states(input.mouse_down, input.mouse_released);
        self.update_combo_box_states(input.mouse_down, input.mouse_released);
        self.update_text_box_focus();
        self.apply_text_box_scroll(&input);
        self.apply_text_input(&input, font);
    }

    fn update_hover_flags(&mut self, cursor_pos: (f64, f64)) {
        let open_overlay_id = self.order.iter().rev().find_map(|id| {
            let WidgetNode::ComboBox(combo_box) = self.widgets.get(id)? else {
                return None;
            };
            if combo_box.enabled
                && combo_box.is_open
                && combo_expanded_contains(combo_box, cursor_pos.0, cursor_pos.1)
            {
                Some(id.clone())
            } else {
                None
            }
        });

        for (id, node) in &mut self.widgets {
            match node {
                WidgetNode::Button(button) => {
                    button.clicked = false;
                    button.hovered = if !button.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(button.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::TextBox(text_box) => {
                    text_box.changed = false;
                    text_box.hovered = if !text_box.enabled || open_overlay_id.is_some() {
                        false
                    } else {
                        contains(text_box.rect, cursor_pos.0, cursor_pos.1)
                    };
                }
                WidgetNode::ComboBox(combo_box) => {
                    combo_box.changed = false;
                    if !combo_box.enabled {
                        combo_box.hovered = false;
                        combo_box.hovered_item = None;
                        combo_box.pressed = false;
                        combo_box.is_open = false;
                        continue;
                    }
                    combo_box.hovered = contains(combo_box.rect, cursor_pos.0, cursor_pos.1);
                    combo_box.hovered_item = if open_overlay_id
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                    {
                        combo_item_index_at(combo_box, cursor_pos.0, cursor_pos.1)
                    } else {
                        None
                    };
                }
                WidgetNode::Label(_) => {}
            }
        }
    }

    fn handle_mouse_press(&mut self, mouse_pressed: bool) {
        if mouse_pressed {
            self.active_button = self.order.iter().rev().find_map(|id| {
                let WidgetNode::Button(button) = self.widgets.get(id)? else {
                    return None;
                };
                if button.enabled && button.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.focused_text_box = self.order.iter().rev().find_map(|id| {
                let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                    return None;
                };
                if text_box.enabled && text_box.hovered {
                    Some(id.clone())
                } else {
                    None
                }
            });

            self.active_combo_box = self.order.iter().rev().find_map(|id| {
                let WidgetNode::ComboBox(combo_box) = self.widgets.get(id)? else {
                    return None;
                };
                if combo_box.enabled && (combo_box.hovered || combo_box.hovered_item.is_some()) {
                    Some(id.clone())
                } else {
                    None
                }
            });
        }
    }

    fn handle_text_box_scrollbar_input(
        &mut self,
        mouse_pressed: bool,
        mouse_down: bool,
        mouse_released: bool,
        cursor_pos: (f64, f64),
    ) {
        if mouse_pressed {
            let scrollbar_id = self.order.iter().rev().find_map(|id| {
                let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                    return None;
                };
                if text_box.enabled
                    && text_box.overflow_x.allows_scroll()
                    && text_box_max_scroll_x(text_box) > 0.0
                    && text_box_scrollbar_track_contains(text_box, cursor_pos.0, cursor_pos.1)
                {
                    Some(id.clone())
                } else {
                    None
                }
            });
            self.active_text_box_scrollbar = scrollbar_id;
        }

        if mouse_down
            && let Some(id) = self.active_text_box_scrollbar.clone()
            && let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&id)
            && text_box.enabled
            && text_box.overflow_x.allows_scroll()
            && text_box_max_scroll_x(text_box) > 0.0
        {
            set_scroll_from_scrollbar_cursor(text_box, cursor_pos.0);
        }

        if mouse_released {
            self.active_text_box_scrollbar = None;
        }
    }

    fn update_button_states(&mut self, mouse_down: bool, mouse_released: bool) {
        let mut clicked_ids = Vec::new();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::Button(button) = node {
                if !button.enabled {
                    button.pressed = false;
                    button.clicked = false;
                    continue;
                }
                button.pressed = mouse_down
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false);
                if mouse_released
                    && button.hovered
                    && self
                        .active_button
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false)
                {
                    button.clicked = true;
                    clicked_ids.push(id.clone());
                }
            }
        }

        for id in clicked_ids {
            self.push_event(UiEvent::ButtonClicked { id });
        }

        if mouse_released {
            self.active_button = None;
        }
    }

    fn update_combo_box_states(&mut self, mouse_down: bool, mouse_released: bool) {
        let mut changed = Vec::new();
        let active_combo_box = self.active_combo_box.clone();
        for (id, node) in &mut self.widgets {
            if let WidgetNode::ComboBox(combo_box) = node {
                if !combo_box.enabled {
                    combo_box.hovered = false;
                    combo_box.hovered_item = None;
                    combo_box.pressed = false;
                    combo_box.changed = false;
                    combo_box.is_open = false;
                    continue;
                }
                combo_box.pressed = mouse_down
                    && active_combo_box
                        .as_ref()
                        .map(|active| active == id)
                        .unwrap_or(false);

                if !mouse_released {
                    continue;
                }

                if active_combo_box
                    .as_ref()
                    .map(|active| active == id)
                    .unwrap_or(false)
                {
                    if combo_box.is_open {
                        if let Some(index) = combo_box.hovered_item
                            && index < combo_box.items.len()
                        {
                            combo_box.changed = combo_box.selected_index != index;
                            combo_box.selected_index = index;
                            if combo_box.changed {
                                changed.push((
                                    id.clone(),
                                    combo_box.selected_index,
                                    combo_box.items[combo_box.selected_index].clone(),
                                ));
                            }
                            combo_box.is_open = false;
                        } else if combo_box.hovered {
                            combo_box.is_open = false;
                        } else {
                            combo_box.is_open = false;
                        }
                    } else if combo_box.hovered {
                        combo_box.is_open = true;
                    }
                } else {
                    combo_box.is_open = false;
                }
            }
        }

        for (id, selected_index, selected_text) in changed {
            self.push_event(UiEvent::ComboBoxChanged {
                id,
                selected_index,
                selected_text,
            });
        }

        if mouse_released {
            self.active_combo_box = None;
        }
    }

    fn update_text_box_focus(&mut self) {
        for node in self.widgets.values_mut() {
            if let WidgetNode::TextBox(text_box) = node {
                text_box.focused = false;
            }
        }
    }

    fn apply_text_input(&mut self, input: &InputFrame, font: Option<&FontData>) {
        let mut pending_event: Option<UiEvent> = None;
        if let Some(id) = self.focused_text_box.clone()
            && let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&id)
        {
            if text_box.enabled {
                text_box.focused = true;

                if input.backspace_pressed && text_box.text.pop().is_some() {
                    text_box.changed = true;
                }

                if !input.text_input.is_empty() {
                    for ch in input.text_input.chars() {
                        if !ch.is_control() {
                            text_box.text.push(ch);
                            text_box.changed = true;
                        }
                    }
                }

                if text_box.changed {
                    sync_text_box_text_advance(text_box, font);
                    Self::keep_text_box_end_visible(text_box);
                    pending_event = Some(UiEvent::TextBoxChanged {
                        id,
                        text: text_box.text.clone(),
                    });
                }
            }
        }

        if let Some(event) = pending_event {
            self.push_event(event);
        }
    }

    fn apply_text_box_scroll(&mut self, input: &InputFrame) {
        if input.scroll_x == 0.0 && input.scroll_y == 0.0 {
            return;
        }

        let target_id = self.order.iter().rev().find_map(|id| {
            let WidgetNode::TextBox(text_box) = self.widgets.get(id)? else {
                return None;
            };
            if text_box.enabled && (text_box.hovered || text_box.focused) {
                Some(id.clone())
            } else {
                None
            }
        });

        let Some(target_id) = target_id else {
            return;
        };
        let Some(WidgetNode::TextBox(text_box)) = self.widgets.get_mut(&target_id) else {
            return;
        };

        if text_box.overflow_x.allows_scroll() {
            if self.active_text_box_scrollbar.as_deref() != Some(target_id.as_str()) {
                // Prefer real horizontal wheel when dominant; otherwise map vertical wheel
                // to horizontal for single-line textbox scrolling.
                let wheel_x = if input.scroll_x.abs() > input.scroll_y.abs() * 0.5 {
                    -input.scroll_x
                } else {
                    input.scroll_y
                };
                text_box.scroll_x =
                    (text_box.scroll_x + wheel_x).clamp(0.0, Self::max_scroll_x(text_box));
            } else {
                text_box.scroll_x = text_box.scroll_x.clamp(0.0, Self::max_scroll_x(text_box));
            }
        } else {
            text_box.scroll_x = 0.0;
        }

        if text_box.overflow_y.allows_scroll() {
            text_box.scroll_y =
                (text_box.scroll_y - input.scroll_y).clamp(0.0, Self::max_scroll_y(text_box));
        } else {
            text_box.scroll_y = 0.0;
        }
    }

    fn keep_text_box_end_visible(text_box: &mut crate::retained::node::TextBoxNode) {
        // `Scroll` is manual scrolling. Only `Auto` should follow the text end.
        if !matches!(text_box.overflow_x, crate::widget::text_box::Overflow::Auto) {
            text_box.scroll_x = text_box.scroll_x.clamp(0.0, Self::max_scroll_x(text_box));
            return;
        }
        text_box.scroll_x = Self::max_scroll_x(text_box);
    }

    fn max_scroll_x(text_box: &crate::retained::node::TextBoxNode) -> f64 {
        text_box_max_scroll_x(text_box)
    }

    fn max_scroll_y(_text_box: &crate::retained::node::TextBoxNode) -> f64 {
        0.0
    }
}

fn contains(rect: Rect, x: f64, y: f64) -> bool {
    x >= rect.x0 && x <= rect.x1 && y >= rect.y0 && y <= rect.y1
}

fn combo_item_index_at(
    combo_box: &crate::retained::node::ComboBoxNode,
    x: f64,
    y: f64,
) -> Option<usize> {
    if !combo_box.is_open || combo_box.items.is_empty() {
        return None;
    }
    let item_height = combo_box.rect.height();
    for index in 0..combo_box.items.len() {
        let top = combo_box.rect.y1 + item_height * index as f64;
        let rect = Rect::new(combo_box.rect.x0, top, combo_box.rect.x1, top + item_height);
        if contains(rect, x, y) {
            return Some(index);
        }
    }
    None
}

fn combo_expanded_contains(
    combo_box: &crate::retained::node::ComboBoxNode,
    x: f64,
    y: f64,
) -> bool {
    if contains(combo_box.rect, x, y) {
        return true;
    }
    combo_item_index_at(combo_box, x, y).is_some()
}

fn text_box_max_scroll_x(text_box: &crate::retained::node::TextBoxNode) -> f64 {
    let inner_width = (text_box.rect.width() - 24.0).max(1.0);
    let content_width = text_box_content_width(text_box);
    (content_width - inner_width).max(0.0)
}

fn text_box_scrollbar_track_contains(
    text_box: &crate::retained::node::TextBoxNode,
    x: f64,
    y: f64,
) -> bool {
    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;
    // Keep the visual bar thin, but use a wider hit area for usability.
    let hit_height = 12.0;
    let hit_bottom = text_box.rect.y1 - 2.0;
    let hit_top = (hit_bottom - hit_height).max(text_box.rect.y0);
    let hit = Rect::new(inner_left, hit_top, inner_right, hit_bottom);
    contains(hit, x, y)
}

fn set_scroll_from_scrollbar_cursor(
    text_box: &mut crate::retained::node::TextBoxNode,
    cursor_x: f64,
) {
    let inner_left = text_box.rect.x0 + 12.0;
    let inner_right = text_box.rect.x1 - 12.0;
    let inner_width = (inner_right - inner_left).max(1.0);
    let content_width = text_box_content_width(text_box);
    let max_scroll = text_box_max_scroll_x(text_box);
    if max_scroll <= 0.0 {
        text_box.scroll_x = 0.0;
        return;
    }

    let thumb_w = ((inner_width / content_width) * inner_width)
        .clamp(18.0, inner_width)
        .min(inner_width);
    let den = (inner_width - thumb_w).max(1.0);
    let ratio = ((cursor_x - inner_left - thumb_w * 0.5) / den).clamp(0.0, 1.0);
    text_box.scroll_x = ratio * max_scroll;
}

fn text_box_content_width(text_box: &crate::retained::node::TextBoxNode) -> f64 {
    if text_box.text_advance > 0.0 {
        text_box.text_advance
    } else {
        estimate_text_width(&text_box.text, text_box.font_size) as f64
    }
}

fn sync_text_box_text_advance(
    text_box: &mut crate::retained::node::TextBoxNode,
    font: Option<&FontData>,
) {
    if let Some(font) = font
        && let Some((_, advance)) = layout_text(font, &text_box.text, text_box.font_size)
    {
        text_box.text_advance = advance as f64;
        return;
    }
    text_box.text_advance = estimate_text_width(&text_box.text, text_box.font_size) as f64;
}
