use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::widget::text_box::Overflow;

pub(super) enum WidgetNode {
    Button(ButtonNode),
    Checkbox(CheckboxNode),
    Label(LabelNode),
    TextBox(TextBoxNode),
    ComboBox(ComboBoxNode),
}

pub(super) struct ButtonNode {
    pub(super) rect: Rect,
    pub(super) text: Option<String>,
    pub(super) text_overridden: bool,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
    pub(super) enabled: bool,
    pub(super) hovered: bool,
    pub(super) pressed: bool,
    pub(super) clicked: bool,
}

pub(super) struct LabelNode {
    pub(super) rect: Rect,
    pub(super) text: String,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
    pub(super) enabled: bool,
}

pub(super) struct CheckboxNode {
    pub(super) rect: Rect,
    pub(super) text: Option<String>,
    pub(super) checked: bool,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
    pub(super) enabled: bool,
    pub(super) hovered: bool,
    pub(super) pressed: bool,
    pub(super) changed: bool,
}

pub(super) struct TextBoxNode {
    pub(super) rect: Rect,
    pub(super) text: String,
    pub(super) placeholder: Option<String>,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
    pub(super) bg_color: Color,
    pub(super) border_color: Color,
    pub(super) enabled: bool,
    pub(super) overflow_x: Overflow,
    pub(super) overflow_y: Overflow,
    pub(super) text_advance: f64,
    pub(super) caret_index: usize,
    pub(super) scroll_x: f64,
    pub(super) scroll_y: f64,
    pub(super) hovered: bool,
    pub(super) focused: bool,
    pub(super) changed: bool,
}

pub(super) struct ComboBoxNode {
    pub(super) rect: Rect,
    pub(super) items: Vec<String>,
    pub(super) selected_index: usize,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
    pub(super) bg_color: Color,
    pub(super) border_color: Color,
    pub(super) enabled: bool,
    pub(super) hovered: bool,
    pub(super) hovered_item: Option<usize>,
    pub(super) pressed: bool,
    pub(super) changed: bool,
    pub(super) is_open: bool,
}
