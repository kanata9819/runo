use std::fs;
use std::sync::Arc;

use vello::peniko::{Blob, FontData};

#[cfg(test)]
#[path = "../tests/unit/font.rs"]
mod tests;

pub(crate) fn load_default_font() -> Option<FontData> {
    const CANDIDATES: &[&str] = &[
        "C:\\Windows\\Fonts\\consola.ttf",
        "C:\\Windows\\Fonts\\consolab.ttf",
        "C:\\Windows\\Fonts\\YuGothM.ttc",
        "C:\\Windows\\Fonts\\meiryo.ttc",
        "C:\\Windows\\Fonts\\msgothic.ttc",
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\arial.ttf",
        "/mnt/c/Windows/Fonts/consola.ttf",
        "/mnt/c/Windows/Fonts/consolab.ttf",
        "/mnt/c/Windows/Fonts/YuGothM.ttc",
        "/mnt/c/Windows/Fonts/meiryo.ttc",
        "/mnt/c/Windows/Fonts/msgothic.ttc",
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
    let bytes: Vec<u8> = fs::read(path).ok()?;
    let blob: Blob<_> = Blob::new(Arc::new(bytes.into_boxed_slice()));

    Some(FontData::new(blob, 0))
}
