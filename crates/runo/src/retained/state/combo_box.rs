use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::ComboBoxResponse;
use crate::retained::node::{ComboBoxNode, WidgetNode};
use crate::retained::state::RetainedState;

pub(crate) struct UpsertComboBoxArgs {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) items: Vec<String>,
    pub(crate) selected_index: Option<usize>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) bg_color: Color,
    pub(crate) border_color: Color,
    pub(crate) enabled: bool,
}

impl RetainedState {
    pub(crate) fn upsert_combo_box(&mut self, args: UpsertComboBoxArgs) -> ComboBoxResponse {
        let UpsertComboBoxArgs {
            id,
            rect,
            items,
            selected_index,
            font_size,
            text_color,
            bg_color,
            border_color,
            enabled,
        } = args;

        let selected_index_override = selected_index;
        let items_for_update = items.clone();

        self.upsert_widget_node(
            id,
            || {
                let items = items.clone();
                let initial_selected_index =
                    initial_selected_index(&items.clone(), selected_index_override);

                let args: BuildNodeParams = BuildNodeParams {
                    rect,
                    items,
                    selected_index: initial_selected_index,
                    font_size,
                    text_color,
                    bg_color,
                    border_color,
                    enabled,
                };

                WidgetNode::ComboBox(build_combo_box_node(args))
            },
            |entry| match entry {
                WidgetNode::ComboBox(combo_box) => {
                    let initial_selected_index =
                        initial_selected_index(&items_for_update, selected_index_override);
                    let args: BuildNodeParams = BuildNodeParams {
                        rect,
                        items: items_for_update,
                        selected_index: initial_selected_index,
                        font_size,
                        text_color,
                        bg_color,
                        border_color,
                        enabled,
                    };

                    let replacement = build_combo_box_node(args);
                    update_existing_combo_box(combo_box, replacement, selected_index_override);
                    Some(combo_box_response(combo_box))
                }
                _ => None,
            },
            |node| match node {
                WidgetNode::ComboBox(combo_box) => combo_box_response(combo_box),
                _ => ComboBoxResponse::default(),
            },
        )
    }

    pub(crate) fn combo_box_response(&self, id: impl AsRef<str>) -> ComboBoxResponse {
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get(id.as_ref()) else {
            return ComboBoxResponse::default();
        };

        ComboBoxResponse {
            selected_index: combo_box.selected_index,
            selected_text: combo_box
                .items
                .get(combo_box.selected_index)
                .cloned()
                .unwrap_or_default(),
            hovered: combo_box.hovered,
            pressed: combo_box.pressed,
            changed: combo_box.changed,
            is_open: combo_box.is_open,
        }
    }

    pub(crate) fn set_combo_box_selected_index(&mut self, id: impl AsRef<str>, index: usize) {
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        if combo_box.items.is_empty() {
            combo_box.selected_index = 0;
            combo_box.changed = false;
            return;
        }

        let next_index = index.min(combo_box.items.len() - 1);
        combo_box.changed = combo_box.selected_index != next_index;
        combo_box.selected_index = next_index;
    }

    pub(crate) fn set_combo_box_items<I, T>(&mut self, id: impl AsRef<str>, items: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        let next_items: Vec<String> = items.into_iter().map(Into::into).collect();
        let prev_index = combo_box.selected_index;
        combo_box.items = next_items;

        if combo_box.items.is_empty() {
            combo_box.selected_index = 0;
            combo_box.hovered_item = None;
            combo_box.is_open = false;
            combo_box.changed = false;
            return;
        }

        combo_box.selected_index = prev_index.min(combo_box.items.len() - 1);

        if combo_box
            .hovered_item
            .is_some_and(|idx| idx >= combo_box.items.len())
        {
            combo_box.hovered_item = None;
        }

        combo_box.changed = combo_box.selected_index != prev_index;
    }

    pub(crate) fn set_combo_box_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::ComboBox(combo_box)) = self.widgets.get_mut(id_ref) else {
            return;
        };

        combo_box.enabled = enabled;

        if !enabled {
            combo_box.hovered = false;
            combo_box.hovered_item = None;
            combo_box.pressed = false;
            combo_box.changed = false;
            combo_box.is_open = false;

            if self.active_combo_box.as_deref() == Some(id_ref) {
                self.active_combo_box = None;
            }
        }
    }
}

fn initial_selected_index(items: &[String], selected_index: Option<usize>) -> usize {
    if items.is_empty() {
        0
    } else {
        selected_index.unwrap_or(0).min(items.len() - 1)
    }
}

struct BuildNodeParams {
    rect: Rect,
    items: Vec<String>,
    selected_index: usize,
    font_size: f32,
    text_color: Color,
    bg_color: Color,
    border_color: Color,
    enabled: bool,
}

fn build_combo_box_node(params: BuildNodeParams) -> ComboBoxNode {
    let BuildNodeParams {
        rect,
        items,
        selected_index,
        font_size,
        text_color,
        bg_color,
        border_color,
        enabled,
    } = params;

    ComboBoxNode {
        rect,
        items,
        selected_index,
        font_size,
        text_color,
        bg_color,
        border_color,
        enabled,
        hovered: false,
        hovered_item: None,
        pressed: false,
        changed: false,
        is_open: false,
    }
}

fn update_existing_combo_box(
    combo_box: &mut ComboBoxNode,
    replacement: ComboBoxNode,
    selected_index_override: Option<usize>,
) {
    combo_box.rect = replacement.rect;
    combo_box.items = replacement.items;

    if combo_box.items.is_empty() {
        combo_box.selected_index = 0;
    } else if let Some(next_index) = selected_index_override {
        combo_box.selected_index = next_index.min(combo_box.items.len() - 1);
    } else if combo_box.selected_index >= combo_box.items.len() {
        combo_box.selected_index = combo_box.items.len() - 1;
    }

    combo_box.font_size = replacement.font_size;
    combo_box.text_color = replacement.text_color;
    combo_box.bg_color = replacement.bg_color;
    combo_box.border_color = replacement.border_color;
    combo_box.enabled = replacement.enabled;
}

fn combo_box_response(combo_box: &ComboBoxNode) -> ComboBoxResponse {
    ComboBoxResponse {
        selected_index: combo_box.selected_index,
        selected_text: combo_box
            .items
            .get(combo_box.selected_index)
            .cloned()
            .unwrap_or_default(),
        hovered: combo_box.hovered,
        pressed: combo_box.pressed,
        changed: combo_box.changed,
        is_open: combo_box.is_open,
    }
}

#[cfg(test)]
mod tests {
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
}
