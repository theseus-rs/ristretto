use super::*;
use gc_catalog::{DomainMode, EdgeMembership};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Value {
    identity: u64,
    drops: Arc<AtomicUsize>,
}
// SAFETY: no payload managed references; graph edges exist only in the inventory.
unsafe impl Trace for Value {
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
fn write(
    worker: &mut GraphMutator<'_>,
    helper: &mut LeafMaintenance<'_>,
    source: &GraphRoot<Value>,
    target: Option<&GraphRoot<Value>>,
) {
    let mut request = worker.write(source, 0, target).unwrap();
    for _ in 0..50_000 {
        if request.poll().unwrap() {
            return;
        }
        helper.poll().unwrap();
    }
    panic!("graph write did not settle");
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
fn replacing_an_edge_during_marking_preserves_the_reachable_target() {
    let heap = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&heap);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let source = alloc(&mut worker, &mut helper, &drops, 1);
    let old = alloc(&mut worker, &mut helper, &drops, 2);
    let target = alloc(&mut worker, &mut helper, &drops, 3);
    write(&mut worker, &mut helper, &source, Some(&old));
    assert!((0..20_000).any(|_| {
        helper.poll().unwrap();
        worker.worker.phase_in(&heap).unwrap().domains[0].mode == DomainMode::Full
    }));
    write(&mut worker, &mut helper, &source, Some(&target));
    let target_key = target.root.observe().target;
    drop((old, target));
    collect(&mut helper, &drops, 1);
    let edge = worker.observe(&source.root, 0).unwrap();
    assert!(
        matches!(edge.state, EdgeMembership::Live(state) if state.target.is_some_and(|key| key.identity == target_key.identity()))
    );
    for _ in 0..5000 {
        helper.poll().unwrap();
    }
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(runtime.borrow(&source).unwrap().identity, 1);
    write(&mut worker, &mut helper, &source, None);
    collect(&mut helper, &drops, 2);
    drop(source);
    collect(&mut helper, &drops, 3);
    drop((worker, helper, runtime));
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn stale_write_cannot_change_a_reused_edge_slot() {
    let heap = GarbageCollector::new();
    let caps = Capacities::derive(1, 1, 1)
        .unwrap()
        .with_roots(1)
        .unwrap()
        .with_edges(1)
        .unwrap();
    let budget = Budget::new(usize::MAX);
    let runtime = GraphRuntime::new(&heap, caps, 1, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let old = alloc(&mut worker, &mut helper, &drops, 1);
    let stale = worker.observe(&old.root, 0).unwrap();
    drop(old);
    collect(&mut helper, &drops, 1);
    let current = alloc(&mut worker, &mut helper, &drops, 2);
    let before = worker.observe(&current.root, 0).unwrap();
    assert_ne!(before.key, stale.key);
    worker
        .worker
        .submit_edge(EdgeCommand::Replace {
            edge: stale.key,
            source: current.root.key(),
            target: Some(current.root.key()),
        })
        .unwrap();
    let result = (0..256)
        .map(|_| worker.worker.poll_in(&heap))
        .find(|progress| *progress != Progress::Pending)
        .unwrap();
    assert!(matches!(
        result,
        Progress::Rejected(RequestError::Stale | RequestError::Conflict)
    ));
    assert_eq!(worker.observe(&current.root, 0).unwrap(), before);
    drop(current);
    collect(&mut helper, &drops, 2);
    drop((worker, helper, runtime));
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn edge_shape_and_foreign_targets_are_rejected_before_submission() {
    let heap = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&heap);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    assert!(matches!(
        worker.allocate_with_edges(1_u64, 3),
        Err(GraphError::Layout)
    ));
    assert_eq!(heap.managed_bytes(), 0);
    let source = alloc(&mut worker, &mut helper, &drops, 1);
    assert!(matches!(
        worker.write::<Value, Value>(&source, 2, None),
        Err(GraphError::Index)
    ));
    let (foreign, other_budget, other_drops) = setup(&heap);
    let mut other_worker = foreign.mutator().unwrap();
    let mut other_helper = foreign.maintenance().unwrap();
    let target = alloc(&mut other_worker, &mut other_helper, &other_drops, 2);
    assert!(matches!(
        worker.write(&source, 0, Some(&target)),
        Err(GraphError::Foreign)
    ));
    drop((source, target));
    collect(&mut helper, &drops, 1);
    collect(&mut other_helper, &other_drops, 1);
    drop((worker, helper, runtime, other_worker, other_helper, foreign));
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
    assert_eq!(other_budget.used(), 0);
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

fn child(
    worker: &mut GraphMutator<'_>,
    helper: &mut LeafMaintenance<'_>,
    source: &GraphRoot<Value>,
) -> GraphRoot<Value> {
    let mut request = worker.child::<Value, Value>(source, 0).unwrap();
    for _ in 0..50_000 {
        if let Some(value) = request.poll().unwrap() {
            return match value {
                GraphReadValue::Root(root) => root,
                GraphReadValue::Null => panic!("missing child"),
            };
        }
        helper.poll().unwrap();
    }
    panic!("graph read did not settle");
}

#[test]
fn graph_chain_retains_real_children_and_reads_them_through_new_roots() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let a = alloc(&mut worker, &mut helper, &drops, 11);
    let b = alloc(&mut worker, &mut helper, &drops, 22);
    let c = alloc(&mut worker, &mut helper, &drops, 33);
    for source in [&a, &b, &c] {
        for index in 0..2 {
            let observed = (0..16).find_map(|_| match worker.observe(&source.root, index) {
                Ok(value) => Some(value),
                Err(RequestError::Contended) => None,
                error => panic!("returned graph omitted source inventory: {error:?}"),
            });
            assert!(
                observed.is_some(),
                "returned graph omitted source inventory"
            );
        }
    }
    write(&mut worker, &mut helper, &a, Some(&b));
    write(&mut worker, &mut helper, &b, Some(&c));
    drop(b);
    drop(c);
    for _ in 0..5000 {
        helper.poll().unwrap();
    }
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    let b = child(&mut worker, &mut helper, &a);
    let c = child(&mut worker, &mut helper, &b);
    assert_eq!(runtime.borrow(&a).unwrap().identity, 11);
    assert_eq!(runtime.borrow(&b).unwrap().identity, 22);
    assert_eq!(runtime.borrow(&c).unwrap().identity, 33);
    write(&mut worker, &mut helper, &a, None);
    drop(a);
    drop(b);
    drop(c);
    collect(&mut helper, &drops, 3);
    for _ in 0..5000 {
        helper.poll().unwrap();
    }
    assert_eq!(runtime.catalog.reserved_root_tokens(), 0);
    assert_eq!(runtime.catalog.reserved_edge_tokens(), 0);
    drop(worker);
    drop(helper);
    assert_eq!(runtime.registered(), (0, 0));
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn graph_null_wrong_type_foreign_catalog_and_layout_are_checked() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let (other, other_budget, _) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let a = alloc(&mut worker, &mut helper, &drops, 1);
    let b = alloc(&mut worker, &mut helper, &drops, 2);
    assert!(matches!(other.borrow(&a), Err(GraphError::Foreign)));
    assert!(matches!(
        worker.child::<Value, Value>(&a, 2),
        Err(GraphError::Index)
    ));
    {
        let mut read = worker.child::<Value, u64>(&a, 1).unwrap();
        let outcome = (0..100).find_map(|_| read.poll().unwrap()).unwrap();
        assert!(matches!(outcome, GraphReadValue::Null));
        assert!(matches!(read.poll(), Err(GraphError::Finished)));
    }
    write(&mut worker, &mut helper, &a, Some(&b));
    {
        let mut read = worker.child::<Value, u64>(&a, 0).unwrap();
        let mut rejected = false;
        for _ in 0..2000 {
            match read.poll() {
                Err(GraphError::Access(LeafAccessError::WrongType)) => {
                    rejected = true;
                    break;
                }
                Ok(None) => {}
                _ => panic!("wrong typed child escaped"),
            }
            helper.poll().unwrap();
        }
        assert!(rejected);
    }
    assert!(matches!(
        GraphFootprint::new(capacities(), 0),
        Err(GraphError::Layout)
    ));
    assert!(matches!(
        GraphFootprint::new(capacities(), 3),
        Err(GraphError::Layout)
    ));
    let short = Budget::new(GraphFootprint::new(capacities(), 2).unwrap().total() - 1);
    assert!(matches!(
        GraphRuntime::new(&gc, capacities(), 2, &short),
        Err(GraphError::Config(ConfigError::BudgetExceeded))
    ));
    assert_eq!(short.used(), 0);
    drop(a);
    drop(b);
    collect(&mut helper, &drops, 2);
    drop(worker);
    drop(helper);
    drop(runtime);
    drop(other);
    assert_eq!(budget.used(), 0);
    assert_eq!(other_budget.used(), 0);
}

#[test]
fn graph_dropped_child_reads_leave_no_root_and_preserve_reachable_target() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let a = alloc(&mut worker, &mut helper, &drops, 1);
    let b = alloc(&mut worker, &mut helper, &drops, 2);
    write(&mut worker, &mut helper, &a, Some(&b));
    drop(b);
    let prefixes: Vec<usize> = if cfg!(miri) {
        vec![0, 1, 3, 5, 8, 12, 16, 24]
    } else {
        (0..30).collect()
    };
    for prefix in prefixes {
        {
            let mut read = worker.child::<Value, Value>(&a, 0).unwrap();
            for _ in 0..prefix {
                if read.poll().unwrap().is_some() {
                    break;
                }
                helper.poll().unwrap();
            }
        }
        for _ in 0..1000 {
            helper.poll().unwrap();
        }
        assert_eq!(
            runtime.catalog.reserved_root_tokens(),
            1,
            "child root leaked at prefix {prefix}"
        );
        assert_eq!(drops.load(Ordering::Relaxed), 0);
    }
    let read = child(&mut worker, &mut helper, &a);
    assert_eq!(runtime.borrow(&read).unwrap().identity, 2);
    write(&mut worker, &mut helper, &a, None);
    for _ in 0..1000 {
        helper.poll().unwrap();
    }
    assert_eq!(
        runtime.borrow(&read).unwrap().identity,
        2,
        "independent child root lost on unlink"
    );
    drop(read);
    drop(a);
    collect(&mut helper, &drops, 2);
    drop(worker);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
}

#[test]
fn graph_child_activation_revalidates_an_edge_replaced_after_root_preparation() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut reader = runtime.mutator().unwrap();
    let mut writer = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let a = alloc(&mut reader, &mut helper, &drops, 1);
    let b = alloc(&mut reader, &mut helper, &drops, 2);
    let c = alloc(&mut reader, &mut helper, &drops, 3);
    write(&mut writer, &mut helper, &a, Some(&b));
    let mut request = reader.child::<Value, Value>(&a, 0).unwrap();
    for _ in 0..1000 {
        if request.waiting_for_activation() {
            break;
        }
        assert!(request.poll().unwrap().is_none());
        helper.poll().unwrap();
    }
    assert!(request.waiting_for_activation());
    write(&mut writer, &mut helper, &a, Some(&c));
    let mut result = None;
    for _ in 0..10_000 {
        if let Some(value) = request.poll().unwrap() {
            result = Some(value);
            break;
        }
        helper.poll().unwrap();
    }
    let GraphReadValue::Root(child) = result.expect("replacement read did not settle") else {
        panic!("nonnull child missing");
    };
    assert_eq!(
        runtime.borrow(&child).unwrap().identity,
        3,
        "child read activated a replaced target"
    );
    drop(request);
    drop(child);
    drop(a);
    drop(b);
    drop(c);
    collect(&mut helper, &drops, 3);
    drop(reader);
    drop(writer);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
}

#[test]
fn unused_root_backing_survives_null_reads_and_cancellation_until_handoff() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let parent = alloc(&mut worker, &mut helper, &drops, 1);
    let target = alloc(&mut worker, &mut helper, &drops, 2);
    assert!(worker.prepared.is_none());
    {
        let mut read = worker.child::<Value, Value>(&parent, 0).unwrap();
        assert!(matches!(read.poll(), Ok(Some(GraphReadValue::Null))));
    }
    assert!(worker.prepared.is_some());
    drop(worker.child::<Value, Value>(&parent, 0).unwrap());
    assert!(worker.prepared.is_some());
    write(&mut worker, &mut helper, &parent, Some(&target));
    let result = child(&mut worker, &mut helper, &parent);
    assert_eq!(result.root.observe().target, target.root.observe().target);
    assert!(worker.prepared.is_none());
    drop((result, parent, target, worker));
    collect(&mut helper, &drops, 2);
    drop((helper, runtime));
    assert_eq!(budget.used(), 0);
}

#[test]
fn graph_footprint_covers_the_largest_exclusive_request_and_owning_wrappers() {
    let capacities = capacities();
    let footprint = GraphFootprint::new(capacities, 2).unwrap();
    let request = size_of::<GraphAllocation<'_, '_, Value>>()
        .max(size_of::<GraphRead<'_, '_, Value>>())
        .max(size_of::<GraphWrite<'_, '_>>());
    let owners = size_of::<GraphRuntime<'_>>()
        + capacities.mutators() * (size_of::<GraphMutator<'_>>() + request)
        + capacities.helpers() * size_of::<LeafMaintenance<'_>>()
        + capacities.root_slots() * size_of::<GraphRoot<Value>>()
        + capacities.edge_slots() * size_of::<EdgeHandle<LeafRecord>>();
    assert!(
        footprint.total() >= footprint.catalog.backing() + owners,
        "graph request ownership is not fully prepaid"
    );
}

#[test]
fn graph_null_replacement_completes_a_read_with_every_root_permit_occupied() {
    let gc = GarbageCollector::new();
    let capacities = Capacities::derive(2, 2, 1)
        .unwrap()
        .with_roots(2)
        .unwrap()
        .with_edges(2)
        .unwrap();
    let budget = Budget::new(GraphFootprint::new(capacities, 1).unwrap().total());
    let runtime = GraphRuntime::new(&gc, capacities, 1, &budget).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut reader = runtime.mutator().unwrap();
    let mut writer = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let a = alloc(&mut reader, &mut helper, &drops, 1);
    let b = alloc(&mut reader, &mut helper, &drops, 2);
    write(&mut writer, &mut helper, &a, Some(&b));
    let mut request = reader.child::<Value, Value>(&a, 0).unwrap();
    for _ in 0..20 {
        assert!(request.poll().unwrap().is_none());
        helper.poll().unwrap();
    }
    assert_eq!(runtime.catalog.reserved_root_tokens(), 2);
    write(&mut writer, &mut helper, &a, None);
    let mut result = None;
    for _ in 0..100 {
        if let Some(value) = request.poll().unwrap() {
            result = Some(value);
            break;
        }
        helper.poll().unwrap();
    }
    assert!(
        matches!(result, Some(GraphReadValue::Null)),
        "root quota hid a completed null replacement"
    );
    drop(request);
    drop(a);
    drop(b);
    collect(&mut helper, &drops, 2);
    drop(reader);
    drop(writer);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
}

#[test]
fn logical_edge_count_survives_root_transfer_and_slot_reuse() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let value = |identity| Value {
        identity,
        drops: drops.clone(),
    };
    assert!(matches!(
        worker.allocate_with_edges(value(99), 3),
        Err(GraphError::Layout)
    ));
    assert_eq!(gc.managed_bytes(), 0);
    for round in 0..8 {
        let mut allocate = |identity, count| {
            let mut request = worker.allocate_with_edges(value(identity), count).unwrap();
            for _ in 0..50_000 {
                if let Some(root) = request.poll().unwrap() {
                    return root;
                }
                helper.poll().unwrap();
            }
            panic!("shaped allocation did not complete")
        };
        let parent = allocate(0, 1);
        let target = allocate(round, 0);
        assert!(matches!(
            worker.child::<Value, Value>(&parent, 1),
            Err(GraphError::Index)
        ));
        write(&mut worker, &mut helper, &parent, Some(&target));
        drop(target);
        let read = child(&mut worker, &mut helper, &parent);
        assert_eq!(runtime.borrow(&read).unwrap().identity, round);
        assert!(matches!(
            worker.write(&read, 0, Some(&parent)),
            Err(GraphError::Index)
        ));
        drop((read, parent));
        collect(
            &mut helper,
            &drops,
            1 + (usize::try_from(round).unwrap() + 1) * 2,
        );
        assert_eq!(gc.managed_bytes(), 0);
    }
    drop((helper, worker, runtime));
    assert_eq!(budget.used(), 0);
}

#[test]
fn graph_cycle_destroys_payloads_and_reuses_inventory_before_runtime_drop() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    for round in 0..6 {
        let a = alloc(&mut worker, &mut helper, &drops, 2 * round);
        let b = alloc(&mut worker, &mut helper, &drops, 2 * round + 1);
        write(&mut worker, &mut helper, &a, Some(&b));
        write(&mut worker, &mut helper, &b, Some(&a));
        drop(a);
        drop(b);
        collect(
            &mut helper,
            &drops,
            2 * (usize::try_from(round).unwrap() + 1),
        );
    }
    for _ in 0..5000 {
        helper.poll().unwrap();
    }
    for slot in 0..8 {
        assert!(
            worker.worker.discover_edge(slot).unwrap().is_none(),
            "orphan inventory retained"
        );
    }
    drop(worker);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn graph_concurrent_replacement_reads_and_collection_keep_exact_payloads() {
    struct Done<'a>(&'a AtomicUsize);
    impl Drop for Done<'_> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Release);
        }
    }
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut initial = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let source = alloc(&mut initial, &mut helper, &drops, 0);
    let left = alloc(&mut initial, &mut helper, &drops, 1);
    let right = alloc(&mut initial, &mut helper, &drops, 2);
    write(&mut initial, &mut helper, &source, Some(&left));
    let mut writer = runtime.mutator().unwrap();
    let mut reader = runtime.mutator().unwrap();
    let completed = AtomicUsize::new(0);
    let rounds = if cfg!(miri) { 4 } else { 64 };
    let mut helper = std::thread::scope(|scope| {
        let completed = &completed;
        let runtime = &runtime;
        let source = &source;
        let left = &left;
        let right = &right;
        let helping = scope.spawn(move || {
            for n in 0..1_000_000 {
                if completed.load(Ordering::Acquire) == 2 {
                    return helper;
                }
                helper.poll().unwrap();
                if n % 32 == 0 {
                    std::thread::yield_now();
                }
            }
            panic!("graph worker watchdog");
        });
        let writing = scope.spawn(move || {
            let _done = Done(completed);
            for n in 0..rounds {
                let mut request = writer
                    .write(source, 0, Some(if n % 2 == 0 { right } else { left }))
                    .unwrap();
                let mut finished = false;
                for poll in 0..100_000 {
                    if request.poll().unwrap() {
                        finished = true;
                        break;
                    }
                    if poll % 32 == 0 {
                        std::thread::yield_now();
                    }
                }
                assert!(finished, "concurrent graph write did not settle");
            }
        });
        let reading = scope.spawn(move || {
            let _done = Done(completed);
            for _ in 0..rounds {
                let mut request = reader.child::<Value, Value>(source, 0).unwrap();
                let mut finished = false;
                for poll in 0..100_000 {
                    if let Some(value) = request.poll().unwrap() {
                        let GraphReadValue::Root(root) = value else {
                            panic!("nonnull edge read as null");
                        };
                        assert!([1, 2].contains(&runtime.borrow(&root).unwrap().identity));
                        finished = true;
                        break;
                    }
                    if poll % 32 == 0 {
                        std::thread::yield_now();
                    }
                }
                assert!(finished, "concurrent graph read did not settle");
            }
        });
        writing.join().unwrap();
        reading.join().unwrap();
        helping.join().unwrap()
    });
    assert_eq!(completed.load(Ordering::Acquire), 2);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(source);
    drop(left);
    drop(right);
    collect(&mut helper, &drops, 3);
    drop(initial);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn graph_stopped_allocation_prefix_does_not_hold_unrelated_cycles() {
    for prefix in [0, 5, 12, 20, 30] {
        let gc = GarbageCollector::new();
        let (runtime, budget, drops) = setup(&gc);
        let stopped_drops = Arc::new(AtomicUsize::new(0));
        let mut stopped = runtime.mutator().unwrap();
        let mut active = runtime.mutator().unwrap();
        let mut helper = runtime.maintenance().unwrap();
        std::thread::scope(|scope| {
            let (ready_send, ready_receive) = std::sync::mpsc::sync_channel(0);
            let (release_send, release_receive) = std::sync::mpsc::sync_channel(0);
            let stopped_drops = stopped_drops.clone();
            let waiting = scope.spawn(move || {
                let mut request = stopped
                    .allocate(Value {
                        identity: 99,
                        drops: stopped_drops,
                    })
                    .unwrap();
                let mut result = None;
                for _ in 0..prefix {
                    result = request.poll().unwrap();
                    if result.is_some() {
                        break;
                    }
                }
                ready_send.send(()).unwrap();
                // Disconnect also releases this thread if the active assertion fails.
                let _ = release_receive.recv();
                drop(request);
                drop(result);
            });
            ready_receive.recv().unwrap();
            let a = alloc(&mut active, &mut helper, &drops, 1);
            let b = alloc(&mut active, &mut helper, &drops, 2);
            write(&mut active, &mut helper, &a, Some(&b));
            write(&mut active, &mut helper, &b, Some(&a));
            drop(a);
            drop(b);
            collect(&mut helper, &drops, 2);
            release_send.send(()).unwrap();
            waiting.join().unwrap();
        });
        collect(&mut helper, &stopped_drops, 1);
        drop(active);
        drop(helper);
        drop(runtime);
        assert_eq!(budget.used(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
}

#[test]
fn graph_indexed_degree_boundaries_preserve_the_last_child_and_reclaim_inventory() {
    for degree in [1, 8] {
        let gc = GarbageCollector::new();
        let capacities = Capacities::derive(2, 1, 1)
            .unwrap()
            .with_roots(4)
            .unwrap()
            .with_edges(2 * degree)
            .unwrap();
        let budget = Budget::new(GraphFootprint::new(capacities, degree).unwrap().total());
        let runtime = GraphRuntime::new(&gc, capacities, degree, &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut helper = runtime.maintenance().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        let target = alloc(&mut worker, &mut helper, &drops, 11);
        let source = alloc(&mut worker, &mut helper, &drops, 22);
        {
            let mut write = worker.write(&source, degree - 1, Some(&target)).unwrap();
            assert!((0..10_000).any(|_| {
                let complete = write.poll().unwrap();
                helper.poll().unwrap();
                complete
            }));
        }
        drop(target);
        for _ in 0..2000 {
            helper.poll().unwrap();
        }
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        let child = {
            let mut read = worker.child::<Value, Value>(&source, degree - 1).unwrap();
            (0..10_000)
                .find_map(|_| {
                    let result = read.poll().unwrap();
                    helper.poll().unwrap();
                    result
                })
                .expect("last indexed graph child did not settle")
        };
        let GraphReadValue::Root(child) = child else {
            panic!("last indexed graph child became null");
        };
        assert_eq!(runtime.borrow(&child).unwrap().identity, 11);
        drop(source);
        drop(child);
        collect(&mut helper, &drops, 2);
        for _ in 0..2000 {
            helper.poll().unwrap();
        }
        for slot in 0..2 * degree {
            assert!(
                worker.worker.discover_edge(slot).unwrap().is_none(),
                "indexed graph left orphan inventory"
            );
        }
        drop(worker);
        drop(helper);
        drop(runtime);
        assert_eq!(budget.used(), 0);
        assert_eq!(gc.managed_bytes(), 0);
    }
}

#[test]
fn graph_aliases_preserve_cycles_and_independent_child_roots_without_more_tokens() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let a = alloc(&mut worker, &mut helper, &drops, 101);
    let b = alloc(&mut worker, &mut helper, &drops, 202);
    write(&mut worker, &mut helper, &a, Some(&b));
    write(&mut worker, &mut helper, &b, Some(&a));
    let child = child(&mut worker, &mut helper, &a);
    let a_alias = a.clone();
    let child_alias = child.clone();
    assert_eq!(a.root.key(), a_alias.root.key());
    assert_eq!(child.root.key(), child_alias.root.key());
    assert_ne!(b.root.key(), child_alias.root.key());
    assert_eq!(runtime.catalog.reserved_root_tokens(), 3);
    drop(a);
    drop(b);
    drop(child);
    assert_eq!(runtime.catalog.reserved_root_tokens(), 2);
    drop(alloc(&mut worker, &mut helper, &drops, 303));
    collect(&mut helper, &drops, 1);
    assert_eq!(runtime.borrow(&a_alias).unwrap().identity, 101);
    assert_eq!(runtime.borrow(&child_alias).unwrap().identity, 202);
    drop(a_alias);
    drop(alloc(&mut worker, &mut helper, &drops, 404));
    collect(&mut helper, &drops, 2);
    assert_eq!(runtime.borrow(&child_alias).unwrap().identity, 202);
    drop(child_alias);
    collect(&mut helper, &drops, 4);
    assert_eq!(runtime.catalog.reserved_root_tokens(), 0);
    drop(worker);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn graph_aliases_work_at_full_root_quota_while_child_reads_remain_independent() {
    let gc = GarbageCollector::new();
    let caps = Capacities::derive(2, 1, 1)
        .unwrap()
        .with_roots(2)
        .unwrap()
        .with_edges(2)
        .unwrap();
    let budget = Budget::new(GraphFootprint::new(caps, 1).unwrap().total());
    let runtime = GraphRuntime::new(&gc, caps, 1, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let a = alloc(&mut worker, &mut helper, &drops, 501);
    let b = alloc(&mut worker, &mut helper, &drops, 502);
    write(&mut worker, &mut helper, &a, Some(&b));
    let aliases = [a.clone(), b.clone()];
    drop(a);
    drop(b);
    assert_eq!(runtime.catalog.reserved_root_tokens(), 2);
    {
        let mut read = worker.child::<Value, Value>(&aliases[0], 0).unwrap();
        for _ in 0..32 {
            assert!(
                read.poll().unwrap().is_none(),
                "child read bypassed its independent-root quota"
            );
            helper.poll().unwrap();
        }
    }
    assert_eq!(runtime.borrow(&aliases[0]).unwrap().identity, 501);
    assert_eq!(runtime.borrow(&aliases[1]).unwrap().identity, 502);
    write(&mut worker, &mut helper, &aliases[0], None);
    {
        let mut read = worker.child::<Value, Value>(&aliases[0], 0).unwrap();
        assert!(matches!(read.poll().unwrap(), Some(GraphReadValue::Null)));
    }
    assert_eq!(runtime.catalog.reserved_root_tokens(), 2);
    drop(aliases);
    collect(&mut helper, &drops, 2);
    drop(worker);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn graph_aliases_clone_read_and_drop_concurrently_with_unrelated_reclamation() {
    let gc = GarbageCollector::new();
    let (runtime, budget, drops) = setup(&gc);
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let root = alloc(&mut worker, &mut helper, &drops, 601);
    drop(alloc(&mut worker, &mut helper, &drops, 602));
    let barrier = std::sync::Barrier::new(4);
    std::thread::scope(|scope| {
        let mut threads = Vec::new();
        for _ in 0..3 {
            let owned = root.clone();
            let barrier = &barrier;
            let runtime = &runtime;
            threads.push(scope.spawn(move || {
                barrier.wait();
                for _ in 0..16 {
                    let alias = owned.clone();
                    assert_eq!(runtime.borrow(&alias).unwrap().identity, 601);
                }
            }));
        }
        drop(root);
        barrier.wait();
        for _ in 0..64 {
            helper.poll().unwrap();
        }
        for thread in threads {
            thread.join().unwrap();
        }
    });
    collect(&mut helper, &drops, 2);
    assert_eq!(runtime.catalog.reserved_root_tokens(), 0);
    drop(worker);
    drop(helper);
    drop(runtime);
    assert_eq!(budget.used(), 0);
    assert_eq!(gc.managed_bytes(), 0);
}

#[cfg(feature = "experimental-canonical-runtime")]
#[test]
fn passive_observation_primary_mapping_is_full_only_checked_and_conservative() {
    use gc_catalog::{Domain, DomainMode, Phase};
    let phase = |mode, epoch| Phase {
        root_cursors: [0; 2],
        reference_cursors: [0; 2],
        revision: 1,
        domains: [Domain {
            mode,
            epoch,
            floor: 0,
            pending: 0,
        }; 2],
    };
    for (mode, epoch, count) in [
        (DomainMode::Full, 1, 0),
        (DomainMode::Inactive, 1, 1),
        (DomainMode::Full, 2, 1),
        (DomainMode::Full, u64::MAX, u64::MAX - 1),
        (DomainMode::Inactive, u64::MAX, u64::MAX),
    ] {
        assert_eq!(
            primary_cycles([phase(mode, epoch); 2]).unwrap(),
            (count, count)
        );
    }
    assert_eq!(
        primary_cycles([phase(DomainMode::Inactive, 4), phase(DomainMode::Full, 4)]).unwrap(),
        (3, 4)
    );
    assert_eq!(
        primary_cycles([phase(DomainMode::Full, 5), phase(DomainMode::Inactive, 4)]).unwrap(),
        (4, 4)
    );
    for mode in [
        DomainMode::Full,
        DomainMode::Inactive,
        DomainMode::Young,
        DomainMode::Old,
    ] {
        assert!(matches!(
            primary_cycles([phase(mode, 0); 2]),
            Err(GraphError::Protocol)
        ));
    }
    for mode in [DomainMode::Young, DomainMode::Old] {
        for outcomes in [
            [phase(mode, 1), phase(DomainMode::Full, 1)],
            [phase(DomainMode::Full, 1), phase(mode, 1)],
        ] {
            assert!(matches!(
                primary_cycles(outcomes),
                Err(GraphError::Protocol)
            ));
        }
    }
}

#[test]
fn a_parked_child_read_cannot_acquire_a_replacement_in_the_old_target_slot() {
    let gc = GarbageCollector::new();
    let caps = Capacities::derive(2, 2, 1)
        .unwrap()
        .with_roots(8)
        .unwrap()
        .with_edges(2)
        .unwrap();
    let budget = Budget::new(GraphFootprint::new(caps, 1).unwrap().total());
    let runtime = GraphRuntime::new(&gc, caps, 1, &budget).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut reader = runtime.mutator().unwrap();
    let mut writer = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let source = alloc(&mut writer, &mut helper, &drops, 1);
    let first = alloc(&mut writer, &mut helper, &drops, 2);
    let old_key = first.root.observe().target;
    write(&mut writer, &mut helper, &source, Some(&first));
    let mut read = reader.child::<Value, Value>(&source, 0).unwrap();
    assert!(read.poll().unwrap().is_none());
    write(&mut writer, &mut helper, &source, None);
    drop(first);
    collect(&mut helper, &drops, 1);
    let replacement = alloc(&mut writer, &mut helper, &drops, 3);
    let new_key = replacement.root.observe().target;
    assert_eq!(new_key.slot(), old_key.slot());
    assert_ne!(new_key.identity(), old_key.identity());

    let completed = (0..1000).find_map(|_| {
        let value = read.poll().unwrap();
        helper.poll().unwrap();
        value
    });
    assert!(matches!(completed, Some(GraphReadValue::Null)));
    drop(read);
    assert_eq!(runtime.borrow(&replacement).unwrap().identity, 3);
    drop((replacement, source));
    collect(&mut helper, &drops, 3);
    drop((reader, writer, helper, runtime));
    assert_eq!(budget.used(), 0);
}
