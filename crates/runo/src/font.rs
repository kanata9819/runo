use std::fs;
use std::sync::Arc;

use vello::peniko::{Blob, FontData};

#[cfg(test)]
#[path = "../tests/unit/font.rs"]
mod tests;

pub(crate) fn load_default_font() -> Option<FontData> {
    const CANDIDATES: &[&str] = &[
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\arial.ttf",
        "/mnt/c/Windows/Fonts/segoeui.ttf",
        "/mnt/c/Windows/Fonts/arial.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];

    for path in CANDIDATES {
        if let Some(font) = load_font_from_path(path) {
            return Some(font);
        }
    }

    None
}

fn load_font_from_path(path: &str) -> Option<FontData> {
    let bytes = fs::read(path).ok()?;
    let blob = Blob::new(Arc::new(bytes.into_boxed_slice()));

    Some(FontData::new(blob, 0))
}
