use skrifa::instance::{LocationRef, Size};
use skrifa::{FontRef, MetadataProvider};
use vello::Glyph;
use vello::Scene;
use vello::kurbo::{Affine, Vec2};
use vello::peniko::{Color, Fill, FontData};

use crate::cache::text_layout;

pub(crate) fn layout_text(
    font: &FontData,
    text: &str,
    font_size: f32,
) -> Option<(Vec<Glyph>, f32)> {
    text_layout::get_or_insert_layout(font, text, font_size, || {
        let Ok(font_ref) = FontRef::from_index(font.data.as_ref(), font.index) else {
            return None;
        };
        let charmap = font_ref.charmap();
        let glyph_metrics = font_ref.glyph_metrics(Size::new(font_size), LocationRef::default());

        let mut total_advance = 0.0_f32;
        let mut glyphs = Vec::new();

        for ch in text.chars() {
            let Some(glyph_id) = charmap.map(ch) else {
                continue;
            };
            let advance = glyph_metrics
                .advance_width(glyph_id)
                .unwrap_or(font_size * 0.56);
            glyphs.push(Glyph {
                id: glyph_id.to_u32(),
                x: total_advance,
                y: 0.0,
            });
            total_advance += advance;
        }

        Some((glyphs, total_advance))
    })
}

pub(crate) fn draw_text_run(
    scene: &mut Scene,
    font: &FontData,
    glyphs: Vec<Glyph>,
    x: f64,
    baseline_y: f64,
    font_size: f32,
    color: Color,
) {
    scene
        .draw_glyphs(font)
        .font_size(font_size)
        .transform(Affine::translate(Vec2::new(x, baseline_y)))
        .brush(color)
        .draw(Fill::NonZero, glyphs.into_iter());
}

pub(crate) fn estimate_text_width(text: &str, font_size: f32) -> f32 {
    text.chars().count() as f32 * font_size * 0.56
}

#[cfg(test)]
#[path = "../../tests/unit/widget/text.rs"]
mod tests;
