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
