//! Actual payloads through one bounded rooted-allocation request, including
//! cancellation at every observed poll prefix and exact-slot collisions.
use super::*;
use crate::canonical_catalog as gc_catalog;
use gc_catalog::RootStatus;
use gc_catalog::bounded::RequestError;

fn record(gc: &GarbageCollector, drops: &Arc<AtomicUsize>, number: u64) -> LeafRecord {
    prepare(
        gc,
        Value {
            number,
            drops: drops.clone(),
        },
        false,
    )
    .try_into_canonical_leaf()
    .unwrap_or_else(|_| panic!("private leaf"))
}
fn submit(
    worker: &mut Worker,
    gc: &GarbageCollector,
    drops: &Arc<AtomicUsize>,
    slot: usize,
    root_slot: usize,
    number: u64,
) {
    let record = record(gc, drops, number);
    worker
        .submit_rooted_record(slot, root_slot, record.address(), record)
        .unwrap();
}
fn drain(worker: &mut Worker, gc: &GarbageCollector) {
    // First help an already committed controller, then consume exact monotonic
    // cancellation requests. Unrequested Prepared cells must remain untouched.
    worker.phase_in(gc).unwrap();
    for slot in 0..2 {
        if let Some(root) = worker.discover_root(slot).unwrap() {
            assert!(root.removal_requested, "test released every root token");
            if root.status != RootStatus::Retired {
                roots(worker, gc, RootCommand::RemoveRequested(root.key));
            }
            assert!(worker.release_retired_root(root.key).unwrap());
        }
    }
    for slot in 0..3 {
        if let Some(cell) = worker.discover(slot).unwrap()
            && cell.membership == Membership::Prepared
        {
            assert!(
                cell.abandon_requested,
                "cancelled creation must be discoverable"
            );
            run(worker, gc, GuardedCommand::Abandon(cell.key));
            assert!(worker.release_retired(cell.key).unwrap());
        }
    }
    for domain in 0..2 {
        if worker.phase_in(gc).unwrap().domains[domain].mode != gc_catalog::DomainMode::Inactive {
            close(worker, gc, domain);
            run(worker, gc, GuardedCommand::Finish { domain });
        }
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
    for slot in 0..3 {
        if let Some(cell) = worker.discover(slot).unwrap() {
            run(
                worker,
                gc,
                GuardedCommand::Retire {
                    key: cell.key,
                    domain: 0,
                },
            );
            assert!(worker.release_retired(cell.key).unwrap());
        }
    }
}

#[test]
fn one_request_returns_only_the_certified_root_and_transfers_it_once() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    submit(&mut worker, &gc, &drops, 0, 0, 701);
    assert_eq!(catalog.reserved_root_tokens(), 1);
    let mut polls = 0;
    let key = loop {
        assert!(
            worker.take_root().is_none(),
            "no prepared result may escape"
        );
        polls += 1;
        assert!(polls < 128);
        match worker.poll_in(&gc) {
            Progress::Pending => {}
            Progress::RootCreated(key) => break key,
            other => panic!("allocation failed: {other:?}"),
        }
    };
    assert_eq!(worker.poll_in(&gc), Progress::RootCreated(key));
    assert_eq!(worker.phase_in(&gc), Err(RequestError::Busy));
    let root = worker.take_root().expect("one certified result");
    assert!(worker.take_root().is_none());
    assert_eq!(root.key(), key);
    assert_eq!(root.observe().status, RootStatus::Live);
    assert!(
        !worker
            .observe(root.observe().target)
            .unwrap()
            .abandon_requested
    );
    assert_eq!(
        LeafRecord::borrow_root::<Value>(&root, &gc).unwrap().number,
        701
    );
    assert!(
        gc.metadata_for_address(root.borrow_live_record().unwrap().address())
            .is_none()
    );
    reclaim(&gc).unwrap();
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(root);
    drain(&mut worker, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn every_allocation_poll_prefix_can_cancel_or_drop_with_helper_cleanup() {
    // Includes all private preparation, announced-cell, announced-root,
    // publication preparation and completed-but-untaken result boundaries.
    for drop_participant in [false, true] {
        for prefix in 0..24 {
            let gc = collector(true, RootStorage::Contexts);
            let (catalog, budget, mut worker) = setup();
            let drops = Arc::new(AtomicUsize::new(0));
            submit(&mut worker, &gc, &drops, 0, 0, 702);
            for _ in 0..prefix {
                assert!(matches!(
                    worker.poll_in(&gc),
                    Progress::Pending | Progress::RootCreated(_)
                ));
            }
            if !drop_participant {
                assert!(
                    worker.cancel(),
                    "no concurrent commit remains in this schedule"
                );
                assert_eq!(worker.poll_in(&gc), Progress::Idle);
            }
            drop(worker);
            let mut helper = catalog.register_record_helper().unwrap();
            drain(&mut helper, &gc);
            assert_eq!(
                drops.load(Ordering::Relaxed),
                1,
                "prefix {prefix}, drop={drop_participant}"
            );
            assert_eq!(catalog.reserved_root_tokens(), 0);
            assert_eq!(gc.managed_bytes(), 0);
            drop(helper);
            drop(catalog);
            assert_eq!(budget.used(), 0);
        }
    }
}

#[test]
fn an_occupied_root_slot_leaves_only_the_losing_prepared_record_for_cleanup() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let winner_drops = Arc::new(AtomicUsize::new(0));
    let loser_drops = Arc::new(AtomicUsize::new(0));
    submit(&mut worker, &gc, &winner_drops, 0, 0, 703);
    assert!(matches!(finish(&mut worker, &gc), Progress::RootCreated(_)));
    let root = worker.take_root().unwrap();
    submit(&mut worker, &gc, &loser_drops, 1, 0, 704);
    assert_eq!(
        finish(&mut worker, &gc),
        Progress::Rejected(RequestError::Conflict)
    );
    assert_eq!(catalog.reserved_root_tokens(), 1);
    let losing = worker.discover(1).unwrap().unwrap();
    assert_eq!(losing.membership, Membership::Prepared);
    assert!(losing.abandon_requested);
    run(&mut worker, &gc, GuardedCommand::Abandon(losing.key));
    assert!(worker.release_retired(losing.key).unwrap());
    assert_eq!(loser_drops.load(Ordering::Relaxed), 1);
    assert_eq!(winner_drops.load(Ordering::Relaxed), 0);
    assert_eq!(
        LeafRecord::borrow_root::<Value>(&root, &gc).unwrap().number,
        703
    );
    drop(root);
    drain(&mut worker, &gc);
    assert_eq!(winner_drops.load(Ordering::Relaxed), 1);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn preflight_failure_returns_the_exact_payload_without_reserving_another_root() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    for (slot, root_slot, address) in [(3, 0, 8), (0, 2, 8), (0, 0, 0)] {
        let value = record(&gc, &drops, 705);
        let identity = value.identity();
        let failure = worker
            .submit_rooted_record(slot, root_slot, address, value)
            .unwrap_err();
        assert_eq!(failure.record.identity(), identity);
        assert_eq!(catalog.reserved_root_tokens(), 0);
        drop(failure);
    }
    submit(&mut worker, &gc, &drops, 0, 0, 706);
    let value = record(&gc, &drops, 707);
    let identity = value.identity();
    let failure = worker
        .submit_rooted_record(1, 1, value.address(), value)
        .unwrap_err();
    assert_eq!(failure.error, RequestError::Busy);
    assert_eq!(failure.record.identity(), identity);
    assert_eq!(catalog.reserved_root_tokens(), 1);
    drop(failure);
    assert!(worker.cancel());
    assert_eq!(drops.load(Ordering::Relaxed), 5);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn foreign_publication_cancellation_preserves_cleanup_without_a_typed_result() {
    let gc = collector(true, RootStorage::Contexts);
    let foreign = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    submit(&mut worker, &gc, &drops, 0, 0, 708);
    assert_eq!(finish(&mut worker, &foreign), Progress::Cancelled);
    assert!(worker.take_root().is_none());
    assert_eq!(catalog.reserved_root_tokens(), 0);
    drain(&mut worker, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

use crate::gc::publication_owner::canonical_leaf::allocation::{AllocationError, LeafAllocation};

#[test]
fn real_preparation_and_typed_result_transfer_use_one_backend() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let root = {
        let mut request = LeafAllocation::begin(
            &mut worker,
            &gc,
            0,
            0,
            Value {
                number: 709,
                drops: drops.clone(),
            },
        )
        .unwrap();
        let root = (0..128)
            .find_map(|_| request.poll().unwrap())
            .expect("typed completion");
        assert!(matches!(request.poll(), Err(AllocationError::Finished)));
        root
    };
    assert_eq!(root.borrow(&gc).unwrap().number, 709);
    assert_eq!(catalog.reserved_root_tokens(), 1);
    assert_eq!(worker.poll_in(&gc), Progress::Idle);
    reclaim(&gc).unwrap();
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(root);
    drain(&mut worker, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn typed_request_drop_at_every_prefix_releases_the_participant_for_helper_cleanup() {
    for prefix in 0..24 {
        let gc = collector(true, RootStorage::Contexts);
        let (catalog, budget, mut worker) = setup();
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let mut request = LeafAllocation::begin(
                &mut worker,
                &gc,
                0,
                0,
                Value {
                    number: 710,
                    drops: drops.clone(),
                },
            )
            .unwrap();
            for _ in 0..prefix {
                if let Some(root) = request.poll().unwrap() {
                    assert_eq!(root.borrow(&gc).unwrap().number, 710);
                    drop(root);
                    break;
                }
            }
        }
        assert_eq!(worker.poll_in(&gc), Progress::Idle);
        drain(&mut worker, &gc);
        assert_eq!(drops.load(Ordering::Relaxed), 1, "prefix {prefix}");
        assert_eq!(catalog.reserved_root_tokens(), 0);
        assert_eq!(gc.managed_bytes(), 0);
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn typed_allocation_rejection_refunds_heap_credit_and_keeps_existing_roots() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut roots = Vec::new();
    for slot in 0..2 {
        let mut request = LeafAllocation::begin(
            &mut worker,
            &gc,
            slot,
            slot,
            Value {
                number: 711,
                drops: drops.clone(),
            },
        )
        .unwrap();
        roots.push((0..128).find_map(|_| request.poll().unwrap()).unwrap());
    }
    let charged = gc.managed_bytes();
    assert!(matches!(
        LeafAllocation::begin(
            &mut worker,
            &gc,
            2,
            0,
            Value {
                number: 712,
                drops: drops.clone()
            }
        ),
        Err(AllocationError::Catalog(RequestError::RootLimit))
    ));
    assert_eq!(gc.managed_bytes(), charged);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    for root in &roots {
        assert_eq!(root.borrow(&gc).unwrap().number, 711);
    }
    drop(roots);
    drain(&mut worker, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 3);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn typed_preparation_reports_heap_exhaustion_without_collecting_or_taking_a_root() {
    let gc = GarbageCollector::with_config(Configuration::builder().max_heap_bytes(1).build());
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    assert!(matches!(
        LeafAllocation::begin(
            &mut worker,
            &gc,
            0,
            0,
            Value {
                number: 713,
                drops: drops.clone()
            }
        ),
        Err(AllocationError::Heap(crate::Error::OutOfMemory))
    ));
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    assert_eq!(catalog.reserved_root_tokens(), 0);
    assert_eq!(worker.poll_in(&gc), Progress::Idle);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn typed_preparation_rejects_traced_graphs_before_transferring_ownership() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let child = Arc::new(AtomicUsize::new(0));
    let parent = Arc::new(AtomicUsize::new(0));
    assert!(matches!(
        LeafAllocation::begin(&mut worker, &gc, 0, 0, graph(&gc, &child, &parent)),
        Err(AllocationError::Unsupported)
    ));
    assert_eq!(parent.load(Ordering::Relaxed), 1);
    reclaim(&gc).unwrap();
    assert_eq!(child.load(Ordering::Relaxed), 1);
    assert_eq!(catalog.reserved_root_tokens(), 0);
    assert_eq!(worker.poll_in(&gc), Progress::Idle);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn helper_finishes_publication_and_reclaims_others_before_allocation_returns() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut origin) = setup();
    let catalog = Arc::new(catalog);
    let drops = Arc::new(AtomicUsize::new(0));
    let other_drops = Arc::new(AtomicUsize::new(0));
    let record = record(&gc, &drops, 714);
    let weak_catalog = Arc::downgrade(&catalog);
    let weak_gc = Arc::downgrade(&gc);
    let other = other_drops.clone();
    record.pause_after_admission(move || {
        let catalog = weak_catalog.upgrade().unwrap();
        let gc = weak_gc.upgrade().unwrap();
        let mut helper = catalog.register_record_helper().unwrap();
        assert_eq!(catalog.registered(), (1, 1));
        // phase_in performs one helping attempt; that attempt can finish the
        // original operation and still report a transient owning-load miss.
        for _ in 0..16 {
            if helper.phase_in(&gc).is_ok() {
                break;
            }
        }
        let live = helper.discover_root(0).unwrap().unwrap();
        assert_eq!(live.status, RootStatus::Live);
        let other_key = create(&mut helper, &gc, 1, 715, &other);
        run(&mut helper, &gc, GuardedCommand::Publish(other_key));
        next_full_cycle(&mut helper, &gc);
        run(
            &mut helper,
            &gc,
            GuardedCommand::Retire {
                key: other_key,
                domain: 0,
            },
        );
        assert!(helper.release_retired(other_key).unwrap());
        assert_eq!(other.load(Ordering::Relaxed), 1);
        helper
            .submit(GuardedCommand::Retire {
                key: live.target,
                domain: 0,
            })
            .unwrap();
        assert_eq!(
            finish(&mut helper, &gc),
            Progress::Rejected(RequestError::Conflict)
        );
    });
    origin
        .submit_rooted_record(0, 0, record.address(), record)
        .unwrap();
    assert!(matches!(finish(&mut origin, &gc), Progress::RootCreated(_)));
    assert_eq!(other_drops.load(Ordering::Relaxed), 1);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    let root = origin.take_root().unwrap();
    assert_eq!(
        LeafRecord::borrow_root::<Value>(&root, &gc).unwrap().number,
        714
    );
    drop(root);
    drain(&mut origin, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    drop(origin);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn a_stopped_allocation_origin_does_not_hold_up_helper_reclamation_or_lose_its_result() {
    use std::sync::{Mutex, mpsc};
    use std::time::Duration;
    struct Release(mpsc::Sender<()>);
    impl Drop for Release {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut origin) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let other_drops = Arc::new(AtomicUsize::new(0));
    let record = record(&gc, &drops, 716);
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let release = Release(release_tx);
    let release_rx = Mutex::new(release_rx);
    record.pause_after_admission(move || {
        entered_tx.send(()).unwrap();
        release_rx
            .lock()
            .unwrap()
            .recv_timeout(Duration::from_secs(60))
            .unwrap();
    });
    origin
        .submit_rooted_record(0, 0, record.address(), record)
        .unwrap();
    let held_gc = gc.clone();
    let stopped = std::thread::spawn(move || {
        assert!(matches!(
            finish(&mut origin, &held_gc),
            Progress::RootCreated(_)
        ));
        let root = origin.take_root().unwrap();
        assert!(origin.take_root().is_none());
        assert_eq!(
            LeafRecord::borrow_root::<Value>(&root, &held_gc)
                .unwrap()
                .number,
            716
        );
        (origin, root)
    });
    entered_rx.recv_timeout(Duration::from_secs(60)).unwrap();
    let mut helper = catalog.register_record_helper().unwrap();
    for _ in 0..16 {
        if helper.phase_in(&gc).is_ok() {
            break;
        }
    }
    assert_eq!(
        helper.discover_root(0).unwrap().unwrap().status,
        RootStatus::Live
    );
    let other = create(&mut helper, &gc, 1, 717, &other_drops);
    run(&mut helper, &gc, GuardedCommand::Publish(other));
    next_full_cycle(&mut helper, &gc);
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
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(catalog.registered(), (1, 1));
    drop(release);
    let (origin, root) = stopped.join().unwrap();
    drop(origin);
    drop(root);
    drain(&mut helper, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert_eq!(gc.managed_bytes(), 0);
    drop(helper);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn typed_aliases_share_one_token_at_full_root_quota_until_the_last_drop() {
    let gc = collector(true, RootStorage::Contexts);
    let (catalog, budget, mut worker) = setup();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut originals = Vec::new();
    for slot in 0..2 {
        let mut request = LeafAllocation::begin(
            &mut worker,
            &gc,
            slot,
            slot,
            Value {
                number: slot as u64,
                drops: drops.clone(),
            },
        )
        .unwrap();
        originals.push((0..128).find_map(|_| request.poll().unwrap()).unwrap());
    }
    assert_eq!(catalog.reserved_root_tokens(), 2);
    let before = worker.phase_in(&gc).unwrap();
    let aliases = [
        originals[0].clone(),
        originals[0].clone(),
        originals[1].clone(),
    ];
    assert_eq!(
        catalog.reserved_root_tokens(),
        2,
        "aliases must share root quota"
    );
    assert!(std::ptr::eq(
        &raw const *originals[0].borrow(&gc).unwrap(),
        &raw const *aliases[0].borrow(&gc).unwrap()
    ));
    drop(originals);
    for slot in 0..2 {
        assert!(
            !worker
                .discover_root(slot)
                .unwrap()
                .unwrap()
                .removal_requested,
            "an alias dropped the unique token early"
        );
    }
    assert_eq!(
        worker.phase_in(&gc).unwrap(),
        before,
        "cloning ran publication work"
    );
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(aliases[0].borrow(&gc).unwrap().number, 0);
    assert_eq!(aliases[2].borrow(&gc).unwrap().number, 1);
    let [first, last, other] = aliases;
    drop(first);
    assert_eq!(catalog.reserved_root_tokens(), 2);
    assert!(!worker.discover_root(0).unwrap().unwrap().removal_requested);
    drop(last);
    assert_eq!(
        catalog.reserved_root_tokens(),
        1,
        "last alias must release root quota"
    );
    assert!(worker.discover_root(0).unwrap().unwrap().removal_requested);
    assert_eq!(other.borrow(&gc).unwrap().number, 1);
    drop(other);
    drain(&mut worker, &gc);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(gc.managed_bytes(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}
