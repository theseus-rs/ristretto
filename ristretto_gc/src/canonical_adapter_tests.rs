use super::*;
use crate::{CanonicalPayload, Configuration, Trace};
use gc_catalog::bounded::{Budget, Capacities, ConfigError, RegisterError};
use runtime::{LeafRuntime, RuntimeError, RuntimeFootprint};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

fn heap() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(Configuration::builder().threads(1).build())
}

fn capacities() -> Capacities {
    Capacities::derive(3, 1, 1).unwrap().with_roots(3).unwrap()
}

#[test]
fn leaf_root_reads_scalar_and_rejects_another_runtime() {
    let foreign = heap();
    let heap = heap();
    let budget = Budget::new(usize::MAX);
    let first = LeafRuntime::new(&heap, capacities(), &budget).unwrap();
    let other = LeafRuntime::new(&heap, capacities(), &budget).unwrap();
    let mut worker = first.mutator().unwrap();
    let mut allocation = worker.begin_any(53_u64).unwrap();
    let root = (0..256)
        .find_map(|_| allocation.poll().unwrap())
        .expect("rooted scalar");
    assert_eq!(*first.borrow(&root).unwrap(), 53);
    assert!(matches!(
        root.borrow(&foreign),
        Err(LeafAccessError::Admission(RecordError::Foreign))
    ));
    assert!(matches!(
        other.borrow(&root),
        Err(LeafAccessError::Admission(RecordError::Foreign))
    ));
    assert!(matches!(
        allocation.poll(),
        Err(allocation::AllocationError::Finished)
    ));
    drop(allocation);
    drop(worker);
    drop(first);
    drop(other);
    assert!(budget.used() > 0);
    assert_eq!(heap.managed_bytes(), size_of::<u64>());
    drop(root);
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn wrapper_and_participant_reservations_are_admitted_before_use() {
    let heap = heap();
    let bytes = RuntimeFootprint::for_capacities(capacities())
        .unwrap()
        .total();
    let short = Budget::new(bytes - 1);
    assert!(matches!(
        LeafRuntime::new(&heap, capacities(), &short),
        Err(RuntimeError::Config(ConfigError::BudgetExceeded))
    ));
    assert_eq!(short.used(), 0);
    let exact = Budget::new(bytes);
    let runtime = LeafRuntime::new(&heap, capacities(), &exact).unwrap();
    assert_eq!(runtime.charged_bytes(), bytes);
    let worker = runtime.mutator().unwrap();
    assert!(matches!(runtime.mutator(), Err(RegisterError::Limit)));
    drop(worker);
    assert_eq!(runtime.registered(), (0, 0));
    drop(runtime.mutator().unwrap());
    drop(runtime);
    assert_eq!(exact.used(), 0);
}

struct Counted(Arc<AtomicUsize>);
// SAFETY: this fixture has no managed edges and requires no tracing.
unsafe impl Trace for Counted {
    fn trace(&self, _: &GarbageCollector) {}

    fn may_contain_gc_references() -> bool {
        false
    }

    fn canonical_payload() -> Option<CanonicalPayload<Self>> {
        // SAFETY: the only owned backing is a host counter, with no GC references.
        Some(unsafe { CanonicalPayload::new() })
    }
}
impl Drop for Counted {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn every_finite_allocation_prefix_returns_payload_and_catalog_credit() {
    for prefix in 0..32 {
        let heap = heap();
        let budget = Budget::new(usize::MAX);
        let drops = Arc::new(AtomicUsize::new(0));
        let runtime = LeafRuntime::new(&heap, capacities(), &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut request = worker.begin_any(Counted(drops.clone())).unwrap();
        let mut returned = None;
        for _ in 0..prefix {
            if let Some(root) = request.poll().unwrap() {
                returned = Some(root);
                break;
            }
        }
        drop(request);
        drop(returned);
        drop(worker);
        drop(runtime);
        assert_eq!(drops.load(Ordering::SeqCst), 1, "prefix {prefix}");
        assert_eq!(heap.managed_bytes(), 0, "prefix {prefix}");
        assert_eq!(budget.used(), 0, "prefix {prefix}");
    }
}

#[test]
fn logical_root_aliases_share_one_permit_and_keep_storage() {
    let heap = heap();
    let capacities = Capacities::derive(1, 1, 1).unwrap().with_roots(1).unwrap();
    let budget = Budget::new(usize::MAX);
    let runtime = LeafRuntime::new(&heap, capacities, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut request = worker.begin_any(7_u32).unwrap();
    let root = (0..256)
        .find_map(|_| request.poll().unwrap())
        .expect("one root");
    let aliases = (0..16).map(|_| root.clone()).collect::<Vec<_>>();
    drop(request);
    drop(root);
    for alias in &aliases {
        assert_eq!(*runtime.borrow(alias).unwrap(), 7);
    }
    drop(worker);
    drop(runtime);
    assert!(budget.used() > 0);
    drop(aliases);
    assert_eq!(budget.used(), 0);
    assert_eq!(heap.managed_bytes(), 0);
}

#[test]
fn catalog_root_checks_payload_type_before_borrow() {
    use gc_catalog::bounded::{BoundedCatalog, Progress};
    let heap = heap();
    let budget = Budget::new(usize::MAX);
    let catalog = BoundedCatalog::<LeafRecord>::with_records_in(capacities(), &budget).unwrap();
    let mut worker = catalog.register_record_mutator().unwrap();
    let record = LeafRecord::new(PreparedPayload::new(&heap, 81_u64).unwrap());
    worker
        .submit_rooted_record_any(record.address(), record)
        .map_err(|failure| failure.error)
        .unwrap();
    assert!((0..256).any(|_| matches!(worker.poll_in(&heap), Progress::RootCreated(_))));
    let root = worker.take_root().unwrap();
    assert!(matches!(
        LeafRecord::borrow_root::<i64>(&root, &heap),
        Err(LeafAccessError::WrongType)
    ));
    assert_eq!(*LeafRecord::borrow_root::<u64>(&root, &heap).unwrap(), 81);
    drop(root);
    drop(worker);
    drop(catalog);
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn unproven_or_finalizer_payload_never_enters_catalog() {
    struct Finalized;
    // SAFETY: this fixture has no managed edges and requires no tracing.
    unsafe impl Trace for Finalized {
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl crate::Finalize for Finalized {
        fn finalize(&self) {}
    }
    let heap = heap();
    let budget = Budget::new(usize::MAX);
    let runtime = LeafRuntime::new(&heap, capacities(), &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    assert!(matches!(
        worker.begin_any(Finalized),
        Err(allocation::AllocationError::Unsupported)
    ));
    assert_eq!(heap.managed_bytes(), 0);
    drop(worker);
    drop(runtime);
    assert_eq!(budget.used(), 0);
}
