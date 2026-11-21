//! Exact state atomics; pointer provenance and aggregate pool ownership are not modeled.
#![cfg(not(target_family = "wasm"))]
use super::AllocationError;
use loom::sync::Arc;
use loom::sync::atomic::{AtomicU64, Ordering};
use loom::thread;
#[expect(
    clippy::duplicate_mod,
    reason = "model the production state with Loom atomics"
)]
#[path = "state.rs"]
mod state;
use state::State;

#[test]
fn exact_generation_acquisition_excludes_reused_storage() {
    loom::model(|| {
        let state = Arc::new(State::with_generation_limit(state::MAX_GENERATION));
        let value = Arc::new(AtomicU64::new(7));
        let generation = state.claim().unwrap();
        state.publish_initialized(generation);
        let reader = {
            let state = state.clone();
            let value = value.clone();
            thread::spawn(move || {
                if state.acquire(generation) {
                    assert_eq!(value.load(Ordering::Relaxed), 7);
                    if state.release() {
                        assert_eq!(value.swap(0, Ordering::Relaxed), 7);
                        state.publish_free(generation);
                    }
                }
            })
        };
        if state.release() {
            assert_eq!(value.swap(0, Ordering::Relaxed), 7);
            state.publish_free(generation);
        }
        let replacement = state.claim();
        if let Some(replacement) = replacement {
            assert_eq!(value.load(Ordering::Relaxed), 0);
            value.store(99, Ordering::Relaxed);
            state.publish_initialized(replacement);
        }
        reader.join().unwrap();
        let replacement = replacement.unwrap_or_else(|| {
            let replacement = state.claim().unwrap();
            value.store(99, Ordering::Relaxed);
            state.publish_initialized(replacement);
            replacement
        });
        assert_ne!(replacement, generation);
        assert!(!state.acquire(generation));
        assert!(state.release());
        state.publish_free(replacement);
    });
}

#[test]
fn busy_state_excludes_acquisition_and_reuse_through_destruction() {
    loom::model(|| {
        let state = Arc::new(State::with_generation_limit(state::MAX_GENERATION));
        let generation = state.claim().unwrap();
        assert!(!state.acquire(generation));
        state.publish_initialized(generation);
        assert!(state.release());
        let other = {
            let state = state.clone();
            thread::spawn(move || {
                assert!(!state.acquire(generation));
                assert!(state.claim().is_none());
                assert!(state.occupied());
            })
        };
        other.join().unwrap();
        state.publish_free(generation);
        let next = state.claim().unwrap();
        assert_ne!(next, generation);
        state.publish_initialized(next);
        assert!(state.release());
        state.publish_free(next);
    });
}

#[test]
fn final_release_acquires_prior_owner_writes() {
    loom::model(|| {
        let state = Arc::new(State::with_generation_limit(state::MAX_GENERATION));
        let value = Arc::new(AtomicU64::new(0));
        let generation = state.claim().unwrap();
        state.publish_initialized(generation);
        assert!(state.acquire(generation));
        let writer = {
            let state = state.clone();
            let value = value.clone();
            thread::spawn(move || {
                value.store(17, Ordering::Relaxed);
                if state.release() {
                    assert_eq!(value.load(Ordering::Relaxed), 17);
                    state.publish_free(generation);
                }
            })
        };
        if state.release() {
            assert_eq!(value.load(Ordering::Relaxed), 17);
            state.publish_free(generation);
        }
        writer.join().unwrap();
        assert!(state.claim().is_some());
    });
}

#[test]
fn acquisition_observes_initialized_value() {
    loom::model(|| {
        let state = Arc::new(State::with_generation_limit(state::MAX_GENERATION));
        let value = Arc::new(AtomicU64::new(0));
        let announced = Arc::new(AtomicU64::new(0));
        let writer = {
            let state = state.clone();
            let value = value.clone();
            let announced = announced.clone();
            thread::spawn(move || {
                let generation = state.claim().unwrap();
                value.store(17, Ordering::Relaxed);
                state.publish_initialized(generation);
                announced.store(generation, Ordering::Relaxed);
            })
        };
        let generation = announced.load(Ordering::Relaxed);
        if state.acquire(generation) {
            assert_eq!(value.load(Ordering::Relaxed), 17);
            assert!(!state.release());
        }
        writer.join().unwrap();
        let generation = announced.load(Ordering::Relaxed);
        assert!(state.release());
        state.publish_free(generation);
    });
}

#[test]
fn exhausted_generation_cannot_be_reused() {
    loom::model(|| {
        let state = State::free_at(state::MAX_GENERATION);
        assert!(state.claim().is_none());
        assert!(!state.acquire(state::MAX_GENERATION));
    });
}

#[test]
fn competing_claims_cannot_cross_a_configured_final_generation() {
    loom::model(|| {
        let state = Arc::new(State::with_generation_limit(1));
        let worker = {
            let state = state.clone();
            thread::spawn(move || state.claim_available())
        };
        let right = state.claim_available();
        let left = worker.join().unwrap();
        assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
        let generation = left.or(right).unwrap();
        assert_eq!(generation, state::MAX_GENERATION);
        let failure = left.err().or_else(|| right.err()).unwrap();
        assert!(matches!(
            failure,
            AllocationError::Contended | AllocationError::GenerationExhausted
        ));
        state.publish_initialized(generation);
        assert!(state.acquire(generation));
        assert!(!state.release());
        assert!(state.release());
        state.publish_free(generation);
        assert_eq!(
            state.claim_available(),
            Err(AllocationError::GenerationExhausted)
        );
    });
}
