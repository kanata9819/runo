mod button;
mod checkbox;
mod combo_box;
mod core;
mod radio_button;
mod slider;
mod text_box;

use std::collections::{HashMap, HashSet, VecDeque};

use vello::peniko::Color;

use crate::event::UiEvent;
use crate::retained::node::WidgetNode;

pub(crate) use checkbox::UpsertCheckboxArgs;
pub(crate) use combo_box::UpsertComboBoxArgs;
pub(crate) use radio_button::UpsertRadioButtonArgs;
pub(crate) use slider::UpsertSliderArgs;
pub(crate) use text_box::UpsertTextBoxArgs;

#[cfg(test)]
#[path = "../../../tests/unit/retained/state/mod.rs"]
mod tests;

pub(crate) struct RetainedState {
    pub(super) widgets: HashMap<String, WidgetNode>,
    pub(super) order: Vec<String>,
    pub(super) seen_widget_ids: HashSet<String>,
    pub(super) active_button: Option<String>,
    pub(super) active_checkbox: Option<String>,
    pub(super) active_radio_button: Option<String>,
    pub(super) active_slider: Option<String>,
    pub(super) active_combo_box: Option<String>,
    pub(super) active_text_box_scrollbar: Option<String>,
    pub(super) focused_text_box: Option<String>,
    pub(super) events: VecDeque<UiEvent>,
    pub(super) text_clipboard: String,
    pub(super) div_visible: HashMap<String, bool>,
    pub(super) div_enabled: HashMap<String, bool>,
    pub(super) div_background: HashMap<String, Color>,
}

fn normalize_range(min: f64, max: f64) -> (f64, f64) {
    if min <= max { (min, max) } else { (max, min) }
}

fn snap_and_clamp(value: f64, min: f64, max: f64, step: Option<f64>) -> f64 {
    let mut clamped = value.clamp(min, max);

    if let Some(step) = step
        && step > 0.0
    {
        let snapped = ((clamped - min) / step).round() * step + min;
        clamped = snapped.clamp(min, max);
    }

    clamped
}

impl RetainedState {
    pub(super) fn upsert_widget_node<R, FNode, FUpdate, FNewResponse>(
        &mut self,
        id: String,
        mut make_new_node: FNode,
        update_existing: FUpdate,
        new_or_replaced_response: FNewResponse,
    ) -> R
    where
        FNode: FnMut() -> WidgetNode,
        FUpdate: FnOnce(&mut WidgetNode) -> Option<R>,
        FNewResponse: FnOnce(&WidgetNode) -> R,
    {
        self.seen_widget_ids.insert(id.clone());

        if !self.widgets.contains_key(&id) {
            self.order.push(id.clone());
            self.widgets.insert(id.clone(), make_new_node());
            let node = self
                .widgets
                .get(&id)
                .expect("newly inserted widget entry missing");

            return new_or_replaced_response(node);
        }

        let entry = self.widgets.get_mut(&id).expect("widget entry missing");

        if let Some(response) = update_existing(entry) {
            return response;
        }

        *entry = make_new_node();
        new_or_replaced_response(entry)
    }
}
