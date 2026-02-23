mod button;
mod checkbox;
mod combo_box;
mod interaction_color;
mod label;
mod radio_button;
mod slider;
mod text_box;

use vello::Scene;
use vello::peniko::FontData;

use crate::retained::node::WidgetNode;
use crate::retained::state::RetainedState;

impl RetainedState {
    /// Renders base widgets first, then paints overlay layers that must appear on top.
    pub(crate) fn render(&mut self, scene: &mut Scene, font: Option<&FontData>) {
        for id in &self.order {
            let Some(node) = self.widgets.get_mut(id) else {
                continue;
            };
            match node {
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
                WidgetNode::Button(_)
                | WidgetNode::Checkbox(_)
                | WidgetNode::RadioButton(_)
                | WidgetNode::Slider(_)
                | WidgetNode::Label(_)
                | WidgetNode::TextBox(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use vello::Scene;
    use vello::kurbo::Rect;
    use vello::peniko::Color;

    use super::*;
    use crate::font::load_default_font;
    use crate::retained::{
        UpsertCheckboxArgs, UpsertRadioButtonArgs, UpsertSliderArgs, UpsertTextBoxArgs,
    };
    use crate::widget::text_box::Overflow;

    #[test]
    fn render_with_empty_state_is_noop() {
        let mut state = RetainedState::new();
        let mut scene = Scene::new();
        state.render(&mut scene, None);
    }

    #[test]
    fn render_visits_base_and_overlay_passes() {
        let mut state = RetainedState::new();
        let mut scene = Scene::new();
        let rect = Rect::new(0.0, 0.0, 140.0, 40.0);
        let color = Color::from_rgb8(240, 240, 240);

        state.upsert_button(
            "btn".to_string(),
            rect,
            Some("btn".to_string()),
            16.0,
            color,
            true,
        );
        state.upsert_checkbox(UpsertCheckboxArgs {
            id: "cb".to_string(),
            rect,
            text: Some("cb".to_string()),
            checked: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_radio_button(UpsertRadioButtonArgs {
            id: "rb".to_string(),
            group: "g".to_string(),
            rect,
            text: Some("rb".to_string()),
            selected: Some(true),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_slider(UpsertSliderArgs {
            id: "sl".to_string(),
            rect,
            min: 0.0,
            max: 1.0,
            value: Some(0.5),
            step: Some(0.1),
            text: Some("sl".to_string()),
            font_size: 16.0,
            text_color: color,
            enabled: true,
        });
        state.upsert_label(
            "lbl".to_string(),
            rect,
            "label".to_string(),
            16.0,
            color,
            true,
        );
        state.upsert_text_box(UpsertTextBoxArgs {
            id: "tb".to_string(),
            rect,
            text: Some("text".to_string()),
            placeholder: Some("p".to_string()),
            font_size: 16.0,
            text_color: color,
            bg_color: Color::from_rgb8(30, 30, 30),
            border_color: color,
            enabled: true,
            overflow_x: Overflow::Auto,
            overflow_y: Overflow::Auto,
        });
        state.upsert_combo_box(crate::retained::UpsertComboBoxArgs {
            id: "combo".to_string(),
            rect,
            items: vec!["a".to_string(), "b".to_string()],
            selected_index: Some(0),
            font_size: 16.0,
            text_color: color,
            bg_color: Color::from_rgb8(30, 30, 30),
            border_color: color,
            enabled: true,
        });

        state.render(&mut scene, None);
        if let Some(font) = load_default_font() {
            state.render(&mut scene, Some(&font));
        }
    }
}
