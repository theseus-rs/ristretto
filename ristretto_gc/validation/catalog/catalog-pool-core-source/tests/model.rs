//! Exact slot-state atomics. Raw pointers, aggregate Arc ownership and ArcSwap's
//! internal protection are covered separately by native/Miri checks, not here.
#![cfg(not(target_family = "wasm"))]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering, fence};
use loom::thread;
#[path = "../src/slot_state.rs"]
mod slot_state;
use slot_state::SlotState;

#[test]
fn destruction_finishes_before_reuse_observes_the_free_slot() {
    loom::model(|| {
        let state = Arc::new(SlotState::new());
        let value = Arc::new(AtomicUsize::new(1));
        let drops = Arc::new(AtomicUsize::new(0));
        assert!(state.claim());
        assert_eq!(state.share(), 1);
        let owner = {
            let state = state.clone();
            let value = value.clone();
            let drops = drops.clone();
            thread::spawn(move || {
                if state.release() {
                    assert_eq!(value.swap(0, Ordering::Relaxed), 1);
                    assert_eq!(drops.fetch_add(1, Ordering::Relaxed), 0);
                    state.publish_free();
                }
            })
        };
        let other = {
            let state = state.clone();
            let value = value.clone();
            let drops = drops.clone();
            thread::spawn(move || {
                if state.release() {
                    assert_eq!(value.swap(0, Ordering::Relaxed), 1);
                    assert_eq!(drops.fetch_add(1, Ordering::Relaxed), 0);
                    state.publish_free();
                }
            })
        };
        let reused = state.claim();
        if reused {
            assert_eq!(value.load(Ordering::Relaxed), 0);
            assert_eq!(drops.load(Ordering::Relaxed), 1);
        }
        owner.join().unwrap();
        other.join().unwrap();
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        if !reused {
            assert!(state.claim());
        }
        assert!(state.release());
        state.publish_free();
    });
}

#[test]
fn zero_references_do_not_publish_reuse_before_destruction() {
    loom::model(|| {
        let state = Arc::new(SlotState::new());
        assert!(state.claim());
        assert!(state.release());
        let allocator = {
            let state = state.clone();
            thread::spawn(move || {
                assert!(!state.claim());
                assert!(state.is_occupied());
                let unrelated = SlotState::new();
                assert!(unrelated.claim());
                assert!(unrelated.release());
                unrelated.publish_free();
            })
        };
        allocator.join().unwrap();
        state.publish_free();
        assert!(state.claim());
        assert!(state.release());
        state.publish_free();
    });
}

#[test]
fn the_final_owner_acquires_prior_owner_writes() {
    loom::model(|| {
        let state = Arc::new(SlotState::new());
        let value = Arc::new(AtomicUsize::new(0));
        assert!(state.claim());
        state.share();
        let writer = {
            let state = state.clone();
            let value = value.clone();
            thread::spawn(move || {
                value.store(17, Ordering::Relaxed);
                if state.release() {
                    assert_eq!(value.load(Ordering::Relaxed), 17);
                    state.publish_free();
                }
            })
        };
        if state.release() {
            assert_eq!(value.load(Ordering::Relaxed), 17);
            state.publish_free();
        }
        writer.join().unwrap();
        assert!(!state.is_occupied());
    });
}

#[test]
fn an_existing_owner_may_clone_while_another_owner_releases() {
    loom::model(|| {
        let state = Arc::new(SlotState::new());
        assert!(state.claim());
        state.share();
        let releaser = {
            let state = state.clone();
            thread::spawn(move || assert!(!state.release()))
        };
        assert!(state.share() >= 1);
        assert!(!state.claim());
        assert!(!state.release());
        releaser.join().unwrap();
        assert!(state.release());
        state.publish_free();
    });
}
