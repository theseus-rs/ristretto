use super::*;

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
