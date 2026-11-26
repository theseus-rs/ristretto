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
fn exact_generation_reuse_and_last_pool_owner() {
    let pool = Pool::new(1);
    let weak = Arc::downgrade(&pool);
    let link = Link::empty(pool.clone());
    let first = pool.try_allocate_at(0, 7_u64).unwrap();
    let stale = first.token;
    assert!(matches!(link.compare(None, Some(first)), Compare::Changed));
    let old = link.try_load().unwrap().unwrap();
    assert_eq!(*old, 7);
    assert!(link.points_to(&old));
    assert!(matches!(link.compare(Some(&old), None), Compare::Changed));
    assert!(!link.points_to(&old));
    assert!(pool.try_allocate_at(0, 8).is_err());
    drop(old);
    let new = pool.try_allocate_at(0, 9).unwrap();
    assert!(pool.try_acquire(stale).is_none());
    assert!(matches!(link.compare(None, Some(new)), Compare::Changed));
    let held = link.try_load().unwrap().unwrap();
    assert!(link.points_to(&held));
    // A different pool can use the exact same numeric slot and generation.
    let foreign = Pool::new(1);
    drop(foreign.try_allocate(3).unwrap());
    let other = foreign.try_allocate(4).unwrap();
    assert_eq!(held.token.0, other.token.0);
    assert!(!link.points_to(&other));
    drop(link);
    drop(pool);
    assert!(weak.upgrade().is_some());
    assert_eq!(*held, 9);
    drop(held);
    assert!(weak.upgrade().is_none());
}

#[test]
fn stale_expected_and_foreign_proposals_keep_ownership() {
    let pool = Pool::new(4);
    let foreign = Pool::new(1);
    let link = Link::empty(pool.clone());
    let first = pool.try_allocate_at(0, 1_u64).unwrap();
    let held = first.try_clone().unwrap();
    assert!(matches!(link.compare(None, Some(first)), Compare::Changed));
    let next = pool.try_allocate_at(1, 2).unwrap();
    assert!(matches!(
        link.compare(Some(&held), Some(next)),
        Compare::Changed
    ));
    let proposal = pool.try_allocate_at(2, 3).unwrap();
    let Compare::Unchanged(Some(proposal)) = link.compare(Some(&held), Some(proposal)) else {
        panic!("stale comparison changed the head")
    };
    assert_eq!(*proposal, 3);
    let other = foreign.try_allocate(4).unwrap();
    let Compare::Foreign(Some(other)) = link.compare(None, Some(other)) else {
        panic!("accepted foreign backing")
    };
    assert_eq!(*other, 4);
    let now = link.try_load().unwrap().unwrap();
    assert_eq!(*now, 2);
    assert!(matches!(
        link.compare(Some(&held), None),
        Compare::Unchanged(None)
    ));
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
fn suspended_reader_cannot_acquire_a_replacement_generation() {
    use std::sync::mpsc;
    let pool = Pool::new(1);
    let link = Arc::new(Link::empty(pool.clone()));
    let initial = pool.try_allocate(7_u64).unwrap();
    assert!(matches!(
        link.compare(None, Some(initial)),
        Compare::Changed
    ));
    let (loaded, observed) = mpsc::channel();
    let (resume, wait) = mpsc::channel();
    let reader = {
        let link = link.clone();
        std::thread::spawn(move || {
            let token = Token(link.token.load(Ordering::Acquire));
            loaded.send(()).unwrap();
            wait.recv().unwrap();
            assert!(link.pool.try_acquire(token).is_none());
        })
    };
    observed.recv().unwrap();
    let previous = link.try_load().unwrap().unwrap();
    assert!(matches!(
        link.compare(Some(&previous), None),
        Compare::Changed
    ));
    drop(previous);
    let replacement = pool.try_allocate(99).unwrap();
    assert!(matches!(
        link.compare(None, Some(replacement)),
        Compare::Changed
    ));
    resume.send(()).unwrap();
    reader.join().unwrap();
    assert_eq!(*link.try_load().unwrap().unwrap(), 99);
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
    let spare = match pool.try_allocate_at(0, spare) {
        Err(spare) => spare,
        Ok(_) => panic!("reused slot before destructor finished"),
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

#[cfg(not(target_family = "wasm"))]
#[test]
fn concurrent_owning_loads_and_replacement_balance_destruction() {
    struct Counted {
        value: usize,
        drops: Arc<AtomicUsize>,
    }
    impl Drop for Counted {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }
    let pool = Pool::new(32);
    let link = Arc::new(Link::empty(pool.clone()));
    let drops = Arc::new(AtomicUsize::new(0));
    let initial = pool
        .try_allocate(Counted {
            value: 0,
            drops: drops.clone(),
        })
        .unwrap_or_else(|_| panic!("fresh slot occupied"));
    assert!(matches!(
        link.compare(None, Some(initial)),
        Compare::Changed
    ));
    let iterations = if cfg!(miri) { 8 } else { 1000 };
    std::thread::scope(|scope| {
        for id in 0..4 {
            let pool = &pool;
            let link = &link;
            let drops = &drops;
            scope.spawn(move || {
                for iteration in 0..iterations {
                    let value = Counted {
                        value: 1 + id * iterations + iteration,
                        drops: drops.clone(),
                    };
                    if let Ok(proposed) = pool.try_allocate(value)
                        && let Ok(Some(expected)) = link.try_load()
                    {
                        assert!(expected.value <= 4 * iterations);
                        if let Some(copy) = expected.try_clone() {
                            assert!(Owned::ptr_eq(&copy, &expected));
                            assert_eq!(copy.value, expected.value);
                        }
                        drop(link.compare(Some(&expected), Some(proposed)));
                    }
                }
            });
        }
    });
    drop(link);
    drop(pool);
    assert_eq!(drops.load(Ordering::Relaxed), 1 + 4 * iterations);
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
        if limit != MAX_GENERATION {
            assert_eq!(
                pool.try_allocate_status(99).err().unwrap().error,
                AllocationError::GenerationExhausted
            );
            assert!(pool.generation_exhaustion_observed());
        } else {
            let next = pool.try_allocate_status(2).unwrap();
            assert_eq!(next.token.generation(), 2);
        }
    }
}
