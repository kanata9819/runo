use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::retained::node::{TerminalViewNode, WidgetNode};
use crate::retained::state::RetainedState;
use crate::widget::terminal_view::{TerminalBuffer, TerminalViewResponse};

#[cfg(test)]
#[path = "../../../tests/unit/retained/state/terminal_view.rs"]
mod tests;

pub(crate) struct UpsertTerminalViewArgs {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) buffer: TerminalBuffer,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) disable_border: bool,
    pub(crate) enabled: bool,
}

impl RetainedState {
    pub(crate) fn upsert_terminal_view(
        &mut self,
        args: UpsertTerminalViewArgs,
    ) -> TerminalViewResponse {
        let UpsertTerminalViewArgs {
            id,
            rect,
            buffer,
            font_size,
            text_color,
            bg_color,
            border_color,
            disable_border,
            enabled,
        } = args;
        let initial_buffer: TerminalBuffer = buffer.clone();
        let update_buffer: TerminalBuffer = buffer;

        self.upsert_widget_node(
            id,
            || {
                let mut node = TerminalViewNode {
                    rect,
                    buffer: initial_buffer.clone(),
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    disable_border,
                    enabled,
                    scroll_y: 0.0,
                    hovered: false,
                };
                node.scroll_y = max_scroll_y(&node);
                WidgetNode::TerminalView(node)
            },
            |entry| match entry {
                WidgetNode::TerminalView(terminal_view) => {
                    let previous_max_scroll: f64 = max_scroll_y(terminal_view);
                    let was_at_bottom: bool =
                        (terminal_view.scroll_y - previous_max_scroll).abs() <= 1.0;

                    terminal_view.rect = rect;
                    terminal_view.buffer = update_buffer.clone();
                    terminal_view.font_size = font_size;
                    terminal_view.text_color = text_color;
                    terminal_view.bg_color = bg_color;
                    terminal_view.border_color = border_color;
                    terminal_view.disable_border = disable_border;
                    terminal_view.enabled = enabled;

                    let next_max_scroll: f64 = max_scroll_y(terminal_view);
                    if was_at_bottom {
                        terminal_view.scroll_y = next_max_scroll;
                    } else {
                        terminal_view.scroll_y = terminal_view.scroll_y.clamp(0.0, next_max_scroll);
                    }

                    Some(TerminalViewResponse {
                        hovered: terminal_view.hovered,
                    })
                }
                _ => None,
            },
            |node| match node {
                WidgetNode::TerminalView(terminal_view) => TerminalViewResponse {
                    hovered: terminal_view.hovered,
                },
                _ => TerminalViewResponse::default(),
            },
        )
    }

    pub(crate) fn terminal_view_response(&self, id: impl AsRef<str>) -> TerminalViewResponse {
        let Some(WidgetNode::TerminalView(terminal_view)) = self.widgets.get(id.as_ref()) else {
            return TerminalViewResponse::default();
        };

        TerminalViewResponse {
            hovered: terminal_view.hovered,
        }
    }

    pub(crate) fn set_terminal_view_buffer(&mut self, id: impl AsRef<str>, buffer: TerminalBuffer) {
        let Some(WidgetNode::TerminalView(terminal_view)) = self.widgets.get_mut(id.as_ref())
        else {
            return;
        };

        let previous_max_scroll: f64 = max_scroll_y(terminal_view);
        let was_at_bottom: bool = (terminal_view.scroll_y - previous_max_scroll).abs() <= 1.0;

        terminal_view.buffer = buffer;

        let next_max_scroll: f64 = max_scroll_y(terminal_view);
        if was_at_bottom {
            terminal_view.scroll_y = next_max_scroll;
        } else {
            terminal_view.scroll_y = terminal_view.scroll_y.clamp(0.0, next_max_scroll);
        }
    }

    pub(crate) fn set_terminal_view_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let Some(WidgetNode::TerminalView(terminal_view)) = self.widgets.get_mut(id.as_ref())
        else {
            return;
        };

        terminal_view.enabled = enabled;
        if !enabled {
            terminal_view.hovered = false;
        }
    }

    pub(crate) fn apply_terminal_view_scroll(&mut self, input: &crate::input::InputFrame) {
        if input.scroll_y == 0.0 {
            return;
        }

        let Some(target_id) = self.order.iter().rev().find_map(|id| {
            let WidgetNode::TerminalView(terminal_view) = self.widgets.get(id)? else {
                return None;
            };

            if terminal_view.enabled && terminal_view.hovered {
                Some(id.clone())
            } else {
                None
            }
        }) else {
            return;
        };

        let Some(WidgetNode::TerminalView(terminal_view)) = self.widgets.get_mut(&target_id) else {
            return;
        };

        let max_scroll: f64 = max_scroll_y(terminal_view);
        terminal_view.scroll_y = (terminal_view.scroll_y
            - input.scroll_y * line_height(terminal_view) * 0.5)
            .clamp(0.0, max_scroll);
    }
}

fn line_height(terminal_view: &TerminalViewNode) -> f64 {
    f64::from(terminal_view.font_size) * 1.35
}

fn max_scroll_y(terminal_view: &TerminalViewNode) -> f64 {
    let line_count: f64 = terminal_view.buffer.lines.len().max(1) as f64;
    let content_height: f64 = line_count * line_height(terminal_view) + 12.0;
    let inner_height: f64 = (terminal_view.rect.height() - 12.0).max(1.0);
    (content_height - inner_height).max(0.0)
}
