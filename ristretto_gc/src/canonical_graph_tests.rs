use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Value {
    identity: u64,
    drops: Arc<AtomicUsize>,
}
// SAFETY: no payload managed references; graph edges exist only in the inventory.
impl Trace for Value {
    fn canonical_payload() -> Option<crate::CanonicalPayload<Self>> {
        // SAFETY: only scalar data and a host counter; managed edges are in the catalog.
        Some(unsafe { crate::CanonicalPayload::new() })
    }
    fn may_contain_gc_references() -> bool {
        false
    }
    fn trace(&self, _: &GarbageCollector) {}
}
impl Drop for Value {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
fn capacities() -> Capacities {
    Capacities::derive(4, 3, 1)
        .unwrap()
        .with_roots(12)
        .unwrap()
        .with_edges(8)
        .unwrap()
}
fn setup(gc: &GarbageCollector) -> (GraphRuntime<'_>, Arc<Budget>, Arc<AtomicUsize>) {
    let budget = Budget::new(GraphFootprint::new(capacities(), 2).unwrap().total());
    let runtime = GraphRuntime::new(gc, capacities(), 2, &budget).unwrap();
    (runtime, budget, Arc::new(AtomicUsize::new(0)))
}
fn alloc(
    worker: &mut GraphMutator<'_>,
    helper: &mut LeafMaintenance<'_>,
    drops: &Arc<AtomicUsize>,
    identity: u64,
) -> GraphRoot<Value> {
    let mut request = worker
        .allocate(Value {
            identity,
            drops: drops.clone(),
        })
        .unwrap();
    for _ in 0..50_000 {
        if let Some(root) = request.poll().unwrap() {
            return root;
        }
        helper.poll().unwrap();
    }
    panic!("graph allocation did not settle");
}
fn collect(helper: &mut LeafMaintenance<'_>, drops: &Arc<AtomicUsize>, expected: usize) {
    for _ in 0..100_000 {
        helper.poll().unwrap();
        if drops.load(Ordering::Relaxed) == expected {
            return;
        }
    }
    assert_eq!(
        drops.load(Ordering::Relaxed),
        expected,
        "real graph values not reclaimed"
    );
}

#[test]
fn graph_allocation_drop_at_every_poll_prefix_cleans_partial_inventory() {
    let prefixes: Vec<usize> = if cfg!(miri) {
        vec![0, 1, 3, 7, 12, 18, 24, 32, 48, 64]
    } else {
        (0..70).collect()
    };
    let mut stages = std::collections::BTreeSet::new();
    for prefix in prefixes {
        let gc = GarbageCollector::new();
        let (runtime, budget, drops) = setup(&gc);
        let mut worker = runtime.mutator().unwrap();
        let mut helper = runtime.maintenance().unwrap();
        let mut result = None;
        {
            let mut request = worker
                .allocate(Value {
                    identity: 7,
                    drops: drops.clone(),
                })
                .unwrap();
            for _ in 0..prefix {
                result = request.poll().unwrap();
                if result.is_some() {
                    break;
                }
                helper.poll().unwrap();
            }
            stages.insert(match request.stage {
                AllocationStage::Object => "object",
                AllocationStage::Create => "create",
                AllocationStage::Creating => "creating",
                AllocationStage::Publish => "publish",
                AllocationStage::Publishing => "publishing",
                AllocationStage::Handoff => "handoff",
            });
        }
        drop(result);
        collect(&mut helper, &drops, 1);
        for _ in 0..2000 {
            helper.poll().unwrap();
        }
        for slot in 0..8 {
            assert!(
                worker.worker.discover_edge(slot).unwrap().is_none(),
                "cancelled inventory at prefix {prefix}"
            );
        }
        assert_eq!(runtime.catalog.reserved_root_tokens(), 0);
        assert_eq!(runtime.catalog.reserved_edge_tokens(), 0);
        drop(worker);
        drop(helper);
        drop(runtime);
        assert_eq!(budget.used(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
    if !cfg!(miri) {
        assert_eq!(
            stages,
            std::collections::BTreeSet::from([
                "object",
                "create",
                "creating",
                "publish",
                "publishing",
                "handoff"
            ])
        );
    }
}

#[test]
fn zero_edge_payload_and_excessive_shape_keep_admission_exact() {
    let heap = GarbageCollector::new();
    let (runtime, budget, _) = setup(&heap);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    assert!(matches!(
        worker.allocate_with_edges(1_u64, 3),
        Err(GraphError::Layout)
    ));
    assert_eq!(heap.managed_bytes(), 0);
    let root = {
        let mut request = worker.allocate_with_edges(7_u64, 0).unwrap();
        (0..512)
            .find_map(|_| {
                helper.poll().unwrap();
                request.poll().unwrap()
            })
            .expect("zero-edge payload")
    };
    assert_eq!(root.root.borrow_live_record().unwrap().graph_edges, 0);
    for slot in 0..8 {
        assert!(worker.worker.discover_edge(slot).unwrap().is_none());
    }
    drop(root);
    assert!((0..20_000).any(|_| {
        helper.poll().unwrap();
        heap.managed_bytes() == 0
    }));
    drop((worker, helper, runtime));
    assert_eq!(budget.used(), 0);
}
