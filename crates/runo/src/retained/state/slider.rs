use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::SliderResponse;
use crate::retained::node::{SliderNode, WidgetNode};
use crate::retained::state::{RetainedState, normalize_range, snap_and_clamp};

pub(crate) struct UpsertSliderArgs {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) value: Option<f64>,
    pub(crate) step: Option<f64>,
    pub(crate) text: Option<String>,
    pub(crate) font_size: f32,
    pub(crate) text_color: Color,
    pub(crate) enabled: bool,
}

impl RetainedState {
    pub(crate) fn upsert_slider(&mut self, args: UpsertSliderArgs) -> SliderResponse {
        let UpsertSliderArgs {
            id,
            rect,
            min,
            max,
            value,
            step,
            text,
            font_size,
            text_color,
            enabled,
        } = args;

        let (min, max) = normalize_range(min, max);
        let default_value = snap_and_clamp(value.unwrap_or(min), min, max, step);
        let text_for_update = text.clone();

        self.upsert_widget_node(
            id,
            || {
                WidgetNode::Slider(SliderNode {
                    rect,
                    min,
                    max,
                    value: default_value,
                    step,
                    text: text.clone(),
                    font_size,
                    text_color,
                    enabled,
                    hovered: false,
                    pressed: false,
                    changed: false,
                })
            },
            |entry| match entry {
                WidgetNode::Slider(slider) => {
                    slider.rect = rect;
                    slider.min = min;
                    slider.max = max;
                    slider.step = step;
                    slider.text = text_for_update;
                    slider.font_size = font_size;
                    slider.text_color = text_color;
                    slider.enabled = enabled;
                    slider.value =
                        snap_and_clamp(slider.value, slider.min, slider.max, slider.step);
                    let _ = value;

                    Some(SliderResponse {
                        value: slider.value,
                        hovered: slider.hovered,
                        pressed: slider.pressed,
                        changed: slider.changed,
                    })
                }
                _ => None,
            },
            |_node| SliderResponse {
                value: default_value,
                hovered: false,
                pressed: false,
                changed: false,
            },
        )
    }

    pub(crate) fn slider_response(&self, id: impl AsRef<str>) -> SliderResponse {
        let Some(WidgetNode::Slider(slider)) = self.widgets.get(id.as_ref()) else {
            return SliderResponse::default();
        };

        SliderResponse {
            value: slider.value,
            hovered: slider.hovered,
            pressed: slider.pressed,
            changed: slider.changed,
        }
    }

    pub(crate) fn set_slider_value(&mut self, id: impl AsRef<str>, value: f64) {
        let Some(WidgetNode::Slider(slider)) = self.widgets.get_mut(id.as_ref()) else {
            return;
        };

        let next = snap_and_clamp(value, slider.min, slider.max, slider.step);
        slider.changed = (slider.value - next).abs() > f64::EPSILON;
        slider.value = next;
    }

    pub(crate) fn set_slider_enabled(&mut self, id: impl AsRef<str>, enabled: bool) {
        let id_ref = id.as_ref();
        let Some(WidgetNode::Slider(slider)) = self.widgets.get_mut(id_ref) else {
            return;
        };

        slider.enabled = enabled;

        if !enabled {
            slider.hovered = false;
            slider.pressed = false;
            slider.changed = false;

            if self.active_slider.as_deref() == Some(id_ref) {
                self.active_slider = None;
            }
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/retained/state/slider.rs"]
mod tests;
