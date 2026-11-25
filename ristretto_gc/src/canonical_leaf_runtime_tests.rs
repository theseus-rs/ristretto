//! The wrapper API is exercised without borrowing its private catalog/participant.
use super::*;
use crate::canonical_catalog as gc_catalog;
use crate::gc::publication_owner::canonical_leaf::allocation::{LeafAllocation, LeafRoot};
use crate::gc::publication_owner::canonical_leaf::maintenance::{
    LeafMaintenance, MaintenanceError,
};
use crate::gc::publication_owner::canonical_leaf::runtime::{
    LeafMutator, LeafRuntime, RuntimeError, RuntimeFootprint,
};
use gc_catalog::bounded::{ConfigError, RegisterError};
use std::mem::size_of;

fn capacities() -> Capacities {
    Capacities::derive(5, 3, 1).unwrap().with_roots(4).unwrap()
}
fn allocate(
    worker: &mut LeafMutator<'_>,
    slot: usize,
    drops: &Arc<AtomicUsize>,
) -> LeafRoot<Value> {
    let mut request = worker
        .begin(
            slot,
            slot,
            Value {
                number: slot as u64,
                drops: drops.clone(),
            },
        )
        .unwrap();
    (0..128)
        .find_map(|_| request.poll().unwrap())
        .expect("runtime allocation")
}
fn reclaim(helper: &mut LeafMaintenance<'_>, drops: &Arc<AtomicUsize>, expected: usize) {
    for _ in 0..50_000 {
        if drops.load(Ordering::Relaxed) == expected {
            return;
        }
        helper.poll().unwrap();
    }
    assert_eq!(
        drops.load(Ordering::Relaxed),
        expected,
        "runtime cleanup did not settle"
    );
}

#[test]
fn setup_reserves_wrapper_storage_and_rejects_one_byte_less() {
    let gc = collector(true, RootStorage::Contexts);
    let capacities = capacities();
    let footprint = RuntimeFootprint::for_capacities(capacities).unwrap();
    let owned_layouts = size_of::<LeafRuntime<'_>>()
        + 3 * (size_of::<LeafMutator<'_>>() + size_of::<LeafAllocation<'_, ()>>())
        + size_of::<LeafMaintenance<'_>>()
        + 4 * size_of::<LeafRoot<()>>();
    assert!(footprint.total() >= footprint.catalog().backing() + owned_layouts);
    assert!(footprint.runtime() > 0);
    let short = Budget::new(footprint.total() - 1);
    assert!(matches!(
        LeafRuntime::new(&gc, capacities, &short),
        Err(RuntimeError::Config(ConfigError::BudgetExceeded))
    ));
    assert_eq!(short.used(), 0);
    let budget = Budget::new(footprint.total());
    let runtime = LeafRuntime::new(&gc, capacities, &budget).unwrap();
    assert_eq!(runtime.charged_bytes(), footprint.total());
    assert_eq!(budget.used(), footprint.total());
    assert_eq!(gc.managed_bytes(), 0);
    drop(runtime);
    assert_eq!(budget.used(), 0);
}

#[test]
fn runtime_registration_obeys_prepaid_mutator_and_helper_quotas() {
    let gc = collector(true, RootStorage::Contexts);
    let footprint = RuntimeFootprint::for_capacities(capacities()).unwrap();
    let budget = Budget::new(footprint.total());
    let runtime = LeafRuntime::new(&gc, capacities(), &budget).unwrap();
    let workers = (0..3)
        .map(|_| runtime.mutator().unwrap())
        .collect::<Vec<_>>();
    let helper = runtime.maintenance().unwrap();
    assert_eq!(runtime.registered(), (3, 1));
    assert!(matches!(runtime.mutator(), Err(RegisterError::Limit)));
    assert!(matches!(
        runtime.maintenance(),
        Err(MaintenanceError::Register(RegisterError::Limit))
    ));
    assert_eq!(budget.used(), footprint.total());
    drop(workers);
    drop(helper);
    assert_eq!(runtime.registered(), (0, 0));
    let replacement = runtime.mutator().unwrap();
    drop(runtime);
    assert_eq!(budget.used(), footprint.total());
    drop(replacement);
    assert_eq!(budget.used(), 0);
}

#[test]
fn runtime_reads_reject_a_different_catalog_even_on_the_same_collector() {
    let gc = collector(true, RootStorage::Contexts);
    let footprint = RuntimeFootprint::for_capacities(capacities()).unwrap();
    let budget = Budget::new(2 * footprint.total());
    let first = LeafRuntime::new(&gc, capacities(), &budget).unwrap();
    let second = LeafRuntime::new(&gc, capacities(), &budget).unwrap();
    let mut worker = first.mutator().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let root = allocate(&mut worker, 0, &drops);
    assert_eq!(first.borrow(&root).unwrap().number, 0);
    assert!(matches!(
        second.borrow(&root),
        Err(LeafAccessError::Admission(RecordError::Foreign))
    ));
    drop(first);
    drop(worker);
    assert_eq!(budget.used(), 2 * footprint.total());
    drop(root);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(budget.used(), footprint.total());
    drop(second);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn participants_keep_storage_charged_and_can_collect_after_runtime_owner_drop() {
    let gc = collector(true, RootStorage::Contexts);
    let footprint = RuntimeFootprint::for_capacities(capacities()).unwrap();
    let budget = Budget::new(footprint.total());
    let runtime = LeafRuntime::new(&gc, capacities(), &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    drop(runtime);
    for expected in 1..=4 {
        drop(allocate(&mut worker, 0, &drops));
        reclaim(&mut helper, &drops, expected);
        assert_eq!(budget.used(), footprint.total());
    }
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    assert_eq!(budget.used(), footprint.total());
    drop(helper);
    assert_eq!(budget.used(), 0);
}

struct ObserveCharge {
    budget: Arc<Budget>,
    total: usize,
    drops: Arc<AtomicUsize>,
}
// SAFETY: independent budget/counter owners, with no managed edges.
unsafe impl Trace for ObserveCharge {
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for ObserveCharge {
    fn drop(&mut self) {
        assert_eq!(
            self.budget.used(),
            self.total,
            "runtime charge refunded during payload destruction"
        );
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn a_typed_root_retains_the_full_runtime_charge_after_heap_shutdown() {
    let footprint = RuntimeFootprint::for_capacities(capacities()).unwrap();
    let budget = Budget::new(footprint.total());
    let drops = Arc::new(AtomicUsize::new(0));
    let root = {
        let gc = collector(true, RootStorage::Contexts);
        let runtime = LeafRuntime::new(&gc, capacities(), &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut request = worker
            .begin(
                0,
                0,
                ObserveCharge {
                    budget: budget.clone(),
                    total: footprint.total(),
                    drops: drops.clone(),
                },
            )
            .unwrap();
        let root = (0..128).find_map(|_| request.poll().unwrap()).unwrap();
        assert_eq!(runtime.borrow(&root).unwrap().total, footprint.total());
        root
    };
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(budget.used(), footprint.total());
    let alias = root.clone();
    drop(root);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(budget.used(), footprint.total());
    drop(alias);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(budget.used(), 0);
}

#[test]
fn runtime_rejects_conditional_reference_capacity_before_reserving_storage() {
    let gc = collector(true, RootStorage::Contexts);
    let budget = Budget::new(usize::MAX);
    let capacities = capacities().with_references(1).unwrap();
    assert_eq!(
        RuntimeFootprint::for_capacities(capacities),
        Err(RuntimeError::ConditionalReferences)
    );
    assert!(matches!(
        LeafRuntime::new(&gc, capacities, &budget),
        Err(RuntimeError::ConditionalReferences)
    ));
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;
    use std::sync::{Mutex, mpsc};
    use std::time::Duration;

    struct Release(mpsc::Sender<()>);
    impl Drop for Release {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }
    struct BlockingDrop {
        entered: mpsc::Sender<()>,
        resume: Mutex<mpsc::Receiver<()>>,
        drops: Arc<AtomicUsize>,
    }
    // SAFETY: independent host channels/counter only; no managed edges. This
    // deliberately demonstrates an unbounded destructor, not a bounded callback.
    // SAFETY: this fixture has no managed edges and requires no tracing.
    unsafe impl Trace for BlockingDrop {
        fn may_contain_gc_references() -> bool {
            false
        }
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl Drop for BlockingDrop {
        fn drop(&mut self) {
            self.entered.send(()).unwrap();
            self.resume
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(60))
                .unwrap();
            self.drops.fetch_add(1, Ordering::Relaxed);
        }
    }
    fn blocking_root(worker: &mut LeafMutator<'_>, value: BlockingDrop) -> LeafRoot<BlockingDrop> {
        let mut request = worker.begin(0, 0, value).unwrap();
        (0..128).find_map(|_| request.poll().unwrap()).unwrap()
    }

    #[test]
    fn stopped_helper_destruction_retains_runtime_credit_after_other_owners_leave() {
        let gc = collector(true, RootStorage::Contexts);
        let capacities = Capacities::derive(3, 1, 2).unwrap().with_roots(2).unwrap();
        let footprint = RuntimeFootprint::for_capacities(capacities).unwrap();
        let budget = Budget::new(footprint.total());
        let runtime = LeafRuntime::new(&gc, capacities, &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut first = runtime.maintenance().unwrap();
        let mut second = runtime.maintenance().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        let other_drops = Arc::new(AtomicUsize::new(0));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        drop(blocking_root(
            &mut worker,
            BlockingDrop {
                entered: entered_tx,
                resume: Mutex::new(resume_rx),
                drops: drops.clone(),
            },
        ));
        let payload_credit = gc.managed_bytes();
        drop(runtime);
        std::thread::scope(|scope| {
            // The guard must unwind before the scope waits for its child.
            let release = Release(resume_tx);
            let first_drops = drops.clone();
            let stopped = scope.spawn(move || reclaim(&mut first, &first_drops, 1));
            entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
            drop(allocate(&mut worker, 1, &other_drops));
            reclaim(&mut second, &other_drops, 1);
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            assert_eq!(budget.used(), footprint.total());
            assert_eq!(gc.managed_bytes(), payload_credit);
            drop(worker);
            drop(second);
            assert_eq!(budget.used(), footprint.total());
            drop(release);
            stopped.join().unwrap();
        });
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(budget.used(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }

    #[test]
    fn last_root_teardown_refunds_runtime_storage_only_after_payload_destruction() {
        let gc = collector(true, RootStorage::Contexts);
        let capacities = Capacities::derive(1, 1, 1).unwrap().with_roots(1).unwrap();
        let footprint = RuntimeFootprint::for_capacities(capacities).unwrap();
        let budget = Budget::new(footprint.total());
        let runtime = LeafRuntime::new(&gc, capacities, &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();
        let root = blocking_root(
            &mut worker,
            BlockingDrop {
                entered: entered_tx,
                resume: Mutex::new(resume_rx),
                drops: drops.clone(),
            },
        );
        let payload_credit = gc.managed_bytes();
        drop(worker);
        drop(runtime);
        let alias = root.clone();
        drop(root);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(budget.used(), footprint.total());
        std::thread::scope(|scope| {
            let release = Release(resume_tx);
            let stopped = scope.spawn(move || drop(alias));
            entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            assert_eq!(budget.used(), footprint.total());
            assert_eq!(gc.managed_bytes(), payload_credit);
            drop(release);
            stopped.join().unwrap();
        });
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(budget.used(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
}
