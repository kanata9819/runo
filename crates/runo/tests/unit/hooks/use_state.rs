
use super::*;

#[test]
fn use_state_keeps_value_while_seen_every_frame() {
    let mut store = StateStore::new();

    store.begin_frame();
    let first = store.use_state("counter", || 10_u32);
    store.end_frame();

    store.begin_frame();
    let second = store.use_state("counter", || 0_u32);
    store.end_frame();

    assert_eq!(first, 10);
    assert_eq!(second, 10);
}

#[test]
fn set_state_marks_changed_only_when_value_changes() {
    let mut store = StateStore::new();

    store.begin_frame();
    let changed = store.set_state("count", 1_u32);
    assert!(changed);
    assert!(store.take_changed());
    assert!(!store.take_changed());

    let unchanged = store.set_state("count", 1_u32);
    assert!(!unchanged);
    assert!(!store.take_changed());
    store.end_frame();
}

#[test]
fn unseen_state_is_removed_at_end_of_frame() {
    let mut store = StateStore::new();

    store.begin_frame();
    let _ = store.use_state("temp", || 5_u32);
    store.end_frame();

    store.begin_frame();
    store.end_frame();

    store.begin_frame();
    let value = store.use_state("temp", || 99_u32);
    store.end_frame();
    assert_eq!(value, 99);
}

#[test]
#[should_panic(expected = "different type")]
fn requesting_same_key_with_different_type_panics() {
    let mut store = StateStore::new();

    store.begin_frame();
    let _ = store.use_state("same", || 1_u32);
    store.end_frame();

    store.begin_frame();
    let _: String = store.use_state("same", String::new);
}
