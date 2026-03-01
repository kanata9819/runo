use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[cfg(test)]
#[path = "../../tests/unit/hooks/use_effect.rs"]
mod tests;

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
