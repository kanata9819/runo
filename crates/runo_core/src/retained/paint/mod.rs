mod button;
mod combo_box;
mod label;
mod text_box;

use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

impl RetainedState {
    pub(crate) fn render(&mut self, scene: &mut Scene, font: Option<&FontData>) {
        for id in &self.order {
            let Some(node) = self.widgets.get_mut(id) else {
                continue;
            };
            match node {
                WidgetNode::Button(button) => button::render(scene, font, button),
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
                    combo_box::render_overlay(scene, font, combo_box)
                }
                WidgetNode::Button(_) | WidgetNode::Label(_) | WidgetNode::TextBox(_) => {}
            }
        }
    }
}
