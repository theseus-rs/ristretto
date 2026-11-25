//! Cancellation, unsupported inputs and retained credit for real leaf storage.
use super::*;
use crate::canonical_catalog as gc_catalog;
use gc_catalog::bounded::{
    BoundedCatalog, Budget, Capacities, CatalogHandle, Footprint, GuardedCommand, Participant,
    Progress, RecordKey, RequestError,
};

type Worker = Participant<LeafRecord, GuardedCommand>;

fn bounded(mutators: usize) -> (CatalogHandle<LeafRecord>, Arc<Budget>) {
    let capacities = Capacities::derive(3, mutators, 1).unwrap();
    let footprint = Footprint::for_records::<LeafRecord>(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
    assert_eq!(catalog.footprint(), footprint);
    (catalog, budget)
}

fn finish(worker: &mut Worker, gc: &GarbageCollector) -> Progress {
    for _ in 0..4096 {
        let progress = worker.poll_in(gc);
        if progress != Progress::Pending {
            return progress;
        }
    }
    panic!("bounded leaf request did not settle");
}

fn create(
    worker: &mut Worker,
    gc: &GarbageCollector,
    slot: usize,
    record: LeafRecord,
) -> RecordKey {
    worker
        .submit_record(slot, record.address(), record)
        .unwrap();
    let Progress::Created(key) = finish(worker, gc) else {
        panic!("leaf creation failed");
    };
    key
}

fn run(worker: &mut Worker, gc: &GarbageCollector, command: GuardedCommand) {
    worker.submit(command).unwrap();
    assert_eq!(finish(worker, gc), Progress::Committed);
}

fn close_domain(worker: &mut Worker, gc: &GarbageCollector, domain: usize) {
    while worker.phase_in(gc).unwrap().domains[domain].floor < gc_catalog::RETIREMENT_FLOOR {
        for slot in 0..3 {
            let Some(record) = worker.discover(slot).unwrap() else {
                continue;
            };
            let Membership::Tracing(marks) = record.membership else {
                continue;
            };
            for (index, mark) in marks.marks.into_iter().enumerate() {
                if mark.ticket != 0 {
                    run(
                        worker,
                        gc,
                        GuardedCommand::Acknowledge {
                            key: record.key,
                            domain: index,
                            ticket: mark.ticket,
                        },
                    );
                }
            }
        }
        run(worker, gc, GuardedCommand::Advance { domain });
    }
}

fn retire(worker: &mut Worker, gc: &GarbageCollector, keys: &[RecordKey]) {
    for domain in 0..2 {
        close_domain(worker, gc, domain);
        run(worker, gc, GuardedCommand::Finish { domain });
    }
    run(
        worker,
        gc,
        GuardedCommand::Start {
            domain: 0,
            mode: gc_catalog::DomainMode::Full,
        },
    );
    close_domain(worker, gc, 0);
    for key in keys {
        run(
            worker,
            gc,
            GuardedCommand::Retire {
                key: *key,
                domain: 0,
            },
        );
    }
}

#[test]
fn bounded_preflight_rejection_returns_the_exact_real_payload() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget) = bounded(1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let first_drops = Arc::new(AtomicUsize::new(0));
    let second_drops = Arc::new(AtomicUsize::new(0));
    let first = leaf(&gc, &first_drops);
    worker.submit_record(0, first.address(), first).unwrap();
    let second = leaf(&gc, &second_drops);
    let identity = second.identity();
    let charge = gc.managed_bytes();
    let error = worker
        .submit_record(1, second.address(), second)
        .unwrap_err();
    assert_eq!(error.error, RequestError::Busy);
    assert_eq!(error.record.identity(), identity);
    assert_eq!(second_drops.load(Ordering::Relaxed), 0);
    assert_eq!(gc.managed_bytes(), charge);
    drop(error);
    assert_eq!(second_drops.load(Ordering::Relaxed), 1);
    assert!(worker.cancel());
    assert_eq!(first_drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn bounded_cancellation_at_every_creation_poll_preserves_discoverable_ownership() {
    for polls in 0..8 {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget) = bounded(1);
        let drops = Arc::new(AtomicUsize::new(0));
        let mut worker = catalog.register_record_mutator().unwrap();
        let record = leaf(&gc, &drops);
        worker.submit_record(0, record.address(), record).unwrap();
        for _ in 0..polls {
            if finish_one(&mut worker, &gc) {
                break;
            }
        }
        assert!(worker.cancel());
        if let Some(record) = worker.discover(0).unwrap() {
            assert_eq!(drops.load(Ordering::Relaxed), 0);
            assert_eq!(record.membership, Membership::Prepared);
            run(&mut worker, &gc, GuardedCommand::Abandon(record.key));
            assert!(worker.release_retired(record.key).unwrap());
        }
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        drop(worker);
        assert_eq!(catalog.registered(), (0, 0));
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

fn finish_one(worker: &mut Worker, gc: &GarbageCollector) -> bool {
    match worker.poll_in(gc) {
        Progress::Pending => false,
        Progress::Created(_) => true,
        other => panic!("unexpected creation poll: {other:?}"),
    }
}

#[test]
fn discovery_collision_and_foreign_publication_leave_the_winner_owned() {
    let gc = collector(true, RootStorage::Slots);
    let foreign = collector(true, RootStorage::Slots);
    let (catalog, budget) = bounded(2);
    let mut first = catalog.register_record_mutator().unwrap();
    let mut second = catalog.register_record_mutator().unwrap();
    let winner_drops = Arc::new(AtomicUsize::new(0));
    let loser_drops = Arc::new(AtomicUsize::new(0));
    let winner = leaf(&gc, &winner_drops);
    let loser = leaf(&gc, &loser_drops);
    first.submit_record(0, winner.address(), winner).unwrap();
    second.submit_record(0, loser.address(), loser).unwrap();
    assert_eq!(first.poll_in(&gc), Progress::Pending);
    assert_eq!(second.poll_in(&gc), Progress::Pending);
    let Progress::Created(key) = finish(&mut first, &gc) else {
        panic!("creation")
    };
    assert_eq!(
        finish(&mut second, &gc),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(loser_drops.load(Ordering::Relaxed), 1);
    assert_eq!(winner_drops.load(Ordering::Relaxed), 0);
    first.submit(GuardedCommand::Publish(key)).unwrap();
    assert_eq!(finish(&mut first, &foreign), Progress::Cancelled);
    assert_eq!(first.observe(key).unwrap().membership, Membership::Prepared);
    assert_eq!(
        first.phase_in(&gc).unwrap().domains.map(|d| d.pending),
        [0, 0]
    );
    run(&mut first, &gc, GuardedCommand::Publish(key));
    retire(&mut first, &gc, &[key]);
    assert!(first.release_retired(key).unwrap());
    assert_eq!(winner_drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(first);
    drop(second);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn leaf_certification_does_not_discard_heap_shutdown_callbacks() {
    struct Owner(Arc<AtomicUsize>);
    impl crate::HeapOwner for Owner {
        fn release_heap_roots(&self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    struct WithOwner(Arc<Owner>, Arc<AtomicUsize>);
    // SAFETY: only ordinary counters; the separate shutdown callback still exists.
    unsafe impl Trace for WithOwner {
        fn may_contain_gc_references() -> bool {
            false
        }
        fn trace(&self, _: &GarbageCollector) {}
        fn heap_owner(&self) -> Option<Arc<dyn crate::HeapOwner>> {
            Some(self.0.clone())
        }
    }
    impl Drop for WithOwner {
        fn drop(&mut self) {
            self.1.fetch_add(1, Ordering::Relaxed);
        }
    }
    let gc = collector(true, RootStorage::Slots);
    let releases = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let publication = prepare(
        &gc,
        WithOwner(Arc::new(Owner(releases.clone())), drops.clone()),
        false,
    );
    let identity = publication.metadata().id;
    let publication = publication
        .try_into_canonical_leaf()
        .err()
        .expect("shutdown owner");
    assert_eq!(publication.metadata().id, identity);
    let value = publication.finish_unrooted(&gc);
    drop(value);
    gc.release_vm_roots();
    drop(gc);
    assert_eq!(releases.load(Ordering::Relaxed), 1);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
}

#[test]
fn an_opaque_trace_hook_is_rejected_even_without_captured_edges() {
    let gc = collector(true, RootStorage::Contexts);
    let drops = Arc::new(AtomicUsize::new(0));
    // Parent fixture Leaf intentionally keeps Trace's default capability. Its
    // empty current graph cannot authorize skipping an opaque tracing action.
    let publication = prepare(&gc, Leaf(drops.clone()), false);
    assert!(publication.construction.is_empty());
    assert!(publication.registration.is_none());
    let identity = publication.metadata().id;
    let publication = publication
        .try_into_canonical_leaf()
        .err()
        .expect("opaque trace hook");
    assert_eq!(publication.metadata().id, identity);
    drop(publication);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn leaf_certification_does_not_discard_finalization() {
    struct Finalized(Arc<AtomicUsize>, Arc<AtomicUsize>);
    // SAFETY: only independent counters, no managed edges or tracing actions.
    unsafe impl Trace for Finalized {
        fn may_contain_gc_references() -> bool {
            false
        }
        fn trace(&self, _: &GarbageCollector) {}
    }
    impl crate::Finalize for Finalized {
        fn finalize(&self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }
    impl Drop for Finalized {
        fn drop(&mut self) {
            assert_eq!(self.0.load(Ordering::Relaxed), 1);
            self.1.fetch_add(1, Ordering::Relaxed);
        }
    }
    let gc = collector(true, RootStorage::Slots);
    let finalized = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let (handle, allocation, reservation) =
        Gc::allocate(&gc, Finalized(finalized.clone(), drops.clone()));
    let metadata = gc.prepare_allocated_object_with_finalizer(allocation, reservation, handle.id);
    let identity = metadata.id;
    let publication = Publication::new(
        &gc,
        handle,
        metadata,
        Detached::from_scope(&gc, None),
        false,
    );
    let publication = publication
        .try_into_canonical_leaf()
        .err()
        .expect("finalizer");
    assert_eq!(publication.metadata().id, identity);
    assert_eq!(finalized.load(Ordering::Relaxed), 0);
    drop(publication);
    assert_eq!(finalized.load(Ordering::Relaxed), 1);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
}

#[test]
fn unrelated_captured_construction_protection_prevents_leaf_transfer() {
    let gc = collector(true, RootStorage::Contexts);
    let child_drops = Arc::new(AtomicUsize::new(0));
    let parent_drops = Arc::new(AtomicUsize::new(0));
    let mut roots = crate::construction::Roots::new(&gc);
    let scope = roots.enter();
    let child = Gc::new(&gc, CertifiedLeaf(child_drops.clone()));
    drop(child);
    let (handle, allocation, reservation) = Gc::allocate(&gc, CertifiedLeaf(parent_drops.clone()));
    let metadata = gc.prepare_allocated_object(allocation, reservation, handle.id);
    let publication = Publication::new(&gc, handle, metadata, scope.detach(), false);
    assert!(!publication.construction.is_empty());
    let publication = publication
        .try_into_canonical_leaf()
        .err()
        .expect("captured roots");
    reclaim(&gc).unwrap();
    assert_eq!(child_drops.load(Ordering::Relaxed), 0);
    drop(publication);
    reclaim(&gc).unwrap();
    assert_eq!(child_drops.load(Ordering::Relaxed), 1);
    assert_eq!(parent_drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
}

#[cfg(not(target_family = "wasm"))]
mod native {
    use super::*;
    use gc_catalog::bounded::RegisterError;
    use std::sync::{Mutex, mpsc};
    use std::time::Duration;

    // Resume even if an assertion fails, so the failure cannot strand a worker.
    struct Release(Option<mpsc::Sender<()>>);
    impl Drop for Release {
        fn drop(&mut self) {
            if let Some(sender) = self.0.take() {
                let _ = sender.send(());
            }
        }
    }

    #[test]
    fn a_reserved_helper_reclaims_real_payloads_while_record_admission_is_stopped() {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget) = bounded(1);
        let mut origin = catalog.register_record_mutator().unwrap();
        let mut helper = catalog.register_record_helper().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        let other_drops = Arc::new(AtomicUsize::new(0));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release = Release(Some(release_tx));
        let release_rx = Mutex::new(release_rx);
        let record = leaf(&gc, &drops);
        record.pause_after_admission(move || {
            entered_tx.send(()).unwrap();
            release_rx
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(60))
                .unwrap();
        });
        let key = create(&mut origin, &gc, 0, record);
        origin.submit(GuardedCommand::Publish(key)).unwrap();
        let heap = gc.clone();
        let stopped = std::thread::spawn(move || {
            assert_eq!(finish(&mut origin, &heap), Progress::Committed);
        });
        entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
        assert_eq!(helper.phase_in(&gc), Err(RequestError::Contended));
        assert_eq!(
            helper.phase_in(&gc).unwrap().domains.map(|d| d.pending),
            [1, 1]
        );
        let other = create(&mut helper, &gc, 1, leaf(&gc, &other_drops));
        run(&mut helper, &gc, GuardedCommand::Publish(other));
        retire(&mut helper, &gc, &[key, other]);
        assert!(helper.release_retired(key).unwrap());
        assert!(helper.release_retired(other).unwrap());
        assert_eq!(
            drops.load(Ordering::Relaxed),
            0,
            "stopped exact guard lost storage"
        );
        assert_eq!(other_drops.load(Ordering::Relaxed), 1);
        let replacement = create(&mut helper, &gc, 0, leaf(&gc, &other_drops));
        assert_ne!(replacement, key);
        assert_eq!(
            helper.observe(replacement).unwrap().membership,
            Membership::Prepared
        );
        drop(release);
        stopped.join().unwrap();
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(
            helper.observe(replacement).unwrap().membership,
            Membership::Prepared
        );
        run(&mut helper, &gc, GuardedCommand::Abandon(replacement));
        assert!(helper.release_retired(replacement).unwrap());
        assert_eq!(other_drops.load(Ordering::Relaxed), 2);
        assert_eq!(gc.managed_bytes(), 0);
        drop(helper);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }

    #[test]
    fn stopped_retired_payload_destruction_retains_credit_and_participant_quota() {
        struct StoppingLeaf {
            entered: mpsc::Sender<()>,
            release: Mutex<mpsc::Receiver<()>>,
            drops: Arc<AtomicUsize>,
        }
        // SAFETY: only ordinary synchronization and counters, no managed edges.
        unsafe impl Trace for StoppingLeaf {
            fn may_contain_gc_references() -> bool {
                false
            }
            fn trace(&self, _: &GarbageCollector) {}
        }
        impl Drop for StoppingLeaf {
            fn drop(&mut self) {
                self.entered.send(()).unwrap();
                self.release
                    .lock()
                    .unwrap()
                    .recv_timeout(Duration::from_secs(60))
                    .unwrap();
                self.drops.fetch_add(1, Ordering::Release);
            }
        }
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget) = bounded(1);
        let catalog_charge = budget.used();
        let mut origin = catalog.register_record_mutator().unwrap();
        let mut helper = catalog.register_record_helper().unwrap();
        let drops = Arc::new(AtomicUsize::new(0));
        let other_drops = Arc::new(AtomicUsize::new(0));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release = Release(Some(release_tx));
        let record = prepare(
            &gc,
            StoppingLeaf {
                entered: entered_tx,
                release: Mutex::new(release_rx),
                drops: drops.clone(),
            },
            false,
        )
        .try_into_canonical_leaf()
        .unwrap_or_else(|_| panic!("stopping leaf"));
        let payload_charge = gc.managed_bytes();
        let key = create(&mut origin, &gc, 0, record);
        run(&mut origin, &gc, GuardedCommand::Publish(key));
        retire(&mut origin, &gc, &[key]);
        let stopped = std::thread::spawn(move || assert!(origin.release_retired(key).unwrap()));
        entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
        assert_eq!(drops.load(Ordering::Acquire), 0);
        assert_eq!(catalog.registered(), (1, 1));
        assert!(matches!(
            catalog.register_record_mutator(),
            Err(RegisterError::Limit)
        ));
        assert_eq!(budget.used(), catalog_charge);
        assert_eq!(gc.managed_bytes(), payload_charge);
        let other = create(&mut helper, &gc, 0, leaf(&gc, &other_drops));
        assert_ne!(other, key);
        run(&mut helper, &gc, GuardedCommand::Publish(other));
        close_domain(&mut helper, &gc, 0);
        run(&mut helper, &gc, GuardedCommand::Finish { domain: 0 });
        run(
            &mut helper,
            &gc,
            GuardedCommand::Start {
                domain: 0,
                mode: gc_catalog::DomainMode::Full,
            },
        );
        close_domain(&mut helper, &gc, 0);
        run(
            &mut helper,
            &gc,
            GuardedCommand::Retire {
                key: other,
                domain: 0,
            },
        );
        assert!(helper.release_retired(other).unwrap());
        assert_eq!(other_drops.load(Ordering::Relaxed), 1);
        let legacy = Gc::new(&gc, CertifiedLeaf(other_drops.clone()));
        drop(legacy);
        reclaim(&gc).unwrap();
        assert_eq!(other_drops.load(Ordering::Relaxed), 2);
        assert_eq!(gc.managed_bytes(), payload_charge);
        drop(release);
        stopped.join().unwrap();
        assert_eq!(drops.load(Ordering::Acquire), 1);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(catalog.registered(), (0, 1));
        drop(helper);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }

    #[test]
    fn panicking_leaf_destructor_still_frees_payload_and_refunds_once() {
        struct Panicking(CertifiedLeaf);
        // SAFETY: this fixture contains only an ordinary destruction counter.
        unsafe impl Trace for Panicking {
            fn may_contain_gc_references() -> bool {
                false
            }
            fn trace(&self, _: &GarbageCollector) {}
        }
        impl Drop for Panicking {
            fn drop(&mut self) {
                panic!("injected leaf destructor failure");
            }
        }
        let gc = collector(true, RootStorage::Contexts);
        let drops = Arc::new(AtomicUsize::new(0));
        let record = prepare(&gc, Panicking(CertifiedLeaf(drops.clone())), false)
            .try_into_canonical_leaf()
            .unwrap_or_else(|_| panic!("panicking leaf"));
        assert!(gc.managed_bytes() > 0);
        drop(record);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        reclaim(&gc).unwrap();
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
}
