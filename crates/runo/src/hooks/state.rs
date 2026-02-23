use std::any::{Any, TypeId};
use std::collections::HashMap;

struct StateEntry {
    type_id: TypeId,
    value: Box<dyn Any>,
    seen_this_frame: bool,
}

pub(crate) struct StateStore {
    entries: HashMap<String, StateEntry>,
    changed_this_frame: bool,
}

impl StateStore {
    pub(crate) fn new() -> Self {
        Self {
            entries: HashMap::new(),
            changed_this_frame: false,
        }
    }

    pub(crate) fn begin_frame(&mut self) {
        self.changed_this_frame = false;
        for entry in self.entries.values_mut() {
            entry.seen_this_frame = false;
        }
    }

    pub(crate) fn use_state<T, F>(&mut self, id: impl Into<String>, init: F) -> T
    where
        T: Clone + 'static,
        F: FnOnce() -> T,
    {
        let key = id.into();
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.seen_this_frame = true;
            ensure_type::<T>(&key, entry.type_id);
            return entry
                .value
                .downcast_ref::<T>()
                .expect("state type checked above")
                .clone();
        }

        let value = init();
        self.entries.insert(
            key,
            StateEntry {
                type_id: TypeId::of::<T>(),
                value: Box::new(value.clone()),
                seen_this_frame: true,
            },
        );
        value
    }

    pub(crate) fn set_state<T>(&mut self, id: impl Into<String>, value: T) -> bool
    where
        T: Clone + PartialEq + 'static,
    {
        let key = id.into();
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.seen_this_frame = true;
            ensure_type::<T>(&key, entry.type_id);
            let current = entry
                .value
                .downcast_mut::<T>()
                .expect("state type checked above");
            if *current == value {
                return false;
            }
            *current = value;
            self.changed_this_frame = true;
            return true;
        }

        self.entries.insert(
            key,
            StateEntry {
                type_id: TypeId::of::<T>(),
                value: Box::new(value),
                seen_this_frame: true,
            },
        );
        self.changed_this_frame = true;
        true
    }

    pub(crate) fn end_frame(&mut self) {
        self.entries.retain(|_, entry| entry.seen_this_frame);
    }

    pub(crate) fn take_changed(&mut self) -> bool {
        std::mem::take(&mut self.changed_this_frame)
    }
}

fn ensure_type<T: 'static>(key: &str, actual: TypeId) {
    let expected = TypeId::of::<T>();
    assert!(
        actual == expected,
        "state `{key}` was requested with a different type"
    );
}

#[cfg(test)]
mod tests {
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
}
