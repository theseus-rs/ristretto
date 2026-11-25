//! The production handle transitions with Loom atomics, independent of GC work.
#![cfg(not(target_family = "wasm"))]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::thread;
#[path = "../src/publication_handle_state.rs"]
mod state;
use state::State;

fn destroy(alive: &AtomicBool, destructions: &AtomicUsize) {
    assert!(
        alive.swap(false, Ordering::Relaxed),
        "private resources destroyed twice"
    );
    destructions.fetch_add(1, Ordering::Relaxed);
}

#[test]
fn acquisition_racing_final_release_never_observes_destroyed_resources() {
    loom::model(|| {
        let state = Arc::new(State::new());
        let alive = Arc::new(AtomicBool::new(true));
        let destructions = Arc::new(AtomicUsize::new(0));
        let helper = {
            let (state, alive, destructions) = (state.clone(), alive.clone(), destructions.clone());
            thread::spawn(move || {
                if state.acquire().is_ok() {
                    assert!(
                        alive.load(Ordering::Relaxed),
                        "helper observed destroyed private resources"
                    );
                    if state.release() {
                        destroy(&alive, &destructions);
                    }
                }
            })
        };
        if state.release() {
            destroy(&alive, &destructions);
        }
        helper.join().unwrap();
        assert_eq!(destructions.load(Ordering::Relaxed), 1);
        assert_eq!(state.acquire(), Err(state::AcquireError::Closed));
    });
}

#[test]
fn exclusive_restoration_and_new_helper_admission_have_one_winner() {
    loom::model(|| {
        let state = Arc::new(State::new());
        let alive = Arc::new(AtomicBool::new(true));
        let destructions = Arc::new(AtomicUsize::new(0));
        let helper = {
            let (state, alive, destructions) = (state.clone(), alive.clone(), destructions.clone());
            thread::spawn(move || {
                if state.acquire().is_ok() {
                    assert!(
                        alive.load(Ordering::Relaxed),
                        "restoration raced a live helper"
                    );
                    if state.release() {
                        destroy(&alive, &destructions);
                    }
                }
            })
        };
        if state.close_if_unique() || state.release() {
            destroy(&alive, &destructions);
        }
        helper.join().unwrap();
        assert_eq!(destructions.load(Ordering::Relaxed), 1);
        assert_eq!(state.acquire(), Err(state::AcquireError::Closed));
    });
}

#[test]
fn the_last_release_observes_every_completed_helper() {
    loom::model(|| {
        let state = Arc::new(State::new());
        state.acquire().unwrap();
        let writes = Arc::new((AtomicUsize::new(0), AtomicUsize::new(0)));
        let destructions = Arc::new(AtomicUsize::new(0));
        let helper = {
            let (state, writes, destructions) =
                (state.clone(), writes.clone(), destructions.clone());
            thread::spawn(move || {
                writes.0.store(1, Ordering::Relaxed);
                if state.release() {
                    assert_eq!(
                        writes.1.load(Ordering::Relaxed),
                        1,
                        "last release missed prior helper"
                    );
                    destructions.fetch_add(1, Ordering::Relaxed);
                }
            })
        };
        writes.1.store(1, Ordering::Relaxed);
        if state.release() {
            assert_eq!(
                writes.0.load(Ordering::Relaxed),
                1,
                "last release missed prior helper"
            );
            destructions.fetch_add(1, Ordering::Relaxed);
        }
        helper.join().unwrap();
        assert_eq!(destructions.load(Ordering::Relaxed), 1);
    });
}

#[test]
fn a_retained_helper_excludes_exclusive_restoration() {
    loom::model(|| {
        let state = Arc::new(State::new());
        state.acquire().unwrap();
        let helper = {
            let state = state.clone();
            thread::spawn(move || {
                if state.acquire().is_ok() {
                    assert!(!state.release());
                }
            })
        };
        assert!(!state.close_if_unique());
        helper.join().unwrap();
        assert!(!state.release());
        assert!(state.close_if_unique());
        assert_eq!(state.acquire(), Err(state::AcquireError::Closed));
    });
}

#[test]
fn announced_ownership_cannot_restore_and_closed_announcement_cannot_reopen() {
    loom::model(|| {
        let state = State::new();
        state.announce();
        assert!(state.announced());
        assert!(
            !state.close_if_unique(),
            "announced handle restored exclusive ownership"
        );
        assert!(state.acquire().is_ok());
        assert!(!state.release());
        assert!(state.release());
        assert!(state.announced());
        assert_eq!(
            state.acquire(),
            Err(state::AcquireError::Closed),
            "closed announced header reopened"
        );
    });
}

#[test]
fn acquire_announce_release_cannot_recreate_the_restoration_token() {
    loom::model(|| {
        let state = Arc::new(State::new());
        let announced = Arc::new(AtomicBool::new(false));
        let helper = {
            let state = state.clone();
            let announced = announced.clone();
            thread::spawn(move || {
                if state.acquire().is_ok() {
                    state.announce();
                    announced.store(true, Ordering::Relaxed);
                    state.release();
                }
            })
        };
        let restored = state.close_if_unique();
        if !restored {
            state.release();
        }
        helper.join().unwrap();
        assert!(
            !(restored && announced.load(Ordering::Relaxed)),
            "announcement raced successful exclusive restoration"
        );
        assert_eq!(state.acquire(), Err(state::AcquireError::Closed));
    });
}

#[test]
fn the_last_release_observes_announcement_before_cleanup() {
    loom::model(|| {
        let state = Arc::new(State::new());
        state.acquire().unwrap();
        let closures = Arc::new(AtomicUsize::new(0));
        let helper = {
            let state = state.clone();
            let closures = closures.clone();
            thread::spawn(move || {
                state.announce();
                if state.release() {
                    assert!(state.announced(), "last helper lost announcement");
                    closures.fetch_add(1, Ordering::Relaxed);
                }
            })
        };
        if state.release() {
            assert!(state.announced(), "last origin lost announcement");
            closures.fetch_add(1, Ordering::Relaxed);
        }
        helper.join().unwrap();
        assert_eq!(closures.load(Ordering::Relaxed), 1);
    });
}

#[test]
fn announced_acquisition_racing_final_release_preserves_private_lifetime() {
    loom::model(|| {
        let state = Arc::new(State::new());
        state.announce();
        let alive = Arc::new(AtomicBool::new(true));
        let destructions = Arc::new(AtomicUsize::new(0));
        let helper = {
            let (state, alive, destructions) = (state.clone(), alive.clone(), destructions.clone());
            thread::spawn(move || {
                if state.acquire().is_ok() {
                    assert!(
                        alive.load(Ordering::Relaxed),
                        "announced helper saw cancelled private resources"
                    );
                    if state.release() {
                        destroy(&alive, &destructions);
                    }
                }
            })
        };
        if state.release() {
            destroy(&alive, &destructions);
        }
        helper.join().unwrap();
        assert_eq!(destructions.load(Ordering::Relaxed), 1);
        assert_eq!(state.acquire(), Err(state::AcquireError::Closed));
    });
}

#[test]
fn handle_count_exhaustion_cannot_overflow_into_the_announcement_bit() {
    loom::model(|| {
        for announced in [false, true] {
            let state = State::with_count_for_test(usize::MAX >> 1, announced);
            assert_eq!(
                state.acquire(),
                Err(state::AcquireError::Exhausted),
                "handle count crossed the announcement boundary"
            );
            assert_eq!(state.announced(), announced);
            assert!(!state.release());
            assert!(state.acquire().is_ok());
            assert_eq!(state.announced(), announced);
        }
    });
}
