use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub type EffectCleanup = Box<dyn FnMut() + 'static>;

struct EffectEntry {
    deps_hash: u64,
    cleanup: Option<EffectCleanup>,
    seen_this_frame: bool,
}

pub(crate) struct EffectStore {
    entries: HashMap<String, EffectEntry>,
}

impl EffectStore {
    pub(crate) fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub(crate) fn begin_frame(&mut self) {
        for entry in self.entries.values_mut() {
            entry.seen_this_frame = false;
        }
    }

    pub(crate) fn use_effect<D, F>(&mut self, id: impl Into<String>, deps: D, effect: F)
    where
        D: Hash,
        F: FnOnce() -> Option<EffectCleanup>,
    {
        let key = id.into();
        let deps_hash = hash_value(&deps);

        if let Some(entry) = self.entries.get_mut(&key) {
            entry.seen_this_frame = true;
            if entry.deps_hash != deps_hash {
                if let Some(mut cleanup) = entry.cleanup.take() {
                    cleanup();
                }
                entry.deps_hash = deps_hash;
                entry.cleanup = effect();
            }
            return;
        }

        self.entries.insert(
            key,
            EffectEntry {
                deps_hash,
                cleanup: effect(),
                seen_this_frame: true,
            },
        );
    }

    pub(crate) fn end_frame(&mut self) {
        self.entries.retain(|_, entry| {
            if entry.seen_this_frame {
                true
            } else {
                if let Some(mut cleanup) = entry.cleanup.take() {
                    cleanup();
                }
                false
            }
        });
    }
}

impl Drop for EffectStore {
    fn drop(&mut self) {
        for entry in self.entries.values_mut() {
            if let Some(mut cleanup) = entry.cleanup.take() {
                cleanup();
            }
        }
    }
}

fn hash_value<T: Hash>(value: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn unchanged_deps_do_not_rerun_effect() {
        let mut store = EffectStore::new();
        let calls = Rc::new(RefCell::new(Vec::<String>::new()));

        store.begin_frame();
        store.use_effect("e1", 1_u32, {
            let calls = Rc::clone(&calls);
            move || {
                calls.borrow_mut().push("effect".to_string());
                None
            }
        });
        store.end_frame();

        store.begin_frame();
        store.use_effect("e1", 1_u32, {
            let calls = Rc::clone(&calls);
            move || {
                calls.borrow_mut().push("effect-again".to_string());
                None
            }
        });
        store.end_frame();

        assert_eq!(calls.borrow().as_slice(), &["effect".to_string()]);
    }

    #[test]
    fn changed_deps_run_cleanup_then_new_effect() {
        let mut store = EffectStore::new();
        let calls = Rc::new(RefCell::new(Vec::<String>::new()));

        store.begin_frame();
        store.use_effect("e1", 1_u32, {
            let calls = Rc::clone(&calls);
            move || {
                calls.borrow_mut().push("effect-1".to_string());
                Some(Box::new({
                    let calls = Rc::clone(&calls);
                    move || calls.borrow_mut().push("cleanup-1".to_string())
                }))
            }
        });
        store.end_frame();

        store.begin_frame();
        store.use_effect("e1", 2_u32, {
            let calls = Rc::clone(&calls);
            move || {
                calls.borrow_mut().push("effect-2".to_string());
                None
            }
        });
        store.end_frame();

        assert_eq!(
            calls.borrow().as_slice(),
            &[
                "effect-1".to_string(),
                "cleanup-1".to_string(),
                "effect-2".to_string(),
            ]
        );
    }

    #[test]
    fn unseen_effect_cleanup_runs_on_end_frame() {
        let mut store = EffectStore::new();
        let calls = Rc::new(RefCell::new(Vec::<String>::new()));

        store.begin_frame();
        store.use_effect("e1", 1_u32, {
            let calls = Rc::clone(&calls);
            move || {
                Some(Box::new({
                    let calls = Rc::clone(&calls);
                    move || calls.borrow_mut().push("cleanup".to_string())
                }))
            }
        });
        store.end_frame();

        store.begin_frame();
        store.end_frame();

        assert_eq!(calls.borrow().as_slice(), &["cleanup".to_string()]);
    }

    #[test]
    fn cleanup_runs_on_drop() {
        let calls = Rc::new(RefCell::new(Vec::<String>::new()));

        {
            let mut store = EffectStore::new();
            store.begin_frame();
            store.use_effect("e1", 1_u32, {
                let calls = Rc::clone(&calls);
                move || {
                    Some(Box::new({
                        let calls = Rc::clone(&calls);
                        move || calls.borrow_mut().push("cleanup-on-drop".to_string())
                    }))
                }
            });
            store.end_frame();
        }

        assert_eq!(calls.borrow().as_slice(), &["cleanup-on-drop".to_string()]);
    }
}
