use super::*;

#[cfg(not(target_family = "wasm"))]
#[test]
fn final_release_orders_another_owners_read_before_reuse() {
    let pool = Pool::new(1);
    let owner = pool.try_allocate(17usize).ok().unwrap();
    let reader = owner.try_clone().unwrap();
    let finished = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        let finished = &finished;
        let worker = scope.spawn(move || {
            assert_eq!(std::hint::black_box(*reader), 17);
            drop(reader);
            // This relaxed signal only schedules the fixture. Slot reference
            // release/acquisition must carry the read-to-reuse ordering.
            finished.store(1, Ordering::Relaxed);
        });
        while finished.load(Ordering::Relaxed) == 0 {
            std::thread::yield_now();
        }
        drop(owner);
        let replacement = pool.try_allocate(29).ok().unwrap();
        assert_eq!(*replacement, 29);
        worker.join().unwrap();
    });
}

#[test]
fn observer_preserves_backing_without_retaining_a_reused_value() {
    let pool = Pool::new(1);
    let backing = Arc::downgrade(&pool);
    let first = pool.try_allocate(7_u64).unwrap();
    let old = first.observe();
    drop(first);
    assert!(old.try_load().is_none());
    let next = pool.try_allocate(9).unwrap();
    let current = next.observe();
    assert!(old.try_load().is_none());
    let held = current.try_load().unwrap();
    drop(next);
    assert_eq!(*held, 9);
    drop(held);
    assert!(current.try_load().is_none());
    drop(pool);
    drop(old);
    assert!(backing.upgrade().is_some());
    drop(current);
    assert!(backing.upgrade().is_none());
}

#[test]
fn generation_exhaustion_fails_without_wrapping() {
    let state = State::free_at(state::MAX_GENERATION - 1);
    let generation = state.claim().unwrap();
    assert_eq!(generation, state::MAX_GENERATION);
    state.publish_initialized(generation);
    assert!(state.acquire(generation));
    assert!(!state.release());
    assert!(state.release());
    assert!(!state.acquire(generation));
    assert!(state.claim().is_none());
    state.publish_free(generation);
    assert!(state.claim().is_none());
    assert!(!state.acquire(0));
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn stopped_destructor_excludes_reuse_but_not_another_slot() {
    use std::sync::{Mutex, mpsc};
    struct Wait {
        entered: Option<mpsc::Sender<()>>,
        resume: Mutex<mpsc::Receiver<()>>,
    }
    impl Drop for Wait {
        fn drop(&mut self) {
            if let Some(entered) = self.entered.take() {
                entered.send(()).unwrap();
                self.resume.get_mut().unwrap().recv().unwrap();
            }
        }
    }
    let pool = Pool::new(2);
    let (entered, observed) = mpsc::channel();
    let (resume, wait) = mpsc::channel();
    let value = pool
        .try_allocate_at(
            0,
            Wait {
                entered: Some(entered),
                resume: Mutex::new(wait),
            },
        )
        .unwrap_or_else(|_| panic!("fresh slot occupied"));
    let old = value.token;
    let worker = std::thread::spawn(move || drop(value));
    observed.recv().unwrap();
    assert!(pool.try_acquire(old).is_none());
    let (_, never) = mpsc::channel();
    let spare = Wait {
        entered: None,
        resume: Mutex::new(never),
    };
    let Err(spare) = pool.try_allocate_at(0, spare) else {
        panic!("reused slot before destructor finished");
    };
    let spare = pool
        .try_allocate_at(1, spare)
        .unwrap_or_else(|_| panic!("unrelated slot blocked"));
    drop(spare);
    resume.send(()).unwrap();
    worker.join().unwrap();
    let (_, never) = mpsc::channel();
    let reused = pool
        .try_allocate_at(
            0,
            Wait {
                entered: None,
                resume: Mutex::new(never),
            },
        )
        .unwrap_or_else(|_| panic!("completed destructor retained slot"));
    drop(reused);
}

#[cfg(all(panic = "unwind", not(target_family = "wasm")))]
#[test]
fn panicking_destructor_releases_slot_and_backing() {
    struct Panic(bool);
    impl Drop for Panic {
        fn drop(&mut self) {
            assert!(!self.0, "injected drop failure");
        }
    }
    let pool = Pool::new(1);
    let weak = Arc::downgrade(&pool);
    let first = pool
        .try_allocate(Panic(true))
        .unwrap_or_else(|_| panic!("fresh slot occupied"));
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(first))).is_err());
    let next = pool
        .try_allocate(Panic(false))
        .unwrap_or_else(|_| panic!("unwind stranded slot"));
    drop(pool);
    drop(next);
    assert!(weak.upgrade().is_none());
}

#[test]
fn configured_generation_limit_rejects_reuse_without_invalidating_live_owners() {
    let pool = Pool::with_generation_limit(2, 2);
    drop(pool.try_allocate_at_status(0, 1usize).unwrap());
    let last = pool.try_allocate_at_status(0, 2usize).unwrap();
    let clone = last.try_clone().unwrap();
    let observed = last.observe();
    let failure = pool.try_allocate_at_status(0, 3usize).err().unwrap();
    assert_eq!(failure.error, AllocationError::GenerationExhausted);
    assert_eq!(failure.value, 3);
    assert!(pool.generation_exhaustion_observed());
    assert_eq!(*last, 2);
    assert_eq!(*clone, 2);
    assert_eq!(*observed.try_load().unwrap(), 2);
    drop((last, clone));
    assert!(observed.try_load().is_none());
    assert_eq!(
        pool.try_allocate_at_status(0, 4usize).err().unwrap().error,
        AllocationError::GenerationExhausted
    );
    // This reports an exhausted selected slot, not whole-pool exhaustion.
    assert_eq!(*pool.try_allocate_at_status(1, 9usize).unwrap(), 9);
}

#[test]
fn ordinary_unavailability_and_invalid_slots_do_not_report_generation_exhaustion() {
    let pool = Pool::with_generation_limit(1, 2);
    let first = pool.try_allocate_status(11usize).unwrap();
    let failure = pool.try_allocate_status(12usize).err().unwrap();
    assert_eq!(
        (failure.error, failure.value),
        (AllocationError::Unavailable, 12)
    );
    assert_eq!(
        pool.try_allocate_at_status(1, 13usize).err().unwrap().error,
        AllocationError::InvalidSlot
    );
    assert!(!pool.generation_exhaustion_observed());
    drop(first);
    let last = pool.try_allocate_status(12usize).unwrap();
    assert_eq!(*last, 12);
    assert!(!pool.generation_exhaustion_observed());
    drop(last);
    assert_eq!(
        pool.try_allocate_status(13usize).err().unwrap().error,
        AllocationError::GenerationExhausted
    );
}

#[test]
fn setup_encodes_exact_allocation_budgets_and_preserves_default_first_tokens() {
    for limit in [1, 2, 3, 17, MAX_GENERATION] {
        let pool = Pool::with_generation_limit(1, limit);
        let attempts = if limit == MAX_GENERATION { 1 } else { limit };
        for ordinal in 1..=attempts {
            let owner = pool.try_allocate_status(ordinal).unwrap();
            assert_eq!(owner.token.generation(), MAX_GENERATION - limit + ordinal);
            let stale = owner.observe();
            drop(owner);
            assert!(stale.try_load().is_none());
        }
        assert!(!pool.generation_exhaustion_observed());
        if limit == MAX_GENERATION {
            let next = pool.try_allocate_status(2).unwrap();
            assert_eq!(next.token.generation(), 2);
        } else {
            assert_eq!(
                pool.try_allocate_status(99).err().unwrap().error,
                AllocationError::GenerationExhausted
            );
            assert!(pool.generation_exhaustion_observed());
        }
    }
}
