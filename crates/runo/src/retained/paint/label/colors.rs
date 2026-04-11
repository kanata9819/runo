use crate::retained::node::LabelNode;
use crate::theme::color;

pub(super) fn text_color(label: &LabelNode) -> vello::peniko::Color {
    if label.enabled {
        label.text_color
    } else {
        color::Neutral::tone_142_148_156()
    }
}
