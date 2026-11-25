//! Automatic allocations retain unique real payloads through bounded polling.
use super::*;
use crate::gc::publication_owner::canonical_leaf::allocation::{LeafAllocation, LeafRoot};
use crate::gc::publication_owner::canonical_leaf::runtime::{LeafMutator, LeafRuntime};
fn retry_collector() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(
        Configuration::builder()
            .concurrent_publication(true)
            .root_storage(RootStorage::Contexts)
            .diagnostics(true)
            .allocation_threshold(usize::MAX)
            .compact_heap(true)
            .build(),
    )
}
struct Counted {
    value: Value,
    prepares: Arc<AtomicUsize>,
}
// SAFETY: owns a number and independently owned counters, with no managed edges.
unsafe impl Trace for Counted {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
    fn prepare(&mut self, _: &crate::region::RegionHeap) -> crate::Result<()> {
        self.prepares.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
fn value(number: u64, drops: &Arc<AtomicUsize>, prepares: &Arc<AtomicUsize>) -> Counted {
    Counted {
        value: Value {
            number,
            drops: drops.clone(),
        },
        prepares: prepares.clone(),
    }
}
fn finish(request: &mut LeafAllocation<'_, Counted>) -> LeafRoot<Counted> {
    (0..256)
        .find_map(|_| request.poll().unwrap())
        .expect("allocation settled")
}
fn allocate(worker: &mut LeafMutator<'_>, v: Counted) -> LeafRoot<Counted> {
    finish(&mut worker.begin_any(v).unwrap())
}
#[test]
fn full_object_array_retains_payload_until_helper_reclaims_space() {
    let gc = retry_collector();
    let caps = Capacities::derive(2, 1, 1).unwrap().with_roots(3).unwrap();
    let budget = Budget::new(usize::MAX);
    let runtime = LeafRuntime::new(&gc, caps, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let prepares = Arc::new(AtomicUsize::new(0));
    let keeper = allocate(&mut worker, value(1, &drops, &prepares));
    let old = allocate(&mut worker, value(2, &drops, &prepares));
    let mut waiting = worker.begin_any(value(3, &drops, &prepares)).unwrap();
    let charged = gc.managed_bytes();
    for _ in 0..64 {
        assert!(waiting.poll().unwrap().is_none());
    }
    assert_eq!(prepares.load(Ordering::Relaxed), 3);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(gc.managed_bytes(), charged);
    drop(old);
    let replacement = (0..50_000)
        .find_map(|_| {
            helper.poll().unwrap();
            waiting.poll().unwrap()
        })
        .expect("helper freed a discovery slot");
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(prepares.load(Ordering::Relaxed), 3);
    assert_eq!(runtime.borrow(&keeper).unwrap().value.number, 1);
    assert_eq!(runtime.borrow(&replacement).unwrap().value.number, 3);
    drop(waiting);
    drop((keeper, replacement));
    for _ in 0..50_000 {
        if drops.load(Ordering::Relaxed) == 3 {
            break;
        }
        helper.poll().unwrap();
    }
    assert_eq!(drops.load(Ordering::Relaxed), 3);
    assert_eq!(gc.managed_bytes(), 0);
    drop((runtime, worker, helper));
    assert_eq!(budget.used(), 0);
}
#[test]
fn stale_root_location_stays_pending_until_helper_finishes_removal() {
    let gc = retry_collector();
    let caps = Capacities::derive(3, 1, 1).unwrap().with_roots(1).unwrap();
    let runtime = LeafRuntime::new(&gc, caps, &Budget::new(usize::MAX)).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let prepares = Arc::new(AtomicUsize::new(0));
    drop(allocate(&mut worker, value(1, &drops, &prepares)));
    let mut waiting = worker.begin_any(value(2, &drops, &prepares)).unwrap();
    for _ in 0..64 {
        assert!(waiting.poll().unwrap().is_none());
    }
    assert_eq!(prepares.load(Ordering::Relaxed), 2);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    let root = (0..50_000)
        .find_map(|_| {
            helper.poll().unwrap();
            waiting.poll().unwrap()
        })
        .expect("physical root reclaimed");
    assert_eq!(runtime.borrow(&root).unwrap().value.number, 2);
    assert_eq!(prepares.load(Ordering::Relaxed), 2);
    drop(waiting);
    drop((root, worker, helper, runtime));
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(gc.managed_bytes(), 0);
}
#[test]
fn cancellation_of_waiting_allocations_retains_no_payload_or_quota_leak() {
    // Two fixtures distinguish a private payload from an announced cell waiting
    // before its root is created. The latter needs durable helper cleanup.
    for full_objects in [true, false] {
        let gc = retry_collector();
        let caps = Capacities::derive(if full_objects { 1 } else { 2 }, 1, 1)
            .unwrap()
            .with_roots(2)
            .unwrap();
        let budget = Budget::new(usize::MAX);
        let runtime = LeafRuntime::new(&gc, caps, &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut helper = runtime.maintenance().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        let prepares = Arc::new(AtomicUsize::new(0));
        let keeper = allocate(&mut worker, value(1, &drops, &prepares));
        let mut waiting = worker.begin_any(value(2, &drops, &prepares)).unwrap();
        for _ in 0..if full_objects { 32 } else { 4 } {
            assert!(waiting.poll().unwrap().is_none());
        }
        drop(waiting);
        for _ in 0..50_000 {
            if drops.load(Ordering::Relaxed) == 1 {
                break;
            }
            helper.poll().unwrap();
        }
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(runtime.borrow(&keeper).unwrap().value.number, 1);
        assert_eq!(prepares.load(Ordering::Relaxed), 2);
        drop(keeper);
        for _ in 0..50_000 {
            if drops.load(Ordering::Relaxed) == 2 {
                break;
            }
            helper.poll().unwrap();
        }
        assert_eq!(drops.load(Ordering::Relaxed), 2);
        // Reusing the same participant proves cancellation returned its quota.
        drop(allocate(&mut worker, value(3, &drops, &prepares)));
        drop((worker, helper, runtime));
        assert_eq!(drops.load(Ordering::Relaxed), 3);
        assert_eq!(budget.used(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
}
#[test]
fn automatic_allocation_reuses_slots_through_repeated_helper_cycles() {
    let gc = retry_collector();
    let caps = Capacities::derive(7, 1, 1).unwrap().with_roots(7).unwrap();
    let runtime = LeafRuntime::new(&gc, caps, &Budget::new(usize::MAX)).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let prepares = Arc::new(AtomicUsize::new(0));
    for round in 0..4 {
        for item in 0..7 {
            let number = (round * 7 + item) as u64;
            let root = allocate(&mut worker, value(number, &drops, &prepares));
            assert_eq!(runtime.borrow(&root).unwrap().value.number, number);
        }
        for _ in 0..50_000 {
            if drops.load(Ordering::Relaxed) == (round + 1) * 7 {
                break;
            }
            helper.poll().unwrap();
        }
        assert_eq!(drops.load(Ordering::Relaxed), (round + 1) * 7);
    }
    assert_eq!(prepares.load(Ordering::Relaxed), 28);
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn full_root_quota_retains_request_until_an_existing_root_is_dropped() {
    let gc = retry_collector();
    let caps = Capacities::derive(3, 1, 1).unwrap().with_roots(1).unwrap();
    let runtime = LeafRuntime::new(&gc, caps, &Budget::new(usize::MAX)).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let prepares = Arc::new(AtomicUsize::new(0));
    let keeper = allocate(&mut worker, value(1, &drops, &prepares));
    let mut waiting = worker.begin_any(value(2, &drops, &prepares)).unwrap();
    for _ in 0..64 {
        helper.poll().unwrap();
        assert!(waiting.poll().unwrap().is_none());
    }
    assert_eq!(runtime.borrow(&keeper).unwrap().value.number, 1);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(prepares.load(Ordering::Relaxed), 2);
    drop(keeper);
    let root = (0..50_000)
        .find_map(|_| {
            helper.poll().unwrap();
            waiting.poll().unwrap()
        })
        .expect("root permit and discovery location became available");
    assert_eq!(runtime.borrow(&root).unwrap().value.number, 2);
    assert_eq!(prepares.load(Ordering::Relaxed), 2);
    drop(waiting);
    drop((root, worker, helper, runtime));
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(gc.managed_bytes(), 0);
}
