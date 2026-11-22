use super::super::allocation::LeafRoot;
use super::super::runtime::{LeafMutator, LeafRuntime};
use super::*;
use crate::{CanonicalPayload, Configuration, Trace};
use gc_catalog::bounded::{BoundedCatalog, Budget, Capacities};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Value {
    number: usize,
    drops: Arc<AtomicUsize>,
}
impl Trace for Value {
    fn trace(&self, _: &GarbageCollector) {}

    fn may_contain_gc_references() -> bool {
        false
    }

    fn canonical_payload() -> Option<CanonicalPayload<Self>> {
        // SAFETY: only a scalar and a host atomic counter; no managed edges.
        Some(unsafe { CanonicalPayload::new() })
    }
}
impl Drop for Value {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

fn heap() -> Arc<GarbageCollector> {
    GarbageCollector::with_config(Configuration::builder().threads(1).build())
}

fn allocate(
    worker: &mut LeafMutator<'_>,
    helper: &mut LeafMaintenance<'_>,
    drops: &Arc<AtomicUsize>,
    number: usize,
) -> LeafRoot<Value> {
    let mut request = worker
        .begin_any(Value {
            number,
            drops: drops.clone(),
        })
        .unwrap();
    (0..20_000)
        .find_map(|_| {
            let value = request.poll().unwrap();
            if value.is_none() {
                helper.poll().unwrap();
            }
            value
        })
        .expect("finite leaf allocation")
}

fn until_dropped(helper: &mut LeafMaintenance<'_>, drops: &AtomicUsize, count: usize) {
    assert!(
        (0..20_000).any(|_| {
            helper.poll().unwrap();
            drops.load(Ordering::SeqCst) == count
        }),
        "finite incremental cleanup"
    );
}

#[test]
fn live_leaf_survives_epochs_then_reclaims_and_reuses_one_slot() {
    let heap = heap();
    let budget = Budget::new(usize::MAX);
    let caps = Capacities::derive(1, 1, 1).unwrap().with_roots(1).unwrap();
    let runtime = LeafRuntime::new(&heap, caps, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    for number in 0..8 {
        let root = allocate(&mut worker, &mut helper, &drops, number);
        let start = helper.phase.map_or(0, |phase| phase.domains[0].epoch);
        assert!((0..20_000).any(|_| {
            helper.poll().unwrap();
            helper
                .phase
                .is_some_and(|phase| phase.domains[0].epoch >= start + 2)
        }));
        assert_eq!(drops.load(Ordering::SeqCst), number);
        assert_eq!(runtime.borrow(&root).unwrap().number, number);
        drop(root);
        until_dropped(&mut helper, &drops, number + 1);
        assert_eq!(heap.managed_bytes(), 0);
    }
    drop((worker, helper, runtime));
    assert_eq!(budget.used(), 0);
}

#[test]
fn pending_allocation_cancellation_is_reclaimed_without_runtime_teardown() {
    for prefix in 0..24 {
        let heap = heap();
        let budget = Budget::new(usize::MAX);
        let caps = Capacities::derive(1, 1, 1).unwrap().with_roots(1).unwrap();
        let runtime = LeafRuntime::new(&heap, caps, &budget).unwrap();
        let mut worker = runtime.mutator().unwrap();
        let mut helper = runtime.maintenance().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let mut request = worker
                .begin_any(Value {
                    number: 11,
                    drops: drops.clone(),
                })
                .unwrap();
            for _ in 0..prefix {
                if let Some(root) = request.poll().unwrap() {
                    drop(root);
                    break;
                }
            }
        }
        until_dropped(&mut helper, &drops, 1);
        assert_eq!(heap.managed_bytes(), 0);
        let root = allocate(&mut worker, &mut helper, &drops, 12);
        assert_eq!(runtime.borrow(&root).unwrap().number, 12);
        drop(root);
        until_dropped(&mut helper, &drops, 2);
        drop((worker, helper, runtime));
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn retained_view_allows_unrelated_reclamation_and_helper_replacement() {
    let heap = heap();
    let budget = Budget::new(usize::MAX);
    let caps = Capacities::derive(2, 1, 1).unwrap().with_roots(2).unwrap();
    let runtime = LeafRuntime::new(&heap, caps, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    assert!(matches!(
        runtime.maintenance(),
        Err(MaintenanceError::Register(RegisterError::Limit))
    ));
    let drops = Arc::new(AtomicUsize::new(0));
    let root = allocate(&mut worker, &mut helper, &drops, 91);
    let victim = allocate(&mut worker, &mut helper, &drops, 17);
    {
        let view = runtime.borrow(&root).unwrap();
        drop(victim);
        until_dropped(&mut helper, &drops, 1);
        drop(helper);
        helper = runtime.maintenance().unwrap();
        assert_eq!(view.number, 91);
    }
    drop(root);
    until_dropped(&mut helper, &drops, 2);
    drop((worker, helper, runtime));
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn another_thread_reclaims_while_a_typed_reader_is_parked() {
    use std::sync::mpsc::sync_channel;
    use std::time::Duration;
    let heap = heap();
    let budget = Budget::new(usize::MAX);
    let caps = Capacities::derive(2, 1, 1).unwrap().with_roots(2).unwrap();
    let runtime = LeafRuntime::new(&heap, caps, &budget).unwrap();
    let mut worker = runtime.mutator().unwrap();
    let mut helper = runtime.maintenance().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let root = allocate(&mut worker, &mut helper, &drops, 29);
    let victim = allocate(&mut worker, &mut helper, &drops, 7);
    let view = runtime.borrow(&root).unwrap();
    std::thread::scope(|scope| {
        let (ready_tx, ready_rx) = sync_channel(1);
        let (release_tx, release_rx) = sync_channel(1);
        scope.spawn(move || {
            assert_eq!(view.number, 29);
            ready_tx.send(()).unwrap();
            release_rx.recv_timeout(Duration::from_secs(30)).unwrap();
            assert_eq!(view.number, 29);
        });
        ready_rx.recv_timeout(Duration::from_secs(30)).unwrap();
        drop(victim);
        until_dropped(&mut helper, &drops, 1);
        release_tx.send(()).unwrap();
    });
    drop(root);
    until_dropped(&mut helper, &drops, 2);
    drop((worker, helper, runtime));
    assert_eq!(heap.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn empty_batch_is_bounded_and_wraps_without_duplicate_visits() {
    for slots in [1, 2, 7, 8, 9, 17] {
        for start in [0, slots / 2, slots - 1] {
            let mut cursor = start;
            let mut visited = Vec::new();
            let (skipped, result) = scan_slots(&mut cursor, slots, |slot| {
                visited.push(slot);
                Ok(None::<()>)
            });
            let expected: Vec<_> = (0..slots.min(8))
                .map(|offset| (start + offset) % slots)
                .collect();
            assert_eq!(visited, expected, "empty batch probe bound or cursor wrap");
            assert_eq!(skipped, expected.len() - 1);
            assert_eq!(
                cursor,
                *expected.last().unwrap(),
                "last probe must await lane completion"
            );
            assert_eq!(result, Ok(None));
        }
    }
}

#[test]
fn empty_batch_stops_at_each_occupied_or_failed_observation() {
    for stop in 0..8 {
        for terminal in [
            Ok(Some(71)),
            Err(RequestError::Contended),
            Err(RequestError::InvalidSlot),
        ] {
            let mut cursor = 13;
            let mut visited = Vec::new();
            let (skipped, result) = scan_slots(&mut cursor, 17, |slot| {
                let offset = visited.len();
                visited.push(slot);
                if offset == stop { terminal } else { Ok(None) }
            });
            assert_eq!(
                visited.len(),
                stop + 1,
                "batch crossed an occupied or failed observation"
            );
            assert_eq!(skipped, stop);
            assert_eq!(cursor, (13 + stop) % 17);
            assert_eq!(result, terminal, "batch lost the final observation");
        }
    }
}

fn phase(epoch: u64, floor: u8, mode: DomainMode) -> Phase {
    Phase {
        root_cursors: [0; 2],
        reference_cursors: [0; 2],
        revision: 0,
        domains: [
            gc_catalog::Domain {
                mode,
                epoch,
                floor,
                pending: 0,
            },
            gc_catalog::Domain {
                mode: DomainMode::Inactive,
                epoch: 1,
                floor: RETIREMENT_FLOOR,
                pending: 0,
            },
        ],
    }
}

fn visited(helper: &mut LeafMaintenance<'_>, success: bool) {
    helper.lane = Lane::Object;
    helper.finish_lane(success);
}

#[test]
fn sweep_counts_consecutive_visits_across_delayed_phase_observations() {
    let gc = GarbageCollector::new();
    let catalog = BoundedCatalog::with_records_in(
        Capacities::derive(3, 0, 1).unwrap(),
        &Budget::new(usize::MAX),
    )
    .unwrap();
    let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
    let phase = phase(7, RETIREMENT_FLOOR, DomainMode::Full);
    helper.observed_phase(phase);
    for _ in 0..3 {
        visited(&mut helper, true);
    }
    assert_eq!(
        helper.swept[0], 0,
        "uncertified interval cannot finish a cycle"
    );
    helper.observed_phase(phase);
    assert_eq!(helper.swept[0], 3);
    assert_eq!(helper.visits_since_phase, 0);
    assert_eq!(helper.swept[1], 0);
}

#[test]
fn a_failed_object_discovery_requires_a_new_complete_sweep() {
    let gc = GarbageCollector::new();
    let catalog = BoundedCatalog::with_records_in(
        Capacities::derive(3, 0, 1).unwrap(),
        &Budget::new(usize::MAX),
    )
    .unwrap();
    let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
    let phase = phase(9, RETIREMENT_FLOOR, DomainMode::Full);
    helper.observed_phase(phase);
    for _ in 0..3 {
        visited(&mut helper, true);
    }
    helper.observed_phase(phase);
    assert_eq!(helper.swept[0], 3);
    visited(&mut helper, false);
    for _ in 0..2 {
        visited(&mut helper, true);
    }
    helper.observed_phase(phase);
    assert_eq!(helper.swept[0], 2);
    visited(&mut helper, true);
    helper.observed_phase(phase);
    assert_eq!(helper.swept[0], 3);
}

#[test]
fn a_changed_epoch_mode_or_floor_cannot_certify_prior_visits() {
    let gc = GarbageCollector::new();
    let catalog = BoundedCatalog::with_records_in(
        Capacities::derive(3, 0, 1).unwrap(),
        &Budget::new(usize::MAX),
    )
    .unwrap();
    let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
    let baseline = phase(11, RETIREMENT_FLOOR, DomainMode::Full);
    for changed in [
        phase(12, RETIREMENT_FLOOR, DomainMode::Full),
        phase(11, RETIREMENT_FLOOR, DomainMode::Inactive),
        phase(11, RETIREMENT_FLOOR - 1, DomainMode::Full),
    ] {
        helper.observed_phase(baseline);
        for _ in 0..3 {
            visited(&mut helper, true);
        }
        helper.observed_phase(baseline);
        assert_eq!(helper.swept[0], 3, "establish a complete certified sweep");
        visited(&mut helper, true);
        helper.observed_phase(changed);
        assert_eq!(helper.swept[0], 0);
        assert_eq!(helper.visits_since_phase, 0);
    }
}

#[test]
fn stale_queued_finish_cannot_close_a_later_unswept_epoch() {
    fn run(worker: &mut Worker, gc: &GarbageCollector, command: GuardedCommand) {
        worker.submit(command).unwrap();
        let result = (0..128)
            .map(|_| worker.poll_in(gc))
            .find(|r| *r != Progress::Pending)
            .unwrap();
        assert_eq!(result, Progress::Committed);
    }
    let gc = GarbageCollector::new();
    let catalog = BoundedCatalog::with_records_in(
        Capacities::derive(3, 1, 1).unwrap(),
        &Budget::new(usize::MAX),
    )
    .unwrap();
    let mut external = catalog.register_record_mutator().unwrap();
    let mut helper = LeafMaintenance::new(&catalog, &gc).unwrap();
    let mut old_epoch = None;
    for _ in 0..4096 {
        helper.poll().unwrap();
        if matches!(helper.lane, Lane::Phase)
            && matches!(helper.pending, Pending::Command)
            && helper.swept[0] == helper.registry
            && let Some(phase) = helper.phase
            && phase.domains[0].floor == RETIREMENT_FLOOR
            && phase.domains[0].mode != DomainMode::Inactive
            && phase.domains[1].mode == DomainMode::Inactive
        {
            old_epoch = Some(phase.domains[0].epoch);
            break;
        }
    }
    let old_epoch = old_epoch.expect("queued finish after a complete local sweep");
    // Another policy owner completes this epoch, then reaches retirement in
    // a new epoch while this helper's Finish request remains unpolled.
    run(&mut external, &gc, GuardedCommand::Finish { domain: 0 });
    run(
        &mut external,
        &gc,
        GuardedCommand::Start {
            domain: 0,
            mode: DomainMode::Full,
        },
    );
    for _ in 0..RETIREMENT_FLOOR {
        run(&mut external, &gc, GuardedCommand::Advance { domain: 0 });
    }
    let current = external.phase_in(&gc).unwrap();
    assert_eq!(current.domains[0].epoch, old_epoch + 1);
    let result = (0..128)
        .map(|_| helper.poll().unwrap())
        .find(|r| *r != MaintenanceStep::Pending)
        .unwrap();
    assert_eq!(
        result,
        MaintenanceStep::Visited,
        "stale Finish must be rejected before closing another epoch"
    );
    let after = external.phase_in(&gc).unwrap();
    assert_eq!(after.domains[0].mode, DomainMode::Full);
    assert_eq!(after.domains[0].epoch, current.domains[0].epoch);
    assert_eq!(after.domains[0].floor, RETIREMENT_FLOOR);
}
