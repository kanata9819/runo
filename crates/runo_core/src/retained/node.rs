use vello::kurbo::Rect;
use vello::peniko::Color;

pub(super) enum WidgetNode {
    Button(ButtonNode),
    Label(LabelNode),
    TextBox(TextBoxNode),
    ComboBox(ComboBoxNode),
}

pub(super) struct ButtonNode {
    pub(super) rect: Rect,
    pub(super) text: Option<String>,
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

pub(super) struct TextBoxNode {
    pub(super) rect: Rect,
    pub(super) text: String,
    pub(super) placeholder: Option<String>,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
    pub(super) bg_color: Color,
    pub(super) border_color: Color,
    pub(super) enabled: bool,
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
