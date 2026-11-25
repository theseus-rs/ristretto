//! End-to-end leaf cleanup uses only maintenance polls, not hand-authored phase,
//! root-removal, acknowledgment or retirement commands.
use super::*;
use crate::canonical_catalog as gc_catalog;
use crate::gc::publication_owner::canonical_leaf::allocation::{LeafAllocation, LeafRoot};
use crate::gc::publication_owner::canonical_leaf::maintenance::LeafMaintenance;

fn configured(
    registry: usize,
    roots: usize,
    helpers: usize,
) -> (CatalogHandle<LeafRecord>, Arc<Budget>, Worker) {
    let capacities = Capacities::derive(registry, 1, helpers)
        .unwrap()
        .with_roots(roots)
        .unwrap();
    let footprint = Footprint::for_records::<LeafRecord>(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
    let worker = catalog.register_record_mutator().unwrap();
    (catalog, budget, worker)
}
fn allocate(
    worker: &mut Worker,
    gc: &GarbageCollector,
    slot: usize,
    drops: &Arc<AtomicUsize>,
) -> LeafRoot<Value> {
    let mut request = LeafAllocation::begin(
        worker,
        gc,
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
        .expect("typed leaf allocation")
}
fn until_dropped(maintenance: &mut LeafMaintenance<'_>, drops: &Arc<AtomicUsize>, expected: usize) {
    for _ in 0..50_000 {
        if drops.load(Ordering::Relaxed) == expected {
            return;
        }
        maintenance.poll().expect("maintenance step");
    }
    assert_eq!(
        drops.load(Ordering::Relaxed),
        expected,
        "maintenance did not reclaim all expected payloads"
    );
}
fn empty(worker: &mut Worker, catalog: &CatalogHandle<LeafRecord>) {
    assert_eq!(catalog.reserved_root_tokens(), 0);
    for slot in 0..catalog.capacities().registry() {
        assert!(worker.discover(slot).unwrap().is_none());
    }
    for slot in 0..catalog.capacities().root_slots() {
        assert!(worker.discover_root(slot).unwrap().is_none());
    }
}

#[test]
fn maintenance_reclaims_every_cancelled_request_prefix_and_dropped_typed_result() {
    for prefix in 0..24 {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget, mut worker) = configured(3, 2, 1);
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let mut request = LeafAllocation::begin(
                &mut worker,
                &gc,
                0,
                0,
                Value {
                    number: 800,
                    drops: drops.clone(),
                },
            )
            .unwrap();
            for _ in 0..prefix {
                if let Some(root) = request.poll().unwrap() {
                    drop(root);
                    break;
                }
            }
        }
        let mut maintenance = LeafMaintenance::new(&catalog, &gc).unwrap();
        until_dropped(&mut maintenance, &drops, 1);
        empty(&mut worker, &catalog);
        assert_eq!(gc.managed_bytes(), 0);
        drop(maintenance);
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn complete_sweeps_preserve_live_roots_and_reuse_every_unreachable_slot() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = configured(17, 17, 1);
    let drops = Arc::new(AtomicUsize::new(0));
    let keeper = allocate(&mut worker, &gc, 0, &drops);
    for slot in 1..17 {
        drop(allocate(&mut worker, &gc, slot, &drops));
    }
    let mut maintenance = LeafMaintenance::new(&catalog, &gc).unwrap();
    until_dropped(&mut maintenance, &drops, 16);
    assert_eq!(keeper.borrow(&gc).unwrap().number, 0);
    for slot in 1..17 {
        assert!(worker.discover(slot).unwrap().is_none());
        assert!(worker.discover_root(slot).unwrap().is_none());
        drop(allocate(&mut worker, &gc, slot, &drops));
    }
    until_dropped(&mut maintenance, &drops, 32);
    assert_eq!(keeper.borrow(&gc).unwrap().number, 0);
    drop(keeper);
    until_dropped(&mut maintenance, &drops, 33);
    empty(&mut worker, &catalog);
    assert_eq!(gc.managed_bytes(), 0);
    drop(maintenance);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn two_helpers_finish_sweeps_and_reclaim_all_slots_across_repeated_cycles() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = configured(7, 7, 2);
    let drops = Arc::new(AtomicUsize::new(0));
    let mut first = LeafMaintenance::new(&catalog, &gc).unwrap();
    let mut second = LeafMaintenance::new(&catalog, &gc).unwrap();
    assert_eq!(catalog.registered(), (1, 2));
    for round in 0..4 {
        for slot in 0..7 {
            drop(allocate(&mut worker, &gc, slot, &drops));
        }
        let expected = (round + 1) * 7;
        for turn in 0..50_000 {
            if drops.load(Ordering::Relaxed) == expected {
                break;
            }
            first.poll().unwrap();
            if turn % 3 != 0 {
                second.poll().unwrap();
            }
        }
        assert_eq!(drops.load(Ordering::Relaxed), expected);
        // A controller attempt in the other helper can retain an already retired
        // cell. Finish that bounded request before asserting physical emptiness.
        for _ in 0..128 {
            first.poll().unwrap();
            second.poll().unwrap();
        }
        empty(&mut worker, &catalog);
        assert_eq!(gc.managed_bytes(), 0);
    }
    drop(first);
    drop(second);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn dropping_a_maintenance_owner_releases_its_helper_and_preserves_cleanup_work() {
    for prefix in 0..32 {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget, mut worker) = configured(3, 2, 1);
        let drops = Arc::new(AtomicUsize::new(0));
        drop(allocate(&mut worker, &gc, 0, &drops));
        {
            let mut maintenance = LeafMaintenance::new(&catalog, &gc).unwrap();
            for _ in 0..prefix {
                maintenance.poll().unwrap();
            }
        }
        assert_eq!(catalog.registered(), (1, 0));
        let mut replacement = LeafMaintenance::new(&catalog, &gc).unwrap();
        until_dropped(&mut replacement, &drops, 1);
        empty(&mut worker, &catalog);
        assert_eq!(gc.managed_bytes(), 0);
        drop(replacement);
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn leaf_maintenance_rejects_conditional_reference_catalogs_without_consuming_a_helper() {
    let gc = collector(true, RootStorage::Contexts);
    let capacities = Capacities::derive(2, 0, 1)
        .unwrap()
        .with_roots(1)
        .unwrap()
        .with_references(1)
        .unwrap();
    // The packaged catalog rejects conditional references at construction,
    // before a helper or any backing credit can be reserved.
    assert!(matches!(
        Footprint::for_records::<LeafRecord>(capacities),
        Err(gc_catalog::bounded::ConfigError::UnsupportedReferences)
    ));
    let budget = Budget::new(usize::MAX);
    assert!(matches!(
        BoundedCatalog::<LeafRecord>::with_records_in(capacities, &budget),
        Err(gc_catalog::bounded::ConfigError::UnsupportedReferences)
    ));
    assert_eq!(budget.used(), 0);
    let supported = capacities.with_references(0).unwrap();
    let catalog = BoundedCatalog::with_records_in(supported, &budget).unwrap();
    assert_eq!(catalog.registered(), (0, 0));
    let helper = LeafMaintenance::new(&catalog, &gc).unwrap();
    assert_eq!(catalog.registered(), (0, 1));
    drop(helper);
    assert_eq!(catalog.registered(), (0, 0));
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn revoked_root_views_keep_only_their_old_storage_while_maintenance_reuses_the_object_slot() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = configured(3, 2, 1);
    let old_drops = Arc::new(AtomicUsize::new(0));
    let other_drops = Arc::new(AtomicUsize::new(0));
    let old = allocate(&mut worker, &gc, 0, &old_drops);
    let root_key = worker.discover_root(0).unwrap().unwrap().key;
    let alias = old.clone();
    let view = alias.borrow(&gc).unwrap();
    // Explicit revocation is setup for a retained-view schedule. All collection
    // and physical cleanup below run through the maintenance policy.
    roots(&mut worker, &gc, RootCommand::Remove(root_key));
    drop(allocate(&mut worker, &gc, 1, &other_drops));
    let mut maintenance = LeafMaintenance::new(&catalog, &gc).unwrap();
    until_dropped(&mut maintenance, &other_drops, 1);
    for _ in 0..50_000 {
        if worker.discover(0).unwrap().is_none() {
            break;
        }
        maintenance.poll().unwrap();
    }
    assert!(worker.discover(0).unwrap().is_none());
    assert_eq!(old_drops.load(Ordering::Relaxed), 0);
    assert_eq!(view.number, 0);
    assert!(matches!(old.borrow(&gc), Err(LeafAccessError::NotLive)));
    assert!(matches!(alias.borrow(&gc), Err(LeafAccessError::NotLive)));
    let replacement = {
        let mut request = LeafAllocation::begin(
            &mut worker,
            &gc,
            0,
            1,
            Value {
                number: 801,
                drops: other_drops.clone(),
            },
        )
        .unwrap();
        (0..128).find_map(|_| request.poll().unwrap()).unwrap()
    };
    assert_eq!(view.number, 0);
    assert_eq!(replacement.borrow(&gc).unwrap().number, 801);
    drop(replacement);
    until_dropped(&mut maintenance, &other_drops, 2);
    assert_eq!(view.number, 0);
    assert!(gc.managed_bytes() > 0);
    // The last use of the view ends before its exact token can be dropped.
    drop(old);
    assert_eq!(old_drops.load(Ordering::Relaxed), 0);
    drop(alias);
    until_dropped(&mut maintenance, &old_drops, 1);
    empty(&mut worker, &catalog);
    assert_eq!(gc.managed_bytes(), 0);
    drop(maintenance);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
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

    #[test]
    fn maintenance_reclaims_another_payload_while_an_allocation_origin_is_stopped() {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget, mut worker) = configured(3, 2, 1);
        let drops = Arc::new(AtomicUsize::new(0));
        let other_drops = Arc::new(AtomicUsize::new(0));
        drop(allocate(&mut worker, &gc, 1, &other_drops));
        let record = prepare(
            &gc,
            Value {
                number: 802,
                drops: drops.clone(),
            },
            false,
        )
        .try_into_canonical_leaf()
        .unwrap_or_else(|_| panic!("private leaf"));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release = Release(release_tx);
        let resume = Mutex::new(release_rx);
        record.pause_after_admission(move || {
            entered_tx.send(()).unwrap();
            resume
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(60))
                .unwrap();
        });
        worker
            .submit_rooted_record(0, 0, record.address(), record)
            .unwrap();
        let held_gc = gc.clone();
        let stopped = std::thread::spawn(move || {
            assert!(matches!(
                finish(&mut worker, &held_gc),
                Progress::RootCreated(_)
            ));
            let root = worker.take_root().unwrap();
            assert_eq!(
                LeafRecord::borrow_root::<Value>(&root, &held_gc)
                    .unwrap()
                    .number,
                802
            );
            (worker, root)
        });
        entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
        let mut maintenance = LeafMaintenance::new(&catalog, &gc).unwrap();
        until_dropped(&mut maintenance, &other_drops, 1);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(catalog.registered(), (1, 1));
        drop(release);
        let (mut worker, root) = stopped.join().unwrap();
        drop(root);
        until_dropped(&mut maintenance, &drops, 1);
        empty(&mut worker, &catalog);
        assert_eq!(gc.managed_bytes(), 0);
        drop(maintenance);
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }

    struct BlockingDrop {
        entered: mpsc::Sender<()>,
        resume: Mutex<mpsc::Receiver<()>>,
        drops: Arc<AtomicUsize>,
    }
    // SAFETY: independent host synchronization and a counter, with no managed
    // edges or trace actions. This does NOT certify bounded destructor time.
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
    #[test]
    fn a_stopped_retired_destructor_keeps_its_credit_while_another_helper_reclaims() {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget, mut worker) = configured(3, 2, 2);
        let drops = Arc::new(AtomicUsize::new(0));
        let other_drops = Arc::new(AtomicUsize::new(0));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release = Release(release_tx);
        {
            let mut request = LeafAllocation::begin(
                &mut worker,
                &gc,
                0,
                0,
                BlockingDrop {
                    entered: entered_tx,
                    resume: Mutex::new(release_rx),
                    drops: drops.clone(),
                },
            )
            .unwrap();
            drop((0..128).find_map(|_| request.poll().unwrap()).unwrap());
        }
        let charged = gc.managed_bytes();
        let held_gc = gc.clone();
        let held_catalog = catalog.clone();
        let held_drops = drops.clone();
        let stopped = std::thread::spawn(move || {
            let mut helper = LeafMaintenance::new(&held_catalog, &held_gc).unwrap();
            until_dropped(&mut helper, &held_drops, 1);
        });
        entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
        assert_eq!(catalog.registered(), (1, 1));
        assert_eq!(gc.managed_bytes(), charged);
        let mut active = LeafMaintenance::new(&catalog, &gc).unwrap();
        drop(allocate(&mut worker, &gc, 1, &other_drops));
        until_dropped(&mut active, &other_drops, 1);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(gc.managed_bytes(), charged);
        assert_eq!(catalog.registered(), (1, 2));
        drop(release);
        stopped.join().unwrap();
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        empty(&mut worker, &catalog);
        drop(active);
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn a_finished_full_epoch_has_retired_every_unreachable_record_from_its_start() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = configured(17, 17, 1);
    let drops = Arc::new(AtomicUsize::new(0));
    let allocation_epoch = worker.phase_in(&gc).unwrap().domains[0].epoch;
    for slot in 0..17 {
        drop(allocate(&mut worker, &gc, slot, &drops));
    }
    let mut maintenance = LeafMaintenance::new(&catalog, &gc).unwrap();
    let mut full_epoch = None;
    let mut checked = false;
    for _ in 0..50_000 {
        maintenance.poll().unwrap();
        let Ok(phase) = worker.phase_in(&gc) else {
            continue;
        };
        let domain = phase.domains[0];
        if domain.mode == gc_catalog::DomainMode::Full && domain.epoch > allocation_epoch {
            full_epoch.get_or_insert(domain.epoch);
        }
        if Some(domain.epoch) == full_epoch && domain.mode == gc_catalog::DomainMode::Inactive {
            // Every original root was dropped before this Full epoch began. A
            // completed sweep must have logically retired each such allocation;
            // release of already Retired physical slots can follow afterward.
            for slot in 0..17 {
                if let Some(record) = worker.discover(slot).unwrap() {
                    assert_eq!(
                        record.membership,
                        Membership::Retired,
                        "finished epoch skipped object slot {slot}"
                    );
                }
            }
            checked = true;
            break;
        }
    }
    assert!(checked, "no full epoch completed");
    until_dropped(&mut maintenance, &drops, 17);
    empty(&mut worker, &catalog);
    assert_eq!(gc.managed_bytes(), 0);
    drop(maintenance);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}
