use vello::kurbo::Rect;
use vello::peniko::Color;

pub(super) enum WidgetNode {
    Button(ButtonNode),
    Label(LabelNode),
}

pub(super) struct ButtonNode {
    pub(super) rect: Rect,
    pub(super) text: Option<String>,
    pub(super) text_color: Color,
    pub(super) hovered: bool,
    pub(super) pressed: bool,
    pub(super) clicked: bool,
}

pub(super) struct LabelNode {
    pub(super) rect: Rect,
    pub(super) text: String,
    pub(super) font_size: f32,
    pub(super) text_color: Color,
}
