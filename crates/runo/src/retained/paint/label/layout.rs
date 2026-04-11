use vello::Glyph;
use vello::peniko::FontData;

use crate::retained::node::LabelNode;
use crate::widget::text;

pub(super) fn label_glyphs<'a>(
    font: Option<&'a FontData>,
    label: &LabelNode,
) -> Option<(&'a FontData, Vec<Glyph>)> {
    let font: &'a FontData = font?;
    let (glyphs, _) = text::layout_text(font, &label.text, label.font_size)?;
    Some((font, glyphs))
}

pub(super) fn baseline_y(label: &LabelNode) -> f64 {
    super::super::text_baseline::top_aligned(label.rect, label.font_size)
}
