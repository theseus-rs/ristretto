//! Exact nonce/binding atomics, with separate existing graph-handle admission.
//! Excludes catalog membership, roots, callbacks and physical metadata ownership.
#![cfg(not(target_family = "wasm"))]
#![allow(dead_code)]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use loom::thread;
#[path = "../src/publication_record_state.rs"]
mod binding;
#[path = "../src/publication_handle_state.rs"]
mod handles;

#[test]
fn concurrent_reservations_never_duplicate_a_successful_nonce() {
    loom::model(|| {
        let state = Arc::new(binding::State::new());
        let helper = {
            let state = state.clone();
            thread::spawn(move || state.reserve())
        };
        let local = state.reserve();
        let other = helper.join().unwrap();
        if let (Ok(first), Ok(second)) = (&local, &other) {
            assert_ne!(
                first.value_for_test(),
                second.value_for_test(),
                "duplicate preparation nonce"
            );
        }
        assert!(local.is_ok() || other.is_ok());
        let later = state.reserve().unwrap();
        for earlier in [local, other].into_iter().flatten() {
            assert_ne!(
                earlier.value_for_test(),
                later.value_for_test(),
                "consumed nonce reused"
            );
        }
    });
}

#[test]
fn different_records_cannot_both_bind_one_payload() {
    loom::model(|| {
        let state = Arc::new(binding::State::new());
        let first = state.reserve().unwrap();
        let second = state.reserve().unwrap();
        let helper = {
            let state = state.clone();
            thread::spawn(move || {
                let won = state.bind(&first);
                assert_eq!(
                    state.bind(&first),
                    won,
                    "binding changed after first result"
                );
                won
            })
        };
        let won = state.bind(&second);
        assert_eq!(
            state.bind(&second),
            won,
            "binding changed after second result"
        );
        assert_ne!(
            helper.join().unwrap(),
            won,
            "two record identities bound one payload"
        );
    });
}

#[test]
fn repeated_helpers_share_only_the_original_binding() {
    loom::model(|| {
        let state = Arc::new(binding::State::new());
        let winner = Arc::new(state.reserve().unwrap());
        assert!(state.bind(&winner));
        let loser = state.reserve().unwrap();
        let helper = {
            let (state, winner) = (state.clone(), winner.clone());
            thread::spawn(move || assert!(state.bind(&winner), "same-record helper rejected"))
        };
        assert!(
            !state.bind(&loser),
            "winner overwritten by a different record"
        );
        helper.join().unwrap();
        assert!(state.bind(&winner));
    });
}

#[test]
fn dropping_unbound_and_bound_tickets_never_resets_or_reuses_identity() {
    loom::model(|| {
        let state = binding::State::new();
        let previous = {
            let discarded = state.reserve().unwrap();
            discarded.value_for_test()
        };
        {
            let winner = state.reserve().unwrap();
            assert_ne!(winner.value_for_test(), previous);
            assert!(state.bind(&winner), "unbound discard claimed the payload");
        }
        let loser = state.reserve().unwrap();
        assert!(!state.bind(&loser), "bound ticket drop reset the payload");
    });
}

#[test]
fn exhausted_reservation_never_wraps_or_changes_existing_binding() {
    loom::model(|| {
        let state = binding::State::new();
        let first = state.reserve().unwrap();
        state.exhaust_for_test();
        for _ in 0..2 {
            assert!(
                matches!(state.reserve(), Err(binding::ReserveError::Exhausted)),
                "preparation nonce wrapped"
            );
        }
        assert!(state.bind(&first), "exhaustion consumed binding");
        assert!(state.bind(&first));
    });
}

#[test]
fn graph_admission_excludes_final_cleanup_during_record_binding() {
    loom::model(|| {
        let count = Arc::new(handles::State::new());
        let binding = Arc::new(binding::State::new());
        let ticket = binding.reserve().unwrap();
        count.announce();
        let alive = Arc::new(AtomicBool::new(true));
        let destroyed = Arc::new(AtomicUsize::new(0));
        let helper = {
            let (count, binding, alive, destroyed) = (
                count.clone(),
                binding.clone(),
                alive.clone(),
                destroyed.clone(),
            );
            thread::spawn(move || {
                if count.acquire().is_ok() {
                    assert!(binding.bind(&ticket));
                    assert!(
                        alive.load(Ordering::Relaxed),
                        "binding raced private cleanup"
                    );
                    if count.release() {
                        assert!(alive.swap(false, Ordering::Relaxed));
                        destroyed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            })
        };
        if count.release() {
            assert!(alive.swap(false, Ordering::Relaxed));
            destroyed.fetch_add(1, Ordering::Relaxed);
        }
        helper.join().unwrap();
        assert_eq!(destroyed.load(Ordering::Relaxed), 1);
        assert_eq!(count.acquire(), Err(handles::AcquireError::Closed));
    });
}
