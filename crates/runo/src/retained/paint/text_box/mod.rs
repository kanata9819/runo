//! Core painter for `TextBox`.
//!
//! This module renders background, border, text/placeholder, caret,
//! and horizontal scrollbar with separated responsibilities.
mod caret;
mod metrics;
mod scrollbar;
mod text_content;

use crate::retained::node::TextBoxNode;
use crate::theme::color;
use caret::Caret;
use metrics::TextMetrics;
use scrollbar::ScrollBar;
use text_content::TextContentPainter;
use vello::Scene;
use vello::kurbo::{Affine, RoundedRect};
use vello::peniko::color::{self as vello_color, AlphaColor};
use vello::peniko::{Color, Fill, FontData};

#[cfg(test)]
#[path = "../../../../tests/unit/retained/paint/text_box.rs"]
mod tests;

struct CorePainter;

/// Entry point for rendering the full `TextBox`.
///
/// Draws background and border first, then renders text/placeholder,
/// caret, and horizontal scrollbar only when a font is available.
pub(crate) fn render(scene: &mut Scene, font: Option<&FontData>, text_box: &mut TextBoxNode) {
    CorePainter::draw_background_and_border(scene, text_box);

    let Some(font) = font else {
        return;
    };

    let metrics: TextMetrics = TextMetrics::new(text_box);
    let text_color: AlphaColor<vello_color::Srgb> = CorePainter::resolve_text_color(text_box);
    TextContentPainter::draw_text_content(scene, font, text_box, text_color, metrics);

    let mut caret: Caret = Caret::new(scene, font, text_box, metrics);
    caret.draw_caret();

    let mut scrollbar: ScrollBar = ScrollBar::new(scene, text_box);
    scrollbar.render_horizontal_scrollbar();
}

impl CorePainter {
    /// Draws `TextBox` background fill and border stroke.
    ///
    /// Colors are chosen based on `enabled`, `focused`, and `read_only` states.
    fn draw_background_and_border(scene: &mut Scene, text_box: &TextBoxNode) {
        let bg: RoundedRect = RoundedRect::from_rect(text_box.rect, TextMetrics::BOX_CORNER_RADIUS);

        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            if text_box.enabled {
                text_box.bg_color
            } else {
                color::Neutral::tone_45_49_55()
            },
            None,
            &bg,
        );

        let border_color: Color = if !text_box.enabled {
            color::Neutral::tone_86_92_101()
        } else if text_box.focused && !text_box.read_only {
            color::AccentBlue::tone_89_176_255()
        } else {
            text_box.border_color
        };

        scene.stroke(
            &vello::kurbo::Stroke::new(TextMetrics::BORDER_STROKE_WIDTH),
            Affine::IDENTITY,
            border_color,
            None,
            &bg,
        );
    }

    /// Resolves text color by state.
    ///
    /// Returns different colors for disabled, placeholder, and normal text.
    fn resolve_text_color(text_box: &TextBoxNode) -> Color {
        if !text_box.enabled {
            color::Neutral::tone_147_153_161()
        } else if text_box.text.is_empty() {
            color::Neutral::tone_142_151_163()
        } else {
            text_box.text_color
        }
    }
}
