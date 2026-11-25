//! Execute the production storage-admission state machine with Loom atomics.
//! This covers descriptor borrows, not the collector's graph/reference protocol.
#![cfg(not(target_family = "wasm"))]
#![allow(unsafe_code)] // Loom's checked cell detects accesses across destruction.
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::thread;

#[path = "../src/storage_state.rs"]
mod storage_state;

#[test]
fn admission_racing_retirement_never_borrows_destroyed_storage() {
    loom::model(|| {
        let state = Arc::new(storage_state::StorageState::default());
        let destroyed = Arc::new(AtomicBool::new(false));
        let notifications = Arc::new(AtomicUsize::new(0));
        let data = Arc::new(loom::cell::UnsafeCell::new(0));
        let reader = {
            let (state, destroyed, notifications, data) = (
                state.clone(),
                destroyed.clone(),
                notifications.clone(),
                data.clone(),
            );
            thread::spawn(move || {
                if state.acquire() {
                    assert!(!destroyed.load(Ordering::Acquire));
                    data.with_mut(|ptr| {
                        // SAFETY: one admitted borrower; destruction excludes it.
                        unsafe {
                            *ptr = 42;
                        }
                    });
                    if state.release() {
                        notifications.fetch_add(1, Ordering::Relaxed);
                    }
                }
            })
        };
        let ready = state.retire();
        if !ready && state.arm_notification() {
            notifications.fetch_add(1, Ordering::Relaxed);
        }
        assert!(!state.acquire());
        if ready {
            notifications.fetch_add(1, Ordering::Relaxed);
            assert!(state.destroy());
            destroyed.store(true, Ordering::Release);
            data.with(|ptr| {
                // SAFETY: destruction owns storage exclusively.
                assert!(unsafe { *ptr } == 0 || unsafe { *ptr } == 42);
            });
        }
        reader.join().unwrap();
        if !ready {
            assert!(state.ready());
            assert!(state.destroy());
            data.with(|ptr| {
                // SAFETY: the final borrow ended before destruction.
                assert_eq!(unsafe { *ptr }, 42);
            });
        }
        assert_eq!(notifications.load(Ordering::Acquire), 1);
        assert!(!state.destroy());
        assert!(!state.acquire());
    });
}

#[test]
fn multiple_borrowers_publish_one_readiness_notification() {
    loom::model(|| {
        let state = Arc::new(storage_state::StorageState::default());
        let notifications = Arc::new(AtomicUsize::new(0));
        let writes = Arc::new(AtomicUsize::new(0));
        assert!(state.acquire());
        assert!(state.acquire());
        let readers: Vec<_> = (0..2)
            .map(|_| {
                let (state, notifications, writes) =
                    (state.clone(), notifications.clone(), writes.clone());
                thread::spawn(move || {
                    writes.fetch_add(1, Ordering::Relaxed);
                    if state.release() {
                        notifications.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();
        if state.retire() || state.arm_notification() {
            notifications.fetch_add(1, Ordering::Relaxed);
        }
        for reader in readers {
            reader.join().unwrap();
        }
        assert!(state.ready());
        assert!(state.destroy());
        assert_eq!(writes.load(Ordering::Relaxed), 2);
        assert_eq!(notifications.load(Ordering::Acquire), 1);
    });
}

#[test]
fn arming_after_accounting_publishes_exactly_one_deferred_task() {
    loom::model(|| {
        let state = Arc::new(storage_state::StorageState::default());
        assert!(state.acquire());
        assert!(!state.retire());
        let charged = Arc::new(AtomicBool::new(false));
        let borrower = {
            let state = state.clone();
            let charged = charged.clone();
            thread::spawn(move || {
                let claimed = state.release();
                if claimed {
                    assert!(charged.swap(false, Ordering::AcqRel));
                }
                claimed
            })
        };
        charged.store(true, Ordering::Relaxed);
        let claimed = state.arm_notification();
        if claimed {
            assert!(charged.swap(false, Ordering::AcqRel));
        }
        assert_ne!(claimed, borrower.join().unwrap());
        assert!(state.destroy());
        assert!(!state.destroy());
        assert!(!charged.load(Ordering::Acquire));
    });
}
