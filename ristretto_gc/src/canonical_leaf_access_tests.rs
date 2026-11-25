//! Typed reads borrow one exact canonical root, its leaf storage and its heap.
use super::*;
use crate::canonical_catalog as gc_catalog;
use crate::gc::publication_owner::canonical_leaf::LeafAccessError;
use gc_catalog::bounded::{
    BoundedCatalog, Budget, Capacities, CatalogHandle, Footprint, GuardedCommand, Participant,
    Progress, RecordKey, RootCommand, RootHandle,
};

#[path = "canonical_leaf_allocation_tests.rs"]
mod allocation;

#[path = "canonical_leaf_maintenance_tests.rs"]
mod maintenance;

#[path = "canonical_leaf_runtime_tests.rs"]
mod runtime;

struct Value {
    number: u64,
    drops: Arc<AtomicUsize>,
}
// SAFETY: only a number and an independently owned counter; no managed edges.
unsafe impl Trace for Value {
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

type Worker = Participant<LeafRecord, GuardedCommand>;
fn setup() -> (CatalogHandle<LeafRecord>, Arc<Budget>, Worker) {
    let capacities = Capacities::derive(3, 1, 1).unwrap().with_roots(2).unwrap();
    let footprint = Footprint::for_records::<LeafRecord>(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
    let worker = catalog.register_record_mutator().unwrap();
    (catalog, budget, worker)
}
fn finish(worker: &mut Worker, gc: &GarbageCollector) -> Progress {
    for _ in 0..4096 {
        let result = worker.poll_in(gc);
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("rooted leaf operation did not settle");
}
fn run(worker: &mut Worker, gc: &GarbageCollector, command: GuardedCommand) {
    worker.submit(command).unwrap();
    assert_eq!(finish(worker, gc), Progress::Committed);
}
fn roots(worker: &mut Worker, gc: &GarbageCollector, command: RootCommand) {
    worker.submit_root(command).unwrap();
    assert_eq!(finish(worker, gc), Progress::Committed);
}
fn create(
    worker: &mut Worker,
    gc: &GarbageCollector,
    slot: usize,
    number: u64,
    drops: &Arc<AtomicUsize>,
) -> RecordKey {
    let record = prepare(
        gc,
        Value {
            number,
            drops: drops.clone(),
        },
        false,
    )
    .try_into_canonical_leaf()
    .unwrap_or_else(|_| panic!("private value"));
    worker
        .submit_record(slot, record.address(), record)
        .unwrap();
    let Progress::Created(key) = finish(worker, gc) else {
        panic!("record creation")
    };
    key
}
fn root(
    worker: &mut Worker,
    gc: &GarbageCollector,
    slot: usize,
    key: RecordKey,
) -> RootHandle<LeafRecord> {
    worker.submit_root_creation(slot, key).unwrap();
    assert!(matches!(finish(worker, gc), Progress::RootCreated(_)));
    worker.take_root().expect("unique prepared root")
}
fn close(worker: &mut Worker, gc: &GarbageCollector, domain: usize) {
    while worker.phase_in(gc).unwrap().domains[domain].floor < gc_catalog::RETIREMENT_FLOOR {
        while worker.phase_in(gc).unwrap().root_cursors[domain] < 2 {
            roots(worker, gc, RootCommand::Scan { domain });
        }
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
fn next_full_cycle(worker: &mut Worker, gc: &GarbageCollector) {
    for domain in 0..2 {
        close(worker, gc, domain);
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
    close(worker, gc, 0);
}

#[test]
fn typed_read_requires_live_publication_matching_heap_and_exact_type() {
    let gc = collector(true, RootStorage::Contexts);
    let foreign = collector(true, RootStorage::Slots);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let key = create(&mut worker, &gc, 0, 0x1234_5678_9abc_def0, &drops);
    let root = root(&mut worker, &gc, 0, key);
    assert!(matches!(
        LeafRecord::borrow_root::<Value>(&root, &gc),
        Err(LeafAccessError::NotLive)
    ));
    roots(&mut worker, &gc, RootCommand::Publish(root.key()));
    {
        let view = LeafRecord::borrow_root::<Value>(&root, &gc).expect("published typed leaf");
        assert_eq!(view.number, 0x1234_5678_9abc_def0);
        assert!(matches!(
            LeafRecord::borrow_root::<Value>(&root, &foreign),
            Err(LeafAccessError::Admission(RecordError::Foreign))
        ));
        assert!(matches!(
            LeafRecord::borrow_root::<u64>(&root, &gc),
            Err(LeafAccessError::WrongType)
        ));
        reclaim(&gc).unwrap();
        assert_eq!(view.number, 0x1234_5678_9abc_def0);
        assert_eq!(drops.load(Ordering::Relaxed), 0);
    }
    drop(root);
    drop(worker);
    drop(catalog);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn live_status_without_a_publication_certificate_does_not_grant_typed_access() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let key = create(&mut worker, &gc, 0, 27, &drops);
    let root = root(&mut worker, &gc, 0, key);
    roots(&mut worker, &gc, RootCommand::Publish(root.key()));
    let record = root.borrow_live_record().expect("live root storage");
    // Synthetic access precondition: the generic controller's precise
    // Root-Live-before-record-certificate interval has no runtime pause hook.
    // This checks the leaf view's independent certificate requirement, not a
    // claim that the actual controller reset an already completed publication.
    record.set_certificate_for_test(false);
    assert!(matches!(
        LeafRecord::borrow_root::<Value>(&root, &gc),
        Err(LeafAccessError::NotPublished)
    ));
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    record.set_certificate_for_test(true);
    assert_eq!(
        LeafRecord::borrow_root::<Value>(&root, &gc)
            .expect("certified leaf")
            .number,
        27
    );
    drop(root);
    drop(worker);
    drop(catalog);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[test]
fn revocation_excludes_new_reads_but_an_existing_view_survives_retirement_and_reuse() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let other_drops = Arc::new(AtomicUsize::new(0));
    let key = create(&mut worker, &gc, 0, 91, &drops);
    let root = root(&mut worker, &gc, 0, key);
    let root_key = root.key();
    roots(&mut worker, &gc, RootCommand::Publish(root_key));
    let replacement = {
        let view = LeafRecord::borrow_root::<Value>(&root, &gc).expect("live leaf");
        roots(&mut worker, &gc, RootCommand::Remove(root_key));
        assert!(matches!(
            LeafRecord::borrow_root::<Value>(&root, &gc),
            Err(LeafAccessError::NotLive)
        ));
        let other = create(&mut worker, &gc, 1, 92, &other_drops);
        run(&mut worker, &gc, GuardedCommand::Publish(other));
        next_full_cycle(&mut worker, &gc);
        for target in [key, other] {
            run(
                &mut worker,
                &gc,
                GuardedCommand::Retire {
                    key: target,
                    domain: 0,
                },
            );
            assert!(worker.release_retired(target).unwrap());
        }
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(other_drops.load(Ordering::Relaxed), 1);
        assert!(!worker.release_retired_root(root_key).unwrap());
        let replacement = create(&mut worker, &gc, 0, 93, &other_drops);
        assert_ne!(replacement, key);
        assert_eq!(view.number, 91);
        assert_eq!(catalog.reserved_root_tokens(), 1);
        replacement
    };
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(root);
    assert!(worker.release_retired_root(root_key).unwrap());
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    run(&mut worker, &gc, GuardedCommand::Abandon(replacement));
    assert!(worker.release_retired(replacement).unwrap());
    assert_eq!(other_drops.load(Ordering::Relaxed), 2);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn a_root_can_survive_its_heap_without_allowing_a_foreign_heap_to_read_it() {
    let gc = collector(true, RootStorage::Contexts);
    let weak = Arc::downgrade(&gc);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let key = create(&mut worker, &gc, 0, 11, &drops);
    let root = root(&mut worker, &gc, 0, key);
    roots(&mut worker, &gc, RootCommand::Publish(root.key()));
    drop(gc);
    assert!(weak.upgrade().is_none());
    let foreign = collector(true, RootStorage::Slots);
    assert!(matches!(
        LeafRecord::borrow_root::<Value>(&root, &foreign),
        Err(LeafAccessError::Admission(RecordError::Foreign))
    ));
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(worker);
    drop(catalog);
    assert!(budget.used() > 0);
    drop(root);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(budget.used(), 0);
}

#[test]
fn a_helper_reads_the_certified_root_before_the_original_publication_returns() {
    use std::sync::Mutex;
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut origin) = setup();
    let catalog = Arc::new(catalog);
    let drops = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicBool::new(false));
    let root_slot = Arc::new(Mutex::new(Weak::<RootHandle<LeafRecord>>::new()));
    let record = prepare(
        &gc,
        Value {
            number: 123,
            drops: drops.clone(),
        },
        false,
    )
    .try_into_canonical_leaf()
    .unwrap_or_else(|_| panic!("private leaf"));
    let weak_catalog = Arc::downgrade(&catalog);
    let weak_heap = Arc::downgrade(&gc);
    let helper_root = root_slot.clone();
    let completed = done.clone();
    // Weak captures avoid a record -> callback -> catalog/root ownership cycle
    // if preparation were cancelled before this test's publication attempt.
    record.pause_after_admission(move || {
        let catalog = weak_catalog.upgrade().expect("origin owns catalog");
        let gc = weak_heap.upgrade().expect("publication borrows heap");
        let root = helper_root
            .lock()
            .unwrap()
            .upgrade()
            .expect("origin owns root");
        let mut helper = catalog.register_record_helper().unwrap();
        assert_eq!(
            helper.phase_in(&gc),
            Err(gc_catalog::bounded::RequestError::Contended)
        );
        let view = LeafRecord::borrow_root::<Value>(&root, &gc).expect("helper certified root");
        assert_eq!(view.number, 123);
        completed.store(true, Ordering::Release);
    });
    origin.submit_record(0, record.address(), record).unwrap();
    let Progress::Created(key) = finish(&mut origin, &gc) else {
        panic!("creation")
    };
    let root = Arc::new(root(&mut origin, &gc, 0, key));
    *root_slot.lock().unwrap() = Arc::downgrade(&root);
    roots(&mut origin, &gc, RootCommand::Publish(root.key()));
    assert!(done.load(Ordering::Acquire));
    assert_eq!(
        LeafRecord::borrow_root::<Value>(&root, &gc)
            .expect("origin result")
            .number,
        123
    );
    drop(root);
    drop(origin);
    drop(catalog);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(budget.used(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn typed_access_pauses_retain_only_their_root_while_a_reserved_helper_reclaims_others() {
    use std::sync::{Mutex, mpsc};
    use std::time::Duration;
    struct Release(mpsc::Sender<()>);
    impl Drop for Release {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }
    for pause_in_admission in [false, true] {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget, mut origin) = setup();
        let drops = Arc::new(AtomicUsize::new(0));
        let other_drops = Arc::new(AtomicUsize::new(0));
        let key = create(&mut origin, &gc, 0, 101, &drops);
        let root = Arc::new(root(&mut origin, &gc, 0, key));
        let root_key = root.key();
        roots(&mut origin, &gc, RootCommand::Publish(root_key));
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release = Release(release_tx);
        let release_rx = Arc::new(Mutex::new(release_rx));
        if pause_in_admission {
            let entered = entered_tx.clone();
            let resume = release_rx.clone();
            root.borrow_live_record()
                .expect("published record")
                .pause_after_admission(move || {
                    entered.send(()).unwrap();
                    resume
                        .lock()
                        .unwrap()
                        .recv_timeout(Duration::from_secs(60))
                        .unwrap();
                });
        }
        let held_root = root.clone();
        let held_heap = gc.clone();
        let stopped = std::thread::spawn(move || {
            let view = LeafRecord::borrow_root::<Value>(&held_root, &held_heap)
                .expect("admitted leaf read");
            if !pause_in_admission {
                entered_tx.send(()).unwrap();
                release_rx
                    .lock()
                    .unwrap()
                    .recv_timeout(Duration::from_secs(60))
                    .unwrap();
            }
            // In the admission-pause case, revocation precedes return of the
            // view. Its admission began at the earlier Live observation. Both
            // schedules retain the original exact token and storage throughout.
            assert_eq!(view.number, 101);
        });
        entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
        drop(origin);
        let mut helper = catalog.register_record_helper().unwrap();
        assert_eq!(catalog.registered(), (0, 1));
        roots(&mut helper, &gc, RootCommand::Remove(root_key));
        assert!(matches!(
            LeafRecord::borrow_root::<Value>(&root, &gc),
            Err(LeafAccessError::NotLive)
        ));
        let other = create(&mut helper, &gc, 1, 102, &other_drops);
        run(&mut helper, &gc, GuardedCommand::Publish(other));
        next_full_cycle(&mut helper, &gc);
        for target in [key, other] {
            run(
                &mut helper,
                &gc,
                GuardedCommand::Retire {
                    key: target,
                    domain: 0,
                },
            );
            assert!(helper.release_retired(target).unwrap());
        }
        assert_eq!(drops.load(Ordering::Relaxed), 0);
        assert_eq!(other_drops.load(Ordering::Relaxed), 1);
        let replacement = create(&mut helper, &gc, 0, 103, &other_drops);
        assert_ne!(replacement, key);
        run(&mut helper, &gc, GuardedCommand::Abandon(replacement));
        assert!(helper.release_retired(replacement).unwrap());
        assert_eq!(other_drops.load(Ordering::Relaxed), 2);
        drop(root);
        assert_eq!(catalog.reserved_root_tokens(), 1);
        assert!(!helper.release_retired_root(root_key).unwrap());
        drop(release);
        stopped.join().unwrap();
        assert!(helper.release_retired_root(root_key).unwrap());
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(gc.managed_bytes(), 0);
        assert_eq!(catalog.reserved_root_tokens(), 0);
        drop(helper);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[path = "canonical_leaf_retry_tests.rs"]
mod retry;

#[path = "canonical_leaf_contention_tests.rs"]
mod contention;
