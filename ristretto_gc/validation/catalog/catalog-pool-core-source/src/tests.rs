use super::*;

struct Probe(Arc<AtomicUsize>, usize);
impl Drop for Probe {
    fn drop(&mut self) {
        self.0.fetch_add(self.1, Ordering::Relaxed);
    }
}
fn allocate<T>(pool: &Arc<Pool<T>>, value: T) -> Owned<T> {
    match pool.try_allocate(value) {
        Ok(value) => value,
        Err(_) => panic!("fixture pool occupied"),
    }
}

#[test]
fn references_prevent_reuse_until_exactly_once_destruction() {
    let pool = Pool::new(1);
    let drops = Arc::new(AtomicUsize::new(0));
    let first = allocate(&pool, Probe(drops.clone(), 1));
    let other = first.clone();
    let rejected = pool.try_allocate(Probe(drops.clone(), 100));
    assert!(rejected.is_err());
    drop(rejected);
    drop(first);
    assert_eq!(drops.load(Ordering::Relaxed), 100);
    assert_eq!(pool.occupied(), 1);
    drop(other);
    assert_eq!(drops.load(Ordering::Relaxed), 101);
    assert_eq!(pool.occupied(), 0);
    drop(allocate(&pool, Probe(drops.clone(), 10)));
    assert_eq!(drops.load(Ordering::Relaxed), 111);
}

#[test]
fn a_value_keeps_backing_alive_after_the_external_pool_owner_drops() {
    let pool = Pool::new(1);
    let lifetime = Arc::downgrade(&pool);
    let value = allocate(&pool, 42);
    drop(pool);
    assert!(lifetime.upgrade().is_some());
    assert_eq!(*value, 42);
    drop(value);
    assert!(lifetime.upgrade().is_none());
}

#[test]
fn an_owning_expected_pointer_prevents_reuse_and_stale_cas() {
    let pool = Pool::new(4);
    let initial = allocate(&pool, 1);
    let link = arc_swap::ArcSwapAny::<Owned<_>>::new(initial.clone());
    let second = allocate(&pool, 2);
    let third = allocate(&pool, 3);
    assert!(Owned::ptr_eq(
        &link.compare_and_swap(&initial, second.clone()),
        &initial
    ));
    assert_eq!(**link.compare_and_swap(&initial, third.clone()), 2);
    assert_eq!(**link.load(), 2);
    assert!(pool.try_allocate_at(0, 99).is_err());
    drop(initial);
    let replacement = pool.try_allocate_at(0, 99).ok().unwrap();
    assert_eq!(*replacement, 99);
    assert_eq!(*third, 3);
    drop(link);
    drop(second);
    drop(third);
    drop(replacement);
    assert_eq!(pool.occupied(), 0);
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn destructor_unwind_releases_the_slot_and_aggregate_pool_owner() {
    struct Panics;
    impl Drop for Panics {
        fn drop(&mut self) {
            panic!("intentional destructor failure")
        }
    }
    let pool = Pool::new(1);
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(allocate(
            &pool, Panics
        ))))
        .is_err()
    );
    assert_eq!(pool.occupied(), 0);
    assert_eq!(Arc::strong_count(&pool), 1);
}

#[test]
#[cfg(all(not(target_family = "wasm"), not(miri)))]
fn stopped_destruction_keeps_only_its_slot_unavailable() {
    use std::sync::{Mutex, mpsc};
    use std::time::Duration;
    struct Paused {
        enter: mpsc::Sender<()>,
        resume: Mutex<mpsc::Receiver<()>>,
    }
    impl Drop for Paused {
        fn drop(&mut self) {
            self.enter.send(()).unwrap();
            self.resume
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(10))
                .unwrap();
        }
    }
    let pool = Pool::new(2);
    let (enter, entered) = mpsc::channel();
    let (resume, resumed) = mpsc::channel();
    let value = pool
        .try_allocate_at(
            0,
            Some(Paused {
                enter,
                resume: Mutex::new(resumed),
            }),
        )
        .ok()
        .unwrap();
    let worker = std::thread::spawn(move || drop(value));
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    assert!(pool.try_allocate_at(0, None).is_err());
    for _ in 0..1000 {
        drop(pool.try_allocate_at(1, None).ok().unwrap());
    }
    resume.send(()).unwrap();
    worker.join().unwrap();
    assert_eq!(pool.occupied(), 0);
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn concurrent_owned_loads_and_replacements_preserve_values() {
    let pool = Pool::new(64);
    let link = Arc::new(arc_swap::ArcSwapAny::<Owned<_>>::new(allocate(
        &pool, 0_usize,
    )));
    let mut workers = Vec::new();
    let workers_count = if cfg!(miri) { 2 } else { 4 };
    let iterations = if cfg!(miri) { 3 } else { 500 };
    for worker in 0..workers_count {
        let pool = pool.clone();
        let link = link.clone();
        workers.push(std::thread::spawn(move || {
            for iteration in 0..iterations {
                let value = worker * iterations + iteration;
                let proposed = loop {
                    if let Ok(proposed) = pool.try_allocate(value) {
                        break proposed;
                    }
                };
                let old = link.load_full();
                let observed = link.compare_and_swap(&old, proposed);
                assert!(**observed < workers_count * iterations);
                assert!(*old < workers_count * iterations);
            }
        }));
    }
    for worker in workers {
        worker.join().unwrap();
    }
    drop(link);
    assert_eq!(pool.occupied(), 0);
    assert_eq!(Arc::strong_count(&pool), 1);
}
