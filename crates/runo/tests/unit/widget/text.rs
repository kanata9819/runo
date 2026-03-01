use std::sync::Arc;

use vello::Scene;
use vello::peniko::{Blob, FontData};

use super::*;
use crate::font::load_default_font;

#[test]
fn estimate_text_width_scales_with_chars_and_font_size() {
    let short = estimate_text_width("ab", 10.0);
    let longer = estimate_text_width("abcd", 10.0);
    let bigger_font = estimate_text_width("ab", 20.0);
    assert!(longer > short);
    assert!(bigger_font > short);
}

#[test]
fn layout_text_returns_none_for_invalid_font_bytes() {
    let bytes = vec![0_u8; 8];
    let font = FontData::new(Blob::new(Arc::new(bytes.into_boxed_slice())), 0);
    let layout = layout_text(&font, "hello", 16.0);
    assert!(layout.is_none());
}

#[test]
fn draw_text_run_is_callable_with_empty_glyphs() {
    let Some(font) = load_default_font() else {
        return;
    };
    let mut scene = Scene::new();
    draw_text_run(
        &mut scene,
        &font,
        Vec::new(),
        10.0,
        20.0,
        14.0,
        Color::from_rgb8(255, 255, 255),
    );
}

#[test]
fn layout_text_with_real_font_returns_some_for_ascii() {
    let Some(font) = load_default_font() else {
        return;
    };
    let result = layout_text(&font, "abc", 16.0);
    assert!(result.is_some());
}
