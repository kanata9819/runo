use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use vello::Glyph;
use vello::peniko::FontData;

const MAX_TEXT_LAYOUT_CACHE_ENTRIES: usize = 4096;
type TextLayoutValue = (Vec<Glyph>, f32);
type TextLayoutMap = HashMap<TextLayoutCacheKey, TextLayoutValue>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TextLayoutCacheKey {
    font_ptr: usize,
    font_len: usize,
    font_index: u32,
    text: String,
    font_size_bits: u32,
}

static TEXT_LAYOUT_CACHE: LazyLock<Mutex<TextLayoutMap>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn get_or_insert_layout(
    font: &FontData,
    text: &str,
    font_size: f32,
    compute: impl FnOnce() -> Option<(Vec<Glyph>, f32)>,
) -> Option<(Vec<Glyph>, f32)> {
    let key = TextLayoutCacheKey {
        font_ptr: font.data.as_ref().as_ptr() as usize,
        font_len: font.data.as_ref().len(),
        font_index: font.index,
        text: text.to_string(),
        font_size_bits: font_size.to_bits(),
    };

    if let Ok(cache) = TEXT_LAYOUT_CACHE.lock()
        && let Some(hit) = cache.get(&key)
    {
        return Some(hit.clone());
    }

    let value = compute()?;
    if let Ok(mut cache) = TEXT_LAYOUT_CACHE.lock() {
        if cache.len() >= MAX_TEXT_LAYOUT_CACHE_ENTRIES {
            cache.clear();
        }
        cache.insert(key, value.clone());
    }

    Some(value)
}

#[cfg(test)]
#[path = "../../tests/unit/cache/text_layout.rs"]
mod tests;
