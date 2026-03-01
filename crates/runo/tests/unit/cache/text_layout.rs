use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use vello::peniko::{Blob, FontData};

use super::*;

fn sample_font() -> FontData {
    let bytes = vec![0_u8; 16];
    FontData::new(Blob::new(Arc::new(bytes.into_boxed_slice())), 0)
}

fn clear_cache() {
    TEXT_LAYOUT_CACHE
        .lock()
        .expect("text layout cache lock")
        .clear();
}

#[test]
fn same_key_hits_cache_without_recompute() {
    clear_cache();
    let font = sample_font();
    let calls = AtomicUsize::new(0);

    let first = get_or_insert_layout(&font, "hello", 14.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Some((Vec::new(), 42.0))
    });
    let second = get_or_insert_layout(&font, "hello", 14.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Some((Vec::new(), 99.0))
    });

    assert_eq!(first.as_ref().map(|v| v.1), Some(42.0));
    assert_eq!(second.as_ref().map(|v| v.1), Some(42.0));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn different_text_or_size_uses_different_cache_key() {
    clear_cache();
    let font = sample_font();
    let calls = AtomicUsize::new(0);

    let _ = get_or_insert_layout(&font, "hello", 14.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Some((Vec::new(), 10.0))
    });
    let _ = get_or_insert_layout(&font, "world", 14.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Some((Vec::new(), 11.0))
    });
    let _ = get_or_insert_layout(&font, "hello", 16.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Some((Vec::new(), 12.0))
    });

    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

#[test]
fn none_result_is_not_cached() {
    clear_cache();
    let font = sample_font();
    let calls = AtomicUsize::new(0);

    let first = get_or_insert_layout(&font, "x", 12.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        None
    });
    let second = get_or_insert_layout(&font, "x", 12.0, || {
        calls.fetch_add(1, Ordering::SeqCst);
        Some((Vec::new(), 5.0))
    });

    assert!(first.is_none());
    assert_eq!(second.as_ref().map(|v| v.1), Some(5.0));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
