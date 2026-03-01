//! Paint phase for retained widgets.
//!
//! Rendering is split into two passes:
//! 1. base widgets in layout order
//! 2. overlay elements that must stay on top (e.g. combo-box dropdowns)
mod button;
mod checkbox;
mod combo_box;
mod div;
mod interaction_color;
mod label;
mod radio_button;
mod slider;
mod text_baseline;
mod text_box;

use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

#[cfg(test)]
#[path = "../../../tests/unit/retained/paint/mod.rs"]
mod tests;

impl RetainedState {
    /// Renders base widgets first, then paints overlay layers that must appear on top.
    pub(crate) fn render(&mut self, scene: &mut Scene, font: Option<&FontData>) {
        for id in &self.order {
            let Some(node) = self.widgets.get_mut(id) else {
                continue;
            };

            match node {
                WidgetNode::Div(div_node) => div::render(scene, div_node),
                WidgetNode::Button(button) => button::render(scene, font, button),
                WidgetNode::Checkbox(checkbox) => checkbox::render(scene, font, checkbox),
                WidgetNode::RadioButton(radio_button) => {
                    radio_button::render(scene, font, radio_button)
                }
                WidgetNode::Slider(slider) => slider::render(scene, font, slider),
                WidgetNode::Label(label) => label::render(scene, font, label),
                WidgetNode::TextBox(text_box) => text_box::render(scene, font, text_box),
                WidgetNode::ComboBox(combo_box) => combo_box::render(scene, font, combo_box),
            }
        }

        for id in &self.order {
            let Some(node) = self.widgets.get_mut(id) else {
                continue;
            };
            match node {
                WidgetNode::ComboBox(combo_box) => {
                    combo_box::render_dropdown_overlay(scene, font, combo_box)
                }
                WidgetNode::Div(_)
                | WidgetNode::Button(_)
                | WidgetNode::Checkbox(_)
                | WidgetNode::RadioButton(_)
                | WidgetNode::Slider(_)
                | WidgetNode::Label(_)
                | WidgetNode::TextBox(_) => {}
            }
        }
    }
}
