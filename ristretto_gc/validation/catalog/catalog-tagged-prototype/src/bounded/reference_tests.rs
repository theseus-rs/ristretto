use super::*;
use crate::record_fixture::Fixture;
use crate::{DomainMode, FAIL_LOAD, FAIL_SHARE};
type Worker = Participant<Fixture, GuardedCommand>;
fn setup(q: usize, m: usize, mutators: usize) -> (CatalogHandle<Fixture>, Arc<Budget>) {
    let capacities = Capacities::derive(3, mutators, 1)
        .unwrap()
        .with_roots(m)
        .unwrap()
        .with_references(q)
        .unwrap();
    let footprint = Footprint::for_records::<Fixture>(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
    (catalog, budget)
}
fn finish<R: Payload<Context = usize>>(worker: &mut Participant<R, GuardedCommand>) -> Progress {
    for _ in 0..4096 {
        match worker.poll_in(&7) {
            Progress::Pending => {}
            done => return done,
        }
    }
    panic!("reference request did not settle")
}
fn object(worker: &mut Worker, slot: usize) -> RecordKey {
    worker
        .submit_record(slot, 8 * (slot + 1), Fixture::new())
        .unwrap();
    let Progress::Created(key) = finish(worker) else {
        panic!()
    };
    key
}
fn root<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    slot: usize,
    target: RecordKey,
) -> RootHandle<R> {
    worker.submit_root_creation(slot, target).unwrap();
    assert!(matches!(finish(worker), Progress::RootCreated(_)));
    worker.take_root().unwrap()
}
fn reference<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    slot: usize,
) -> ReferenceHandle<R> {
    worker.submit_reference_creation(slot).unwrap();
    assert!(matches!(finish(worker), Progress::ReferenceCreated(_)));
    worker.take_reference().unwrap()
}
fn run<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    command: GuardedCommand,
) -> Progress {
    worker.submit(command).unwrap();
    finish(worker)
}
fn roots<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    command: RootCommand,
) -> Progress {
    worker.submit_root(command).unwrap();
    finish(worker)
}
fn refs<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    command: ReferenceCommand,
) -> Progress {
    worker.submit_reference(command).unwrap();
    finish(worker)
}
fn drain<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    keys: &[RecordKey],
) {
    for key in keys {
        let Membership::Tracing(marks) = worker.observe(*key).unwrap().membership else {
            continue;
        };
        for domain in 0..2 {
            if marks[domain].ticket != 0 {
                assert_eq!(
                    run(
                        worker,
                        GuardedCommand::Acknowledge {
                            key: *key,
                            domain,
                            ticket: marks[domain].ticket
                        }
                    ),
                    Progress::Committed
                );
            }
        }
    }
}
fn finish_domain<R: Payload<Context = usize>>(
    worker: &mut Participant<R, GuardedCommand>,
    keys: &[RecordKey],
    domain: usize,
) {
    while worker.phase_in(&7).unwrap().domains[domain].floor < crate::RETIREMENT_FLOOR {
        if worker.phase_in(&7).unwrap().domains[domain].floor == 0 {
            while worker.phase_in(&7).unwrap().root_cursors[domain]
                < worker.lease.owner.raw.root_capacity()
            {
                assert_eq!(
                    roots(worker, RootCommand::Scan { domain }),
                    Progress::Committed
                );
            }
        } else {
            while worker.phase_in(&7).unwrap().reference_cursors[domain]
                < worker.lease.owner.capacities.reference_slots
            {
                assert_eq!(
                    refs(worker, ReferenceCommand::Scan { domain }),
                    Progress::Committed
                );
            }
        }
        drain(worker, keys);
        assert_eq!(
            run(worker, GuardedCommand::Advance { domain }),
            Progress::Committed
        );
    }
}
fn published(
    worker: &mut Worker,
    kind: ReferenceKind,
) -> (RecordKey, RootHandle<Fixture>, ReferenceHandle<Fixture>) {
    let key = object(worker, 0);
    let root = root(worker, 0, key);
    assert_eq!(
        roots(worker, RootCommand::Publish(root.key())),
        Progress::Committed
    );
    let reference = reference(worker, 0);
    assert_eq!(
        refs(
            worker,
            ReferenceCommand::Publish {
                reference: reference.key(),
                source: root.key(),
                kind
            }
        ),
        Progress::Committed
    );
    (key, root, reference)
}

#[test]
fn reference_capacity_rederivation_and_byte_charge_are_checked() {
    let base = Capacities::derive(3, 2, 1)
        .unwrap()
        .with_generation_limit(9)
        .unwrap();
    assert_eq!(
        base.with_roots(2).unwrap().with_references(4).unwrap(),
        base.with_references(4).unwrap().with_roots(2).unwrap()
    );
    assert_eq!(
        base.with_references(4).unwrap().with_references(0).unwrap(),
        base
    );
    assert_eq!(
        base.with_roots(2)
            .unwrap()
            .with_references(4)
            .unwrap()
            .with_references(0)
            .unwrap(),
        base.with_roots(2).unwrap()
    );
    assert!(matches!(
        base.with_references(usize::MAX),
        Err(ConfigError::CapacityOverflow)
    ));
    let capacities = base.with_references(2).unwrap();
    assert_eq!(capacities.generation_limit(), 9);
    assert!(
        capacities.root_pool() > 0,
        "M=0 must still reserve internal root backing"
    );
    let footprint = Footprint::for_records::<Fixture>(capacities).unwrap();
    assert_eq!(
        footprint.reference_tokens(),
        2 * std::mem::size_of::<ReferenceHandle<Fixture>>()
    );
    assert_eq!(
        footprint.construction_tokens(),
        2 * std::mem::size_of::<ConstructionHandle<Fixture>>()
    );
    assert_eq!(
        footprint.total(),
        footprint.backing()
            + footprint.participants()
            + footprint.root_tokens()
            + footprint.reference_tokens()
            + footprint.construction_tokens()
    );
    let budget = Budget::new(footprint.total() - 1);
    assert!(matches!(
        BoundedCatalog::<Fixture>::with_records_in(capacities, &budget),
        Err(ConfigError::BudgetExceeded)
    ));
    assert_eq!(budget.used(), 0);
}

#[test]
fn ready_reference_retains_quota_and_charge_until_its_unique_handle_is_destroyed() {
    let (catalog, budget) = setup(1, 0, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    worker.submit_reference_creation(0).unwrap();
    let Progress::ReferenceCreated(key) = finish(&mut worker) else {
        panic!()
    };
    assert_eq!(worker.poll_in(&7), Progress::ReferenceCreated(key));
    assert_eq!(worker.submit_reference_creation(0), Err(RequestError::Busy));
    assert_eq!(worker.observe_reference(key), Err(RequestError::Busy));
    let reference = worker.take_reference().unwrap();
    assert!(worker.take_reference().is_none());
    assert_eq!(catalog.reserved_reference_tokens(), 1);
    assert_eq!(
        worker.submit_reference_creation(0),
        Err(RequestError::ReferenceLimit)
    );
    let charged = catalog.charged_bytes();
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), charged);
    drop(reference);
    assert_eq!(budget.used(), 0);
}

#[test]
fn cancelling_each_reference_creation_prefix_refunds_after_owned_storage() {
    for steps in 0..=4 {
        let (catalog, budget) = setup(1, 0, 1);
        let mut worker = catalog.register_record_mutator().unwrap();
        worker.submit_reference_creation(0).unwrap();
        for _ in 0..steps {
            worker.poll_in(&7);
        }
        assert_eq!(catalog.reserved_reference_tokens(), 1);
        assert!(worker.cancel());
        assert_eq!(catalog.reserved_reference_tokens(), 0);
        if let Some(observed) = worker.discover_reference(0).unwrap() {
            assert!(observed.removal_requested);
            assert_eq!(
                refs(&mut worker, ReferenceCommand::Retire(observed.key)),
                Progress::Committed
            );
            assert!(worker.release_retired_reference(observed.key).unwrap());
        }
        drop(worker);
        drop(catalog);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn reference_identity_and_foreign_sources_cannot_alias_reused_slots() {
    let (catalog, _) = setup(1, 1, 1);
    let (foreign, _) = setup(1, 1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let mut other = foreign.register_record_mutator().unwrap();
    let (_, source, reference) = published(&mut worker, ReferenceKind::Weak);
    assert_eq!(
        other.submit_reference(ReferenceCommand::Clear(reference.key())),
        Err(RequestError::Foreign)
    );
    let key = reference.key();
    drop(reference);
    assert_eq!(
        refs(&mut worker, ReferenceCommand::Retire(key)),
        Progress::Committed
    );
    assert!(worker.release_retired_reference(key).unwrap());
    let replacement = self::reference(&mut worker, 0);
    assert_ne!(replacement.key().identity(), key.identity());
    assert_eq!(worker.observe_reference(key), Err(RequestError::Stale));
    assert_eq!(
        refs(&mut worker, ReferenceCommand::Clear(key)),
        Progress::Rejected(RequestError::Stale)
    );
    let foreign_reference = self::reference(&mut other, 0);
    assert_eq!(
        refs(
            &mut other,
            ReferenceCommand::Publish {
                reference: foreign_reference.key(),
                source: source.key(),
                kind: ReferenceKind::Weak
            }
        ),
        Progress::Rejected(RequestError::Foreign)
    );
}

#[test]
fn construction_handles_have_an_independent_quota_and_helpers_finish_posted_cancellation() {
    let (catalog, budget) = setup(1, 1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let (_, _source, reference) = published(&mut worker, ReferenceKind::Weak);
    worker
        .submit_reference_reservation(reference.key())
        .unwrap();
    let Progress::ConstructionCreated(key) = finish(&mut worker) else {
        panic!()
    };
    assert_eq!(worker.poll_in(&7), Progress::ConstructionCreated(key));
    assert_eq!(
        worker.submit_reference_reservation(reference.key()),
        Err(RequestError::Busy)
    );
    let reservation = worker.take_construction().unwrap();
    assert_eq!(catalog.reserved_construction_tokens(), 1);
    assert_eq!(catalog.reserved_root_tokens(), 1);
    assert_eq!(
        worker.submit_reference_reservation(reference.key()),
        Err(RequestError::ConstructionLimit)
    );
    assert_eq!(
        refs(&mut worker, ReferenceCommand::CancelOwner(reference.key())),
        Progress::Rejected(RequestError::Conflict)
    );
    drop(reservation);
    assert_eq!(catalog.reserved_construction_tokens(), 0);
    let mut helper = catalog.register_record_helper().unwrap();
    assert_eq!(
        refs(&mut helper, ReferenceCommand::CancelOwner(reference.key())),
        Progress::Committed
    );
    assert!(helper.release_retired_root(key).unwrap());
    drop(helper);
    drop(worker);
    drop(reference);
    drop(_source);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn root_and_reference_scan_failures_remain_pending_in_the_embedded_root_range() {
    let (catalog, _) = setup(1, 1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let (target, _root, reference) = published(&mut worker, ReferenceKind::Weak);
    worker
        .submit_reference_reservation(reference.key())
        .unwrap();
    assert!(matches!(
        finish(&mut worker),
        Progress::ConstructionCreated(_)
    ));
    let _reservation = worker.take_construction().unwrap();
    assert_eq!(
        roots(&mut worker, RootCommand::Scan { domain: 0 }),
        Progress::Committed
    );
    assert_eq!(worker.phase_in(&7).unwrap().root_cursors[0], 1);
    worker.submit_root(RootCommand::Scan { domain: 0 }).unwrap();
    // Stable head acquisition succeeds; owning reference lookup fails.
    FAIL_LOAD.with(|n| n.set(Some(1)));
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert_eq!(finish(&mut worker), Progress::Committed);
    assert_eq!(
        roots(&mut worker, RootCommand::Scan { domain: 0 }),
        Progress::Committed
    );
    drain(&mut worker, &[target]);
    assert_eq!(
        run(&mut worker, GuardedCommand::Advance { domain: 0 }),
        Progress::Committed
    );
    worker
        .submit_reference(ReferenceCommand::Scan { domain: 0 })
        .unwrap();
    FAIL_LOAD.with(|n| n.set(Some(1)));
    assert_eq!(worker.poll_in(&7), Progress::Pending);
    assert_eq!(finish(&mut worker), Progress::Committed);
}

#[test]
fn a_stopped_construction_lifetime_does_not_prevent_cycles_or_unrelated_reclamation() {
    let (catalog, _) = setup(1, 1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let (target, source, reference) = published(&mut worker, ReferenceKind::Weak);
    worker
        .submit_reference_reservation(reference.key())
        .unwrap();
    assert!(matches!(
        finish(&mut worker),
        Progress::ConstructionCreated(_)
    ));
    let reservation = worker.take_construction().unwrap();
    let source_key = source.key();
    drop(source);
    assert_eq!(
        roots(&mut worker, RootCommand::RemoveRequested(source_key)),
        Progress::Committed
    );
    assert!(worker.release_retired_root(source_key).unwrap());
    let garbage = object(&mut worker, 1);
    assert_eq!(
        run(&mut worker, GuardedCommand::Publish(garbage)),
        Progress::Committed
    );
    let mut helper = catalog.register_record_helper().unwrap();
    finish_domain(&mut helper, &[target, garbage], 0);
    assert_eq!(
        run(&mut helper, GuardedCommand::Finish { domain: 0 }),
        Progress::Committed
    );
    assert_eq!(
        run(
            &mut helper,
            GuardedCommand::Start {
                domain: 0,
                mode: DomainMode::Full
            }
        ),
        Progress::Committed
    );
    finish_domain(&mut helper, &[target, garbage], 0);
    assert_eq!(
        run(
            &mut helper,
            GuardedCommand::Retire {
                key: garbage,
                domain: 0
            }
        ),
        Progress::Committed
    );
    assert!(helper.release_retired(garbage).unwrap());
    assert_eq!(reservation.observe().status, RootStatus::Live);
    assert_eq!(
        run(
            &mut helper,
            GuardedCommand::Retire {
                key: target,
                domain: 0
            }
        ),
        Progress::Rejected(RequestError::Conflict)
    );
}

struct NotificationFixture {
    catalog: CatalogHandle<Fixture>,
    worker: Worker,
    reference: ReferenceHandle<Fixture>,
    target: RecordKey,
    owner: RecordKey,
    _owner_root: RootHandle<Fixture>,
    destination: RootHandle<Fixture>,
}
impl NotificationFixture {
    fn new() -> Self {
        let (catalog, _) = setup(1, 2, 2);
        let mut worker = catalog.register_record_mutator().unwrap();
        let (target, source, reference) = published(&mut worker, ReferenceKind::Final);
        let owner = object(&mut worker, 1);
        let owner_root = root(&mut worker, 1, owner);
        assert_eq!(
            roots(&mut worker, RootCommand::Publish(owner_root.key())),
            Progress::Committed
        );
        assert_eq!(
            refs(
                &mut worker,
                ReferenceCommand::BindOwner {
                    reference: reference.key(),
                    owner: owner_root.key()
                }
            ),
            Progress::Committed
        );
        let source_key = source.key();
        drop(source);
        assert_eq!(
            roots(&mut worker, RootCommand::RemoveRequested(source_key)),
            Progress::Committed
        );
        assert!(worker.release_retired_root(source_key).unwrap());
        // Publication is physically retained for its birth cycle. Start a fresh
        // cycle so only the managed Final edge can keep the referent alive.
        for domain in 0..2 {
            finish_domain(&mut worker, &[target, owner], domain);
            assert_eq!(
                run(&mut worker, GuardedCommand::Finish { domain }),
                Progress::Committed
            );
        }
        assert_eq!(
            run(
                &mut worker,
                GuardedCommand::Start {
                    domain: 0,
                    mode: DomainMode::Full
                }
            ),
            Progress::Committed
        );
        while worker.phase_in(&7).unwrap().root_cursors[0] < catalog.raw.root_capacity() {
            assert_eq!(
                roots(&mut worker, RootCommand::Scan { domain: 0 }),
                Progress::Committed
            );
        }
        drain(&mut worker, &[target, owner]);
        assert_eq!(
            run(&mut worker, GuardedCommand::Advance { domain: 0 }),
            Progress::Committed
        );
        assert_eq!(
            refs(&mut worker, ReferenceCommand::Scan { domain: 0 }),
            Progress::Committed
        );
        assert_eq!(
            run(&mut worker, GuardedCommand::Advance { domain: 0 }),
            Progress::Committed
        );
        let destination = root(&mut worker, 0, owner);
        assert_eq!(catalog.reserved_root_tokens(), 2);
        Self {
            catalog,
            worker,
            reference,
            target,
            owner,
            _owner_root: owner_root,
            destination,
        }
    }
    fn deliver(&self, helper: &mut Worker) {
        let ReferenceMembership::Live(state) = helper
            .observe_reference(self.reference.key())
            .unwrap()
            .state
        else {
            panic!()
        };
        assert!(state.activated);
        let queued = state.queued_root.expect("durable notification");
        let held = helper
            .discover_reference_root(self.reference.key(), ReferenceRootKind::Notification)
            .unwrap()
            .unwrap();
        assert_eq!(held.key.identity(), queued.identity);
        assert_eq!(held.status, RootStatus::Live);
        assert_eq!(held.target, self.owner);
        assert_eq!(
            refs(
                helper,
                ReferenceCommand::Take {
                    reference: self.reference.key(),
                    root: self.destination.key()
                }
            ),
            Progress::Committed
        );
        assert_eq!(self.destination.observe().status, RootStatus::Live);
        assert_eq!(
            refs(
                helper,
                ReferenceCommand::Take {
                    reference: self.reference.key(),
                    root: self.destination.key()
                }
            ),
            Progress::Rejected(RequestError::Conflict)
        );
        assert!(helper.release_retired_root(held.key).unwrap());
        assert!(
            helper
                .discover_reference_root(self.reference.key(), ReferenceRootKind::Notification)
                .unwrap()
                .is_none()
        );
    }
}

#[test]
fn reserved_helper_delivers_with_full_caller_root_quota_at_every_stopped_scan_prefix() {
    let mut saw_deferred = false;
    let mut saw_creation = false;
    let mut saw_token = false;
    let mut saw_preparation = false;
    let mut saw_attempt = false;
    for prefix in 0..16 {
        let mut fixture = NotificationFixture::new();
        let mut second = fixture.catalog.register_record_mutator().unwrap();
        for worker in [&mut fixture.worker, &mut second] {
            worker
                .submit_reference(ReferenceCommand::Scan { domain: 0 })
                .unwrap();
        }
        for _ in 0..prefix {
            for worker in [&mut fixture.worker, &mut second] {
                if matches!(worker.state, RequestState::Idle) {
                    continue;
                }
                let outcome = worker.poll_in(&7);
                assert!(matches!(
                    outcome,
                    Progress::Pending
                        | Progress::Committed
                        | Progress::Rejected(RequestError::Conflict)
                ));
            }
        }
        for worker in [&fixture.worker, &second] {
            if let RequestState::Reference(Request::Notification(n)) = &worker.state {
                saw_deferred = true;
                saw_creation |= n.creation.is_some();
                saw_token |= n.token.is_some();
                saw_preparation |= matches!(n.work, Work::Preparing(_));
                saw_attempt |= matches!(n.work, Work::Attempt(_));
            }
        }
        // Both mutators stay stopped. Only the independently reserved helper runs.
        let mut helper = fixture.catalog.register_record_helper().unwrap();
        finish_domain(&mut helper, &[fixture.target, fixture.owner], 0);
        fixture.deliver(&mut helper);
        for worker in [&mut fixture.worker, &mut second] {
            if !matches!(worker.state, RequestState::Idle) {
                assert!(matches!(
                    finish(worker),
                    Progress::Committed | Progress::Rejected(RequestError::Conflict)
                ));
            }
        }
        assert_eq!(fixture.destination.observe().status, RootStatus::Live);
        assert_eq!(fixture.catalog.reserved_root_tokens(), 2);
        assert_eq!(fixture.catalog.reserved_construction_tokens(), 0);
    }
    assert!(saw_deferred && saw_creation && saw_token && saw_preparation && saw_attempt);
}

#[test]
fn notification_acquisition_failures_preserve_the_request_and_reserved_capacity() {
    for sharing in [false, true] {
        for offset in 0..32 {
            let fixture = NotificationFixture::new();
            let mut helper = fixture.catalog.register_record_helper().unwrap();
            helper
                .submit_reference(ReferenceCommand::Scan { domain: 0 })
                .unwrap();
            if sharing {
                FAIL_SHARE.with(|n| n.set(Some(offset)));
            } else {
                FAIL_LOAD.with(|n| n.set(Some(offset)));
            }
            let result = finish(&mut helper);
            FAIL_SHARE.with(|n| n.set(None));
            FAIL_LOAD.with(|n| n.set(None));
            assert_eq!(
                result,
                Progress::Committed,
                "sharing={sharing}, offset={offset}"
            );
            fixture.deliver(&mut helper);
            assert_eq!(fixture.catalog.reserved_root_tokens(), 2);
        }
    }
}

#[test]
fn cancelling_or_dropping_notification_requests_keeps_helper_delivery_available() {
    for drop_participant in [false, true] {
        for prefix in 0..16 {
            let fixture = NotificationFixture::new();
            let mut second = fixture.catalog.register_record_mutator().unwrap();
            second
                .submit_reference(ReferenceCommand::Scan { domain: 0 })
                .unwrap();
            for _ in 0..prefix {
                if matches!(second.state, RequestState::Idle) {
                    break;
                }
                assert!(matches!(
                    second.poll_in(&7),
                    Progress::Pending | Progress::Committed
                ));
            }
            if !drop_participant && !second.cancel() {
                assert_eq!(finish(&mut second), Progress::Committed);
            }
            drop(second);
            let mut helper = fixture.catalog.register_record_helper().unwrap();
            finish_domain(&mut helper, &[fixture.target, fixture.owner], 0);
            fixture.deliver(&mut helper);
        }
    }
}

#[test]
fn cancelling_or_dropping_each_construction_prefix_leaves_only_helpable_cleanup() {
    for drop_participant in [false, true] {
        for prefix in 0..14 {
            let (catalog, _) = setup(1, 1, 2);
            let mut origin = catalog.register_record_mutator().unwrap();
            let (_, _source, reference) = published(&mut origin, ReferenceKind::Weak);
            let mut second = catalog.register_record_mutator().unwrap();
            second
                .submit_reference_reservation(reference.key())
                .unwrap();
            for _ in 0..prefix {
                let result = second.poll_in(&7);
                assert!(matches!(
                    result,
                    Progress::Pending | Progress::ConstructionCreated(_)
                ));
            }
            if !drop_participant && !second.cancel() {
                assert!(matches!(
                    finish(&mut second),
                    Progress::ConstructionCreated(_)
                ));
            }
            drop(second);
            assert_eq!(catalog.reserved_construction_tokens(), 0);
            let mut helper = catalog.register_record_helper().unwrap();
            if let Some(root) = helper
                .discover_reference_root(reference.key(), ReferenceRootKind::Construction)
                .unwrap()
            {
                assert!(root.removal_requested);
                let command = match root.status {
                    RootStatus::Prepared => {
                        assert_eq!(
                            roots(&mut helper, RootCommand::RemoveRequested(root.key)),
                            Progress::Committed
                        );
                        None
                    }
                    RootStatus::Live => Some(ReferenceCommand::CancelOwner(reference.key())),
                    RootStatus::Retired => None,
                };
                if let Some(command) = command {
                    assert_eq!(refs(&mut helper, command), Progress::Committed);
                }
                assert!(helper.release_retired_root(root.key).unwrap());
            }
            // Quota and embedded location are both reusable without the origin.
            helper
                .submit_reference_reservation(reference.key())
                .unwrap();
            assert!(matches!(
                finish(&mut helper),
                Progress::ConstructionCreated(_)
            ));
            let ready = helper.take_construction().unwrap();
            assert_eq!(ready.observe().status, RootStatus::Live);
        }
    }
}

#[test]
fn reserved_prepared_root_cancellation_retries_failed_acquisition_without_false_conflict() {
    for requested in [false, true] {
        let (catalog, _) = setup(1, 1, 1);
        let mut worker = catalog.register_record_mutator().unwrap();
        let (_, _source, reference) = published(&mut worker, ReferenceKind::Weak);
        worker
            .submit_reference_reservation(reference.key())
            .unwrap();
        while !matches!(
            worker.state,
            RequestState::Reference(Request::Reserving { .. })
        ) {
            assert_eq!(worker.poll_in(&7), Progress::Pending);
        }
        let mut helper = catalog.register_record_helper().unwrap();
        let root = helper
            .discover_reference_root(reference.key(), ReferenceRootKind::Construction)
            .unwrap()
            .unwrap();
        assert_eq!(root.status, RootStatus::Prepared);
        if requested {
            assert!(worker.cancel());
        }
        helper
            .submit_root(if requested {
                RootCommand::RemoveRequested(root.key)
            } else {
                RootCommand::Cancel(root.key)
            })
            .unwrap();
        // The stable snapshot and lookup succeed; fail the first proposed owner.
        FAIL_SHARE.with(|n| n.set(Some(0)));
        assert_eq!(helper.poll_in(&7), Progress::Pending);
        assert_eq!(finish(&mut helper), Progress::Committed);
        assert!(helper.release_retired_root(root.key).unwrap());
        if !requested {
            assert_eq!(
                finish(&mut worker),
                Progress::Rejected(RequestError::Conflict)
            );
        }
        assert_eq!(catalog.reserved_construction_tokens(), 0);
    }
}

#[test]
fn posting_reference_drop_intent_keeps_its_quota_through_token_teardown() {
    let (catalog, budget) = setup(1, 0, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let mut handle = reference(&mut worker, 0);
    let retained = catalog.clone();
    handle.token.after_drop_request(move || {
        assert_eq!(retained.reserved_reference_tokens(), 1);
        let mut helper = retained.register_record_helper().unwrap();
        assert_eq!(
            helper.submit_reference_creation(0),
            Err(RequestError::ReferenceLimit)
        );
    });
    drop(worker);
    drop(catalog);
    assert!(budget.used() > 0);
    drop(handle);
    assert_eq!(budget.used(), 0);
}

#[test]
fn posting_construction_drop_intent_keeps_its_quota_through_token_teardown() {
    let (catalog, _) = setup(1, 1, 1);
    let mut worker = catalog.register_record_mutator().unwrap();
    let (_, _source, reference) = published(&mut worker, ReferenceKind::Weak);
    worker
        .submit_reference_reservation(reference.key())
        .unwrap();
    assert!(matches!(
        finish(&mut worker),
        Progress::ConstructionCreated(_)
    ));
    let mut handle = worker.take_construction().unwrap();
    let retained = catalog.clone();
    let key = reference.key();
    handle.token.after_drop_request(move || {
        assert_eq!(retained.reserved_construction_tokens(), 1);
        let mut helper = retained.register_record_helper().unwrap();
        assert_eq!(
            helper.submit_reference_reservation(key),
            Err(RequestError::ConstructionLimit)
        );
    });
    drop(handle);
    assert_eq!(catalog.reserved_construction_tokens(), 0);
}

#[test]
fn installed_notification_cancellation_preserves_an_irreversible_commit() {
    for committed in [false, true] {
        for dropping in [false, true] {
            let fixture = NotificationFixture::new();
            let mut second = fixture.catalog.register_record_mutator().unwrap();
            second
                .submit_reference(ReferenceCommand::Scan { domain: 0 })
                .unwrap();
            for _ in 0..32 {
                if matches!(
                    second.state,
                    RequestState::Reference(Request::Notification(Notification {
                        work: Work::Attempt(_),
                        ..
                    }))
                ) {
                    break;
                }
                assert_eq!(second.poll_in(&7), Progress::Pending);
            }
            let RequestState::Reference(Request::Notification(Notification {
                work: Work::Attempt(attempt),
                ..
            })) = &second.state
            else {
                panic!("no prepared notification attempt")
            };
            if committed {
                fixture.catalog.raw.effect_and_stop(attempt);
            } else {
                assert!(fixture.catalog.raw.install_and_stop(attempt));
            }
            if !dropping {
                assert_eq!(second.cancel(), !committed);
            }
            let mut second = if dropping {
                drop(second);
                None
            } else {
                Some(second)
            };
            let mut helper = fixture.catalog.register_record_helper().unwrap();
            // This first snapshot helps the installed controller but never returns
            // an uncertified phase. The next observes its immutable decision.
            assert_eq!(helper.phase_in(&7), Err(RequestError::Contended));
            assert_eq!(
                helper.phase_in(&7).unwrap().reference_cursors[0],
                usize::from(committed)
            );
            if let Some(worker) = &mut second {
                if committed {
                    assert_eq!(finish(worker), Progress::Committed);
                } else {
                    assert!(matches!(worker.state, RequestState::Idle));
                }
            }
            finish_domain(&mut helper, &[fixture.target, fixture.owner], 0);
            fixture.deliver(&mut helper);
        }
    }
}

#[test]
fn invalid_reference_commands_reject_and_generation_exhaustion_refunds_creation_quota() {
    let capacities = Capacities::derive(1, 1, 1)
        .unwrap()
        .with_references(1)
        .unwrap()
        .with_generation_limit(1)
        .unwrap();
    let budget = Budget::new(
        Footprint::for_records::<Fixture>(capacities)
            .unwrap()
            .total(),
    );
    let catalog = BoundedCatalog::<Fixture>::with_records_in(capacities, &budget).unwrap();
    let mut worker = catalog.register_record_mutator().unwrap();
    assert_eq!(
        worker.submit_reference_creation(1),
        Err(RequestError::InvalidSlot)
    );
    assert_eq!(
        refs(&mut worker, ReferenceCommand::Scan { domain: 2 }),
        Progress::Rejected(RequestError::InvalidDomain)
    );
    assert_eq!(
        refs(&mut worker, ReferenceCommand::Scan { domain: 0 }),
        Progress::Rejected(RequestError::Conflict)
    );
    let mut exhausted = false;
    for _ in 0..1024 {
        worker.submit_reference_creation(0).unwrap();
        match finish(&mut worker) {
            Progress::ReferenceCreated(_) => {
                let reference = worker.take_reference().unwrap();
                assert_eq!(
                    refs(&mut worker, ReferenceCommand::Clear(reference.key())),
                    Progress::Rejected(RequestError::Conflict)
                );
                let key = reference.key();
                drop(reference);
                match refs(&mut worker, ReferenceCommand::Retire(key)) {
                    Progress::Committed => {
                        assert!(worker.release_retired_reference(key).unwrap());
                    }
                    Progress::Rejected(RequestError::GenerationExhausted) => {
                        exhausted = true;
                        break;
                    }
                    other => panic!("unexpected retirement: {other:?}"),
                }
            }
            Progress::Rejected(RequestError::GenerationExhausted) => {
                exhausted = true;
                break;
            }
            other => panic!("unexpected creation: {other:?}"),
        }
    }
    assert!(
        exhausted,
        "finite generations must reject without wrapping or spinning"
    );
    assert_eq!(catalog.reserved_reference_tokens(), 0);
    drop(worker);
    drop(catalog);
    assert_eq!(budget.used(), 0);
}

#[test]
fn shared_owner_publication_and_reference_upgrade_transfer_exact_roots() {
    for prebind in [false, true] {
        let (catalog, _) = setup(2, 2, 1);
        let mut worker = catalog.register_record_mutator().unwrap();
        let (target, source, first) = published(&mut worker, ReferenceKind::Weak);
        let second = reference(&mut worker, 1);
        assert_eq!(
            refs(
                &mut worker,
                ReferenceCommand::Clone {
                    source: first.key(),
                    destination: second.key()
                }
            ),
            Progress::Committed
        );
        let mut constructions = Vec::new();
        for reference in [&first, &second] {
            worker
                .submit_reference_reservation(reference.key())
                .unwrap();
            assert!(matches!(
                finish(&mut worker),
                Progress::ConstructionCreated(_)
            ));
            constructions.push(worker.take_construction().unwrap());
        }
        let owner = object(&mut worker, 1);
        let owner_root = root(&mut worker, 1, owner);
        if prebind {
            for reference in [&first, &second] {
                assert_eq!(
                    refs(
                        &mut worker,
                        ReferenceCommand::PrepareOwner {
                            reference: reference.key(),
                            owner
                        }
                    ),
                    Progress::Committed
                );
            }
            assert_eq!(
                roots(&mut worker, RootCommand::Publish(owner_root.key())),
                Progress::Committed
            );
        } else {
            assert_eq!(
                refs(
                    &mut worker,
                    ReferenceCommand::PrepareOwner {
                        reference: second.key(),
                        owner
                    }
                ),
                Progress::Committed
            );
            assert_eq!(
                refs(
                    &mut worker,
                    ReferenceCommand::PublishOwner {
                        reference: first.key(),
                        owner: owner_root.key()
                    }
                ),
                Progress::Committed
            );
        }
        let mut helper = catalog.register_record_helper().unwrap();
        for (i, reference) in [&first, &second].into_iter().enumerate() {
            if prebind || i == 1 {
                assert_eq!(
                    refs(&mut helper, ReferenceCommand::FinishOwner(reference.key())),
                    Progress::Committed
                );
            }
            let ReferenceMembership::Live(state) =
                helper.observe_reference(reference.key()).unwrap().state
            else {
                panic!()
            };
            assert_eq!(state.owner.unwrap().identity, owner.identity());
            assert!(state.construction_root.is_none());
            assert_eq!(constructions[i].observe().status, RootStatus::Retired);
            assert!(helper.release_retired_root(constructions[i].key()).unwrap());
        }
        // Exported lifetime handles keep their own quota after publication;
        // notification capacity remains independent of those handles.
        assert_eq!(catalog.reserved_construction_tokens(), 2);
        assert_eq!(
            helper.submit_reference_reservation(first.key()),
            Err(RequestError::ConstructionLimit)
        );
        let old_source = source.key();
        drop(source);
        assert_eq!(
            roots(&mut helper, RootCommand::RemoveRequested(old_source)),
            Progress::Committed
        );
        assert!(helper.release_retired_root(old_source).unwrap());
        let upgraded = root(&mut helper, 0, target);
        assert_eq!(
            refs(
                &mut helper,
                ReferenceCommand::Upgrade {
                    reference: first.key(),
                    root: upgraded.key()
                }
            ),
            Progress::Committed
        );
        assert_eq!(upgraded.observe().status, RootStatus::Live);
        assert_eq!(
            refs(&mut helper, ReferenceCommand::Clear(second.key())),
            Progress::Committed
        );
        let ReferenceMembership::Live(state) =
            helper.observe_reference(second.key()).unwrap().state
        else {
            panic!()
        };
        assert!(state.target.is_none());
        drop(constructions);
        assert_eq!(catalog.reserved_construction_tokens(), 0);
    }
}

#[cfg(not(target_family = "wasm"))]
mod destructors {
    use super::*;
    use crate::record_fixture::Guard;
    use crate::{Entered, RecordError};
    use std::sync::Barrier;

    struct Value {
        fixture: Fixture,
        on_drop: Option<Box<dyn FnOnce() + Send + Sync>>,
    }
    impl Value {
        fn new() -> Self {
            Self {
                fixture: Fixture::new(),
                on_drop: None,
            }
        }
    }
    impl Payload for Value {
        type Context = usize;
        type Guard<'a> = Guard<'a>;
        fn enter<'a>(&'a self, context: &'a usize) -> Result<Entered<'a, Guard<'a>>, RecordError> {
            self.fixture.enter(context)
        }
    }
    impl Drop for Value {
        fn drop(&mut self) {
            if let Some(hook) = self.on_drop.take() {
                hook();
            }
        }
    }
    fn create(
        worker: &mut Participant<Value, GuardedCommand>,
        slot: usize,
        value: Value,
    ) -> RecordKey {
        worker.submit_record(slot, 8 * (slot + 1), value).unwrap();
        let Progress::Created(key) = finish(worker) else {
            panic!()
        };
        key
    }

    #[test]
    fn stalled_reference_destructor_keeps_quota_and_charge_through_late_embedded_root_destruction()
    {
        let capacities = Capacities::derive(3, 1, 1)
            .unwrap()
            .with_roots(1)
            .unwrap()
            .with_references(1)
            .unwrap();
        let budget = Budget::new(Footprint::for_records::<Value>(capacities).unwrap().total());
        let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
        let mut worker = catalog.register_record_mutator().unwrap();
        let requested = Arc::new(Barrier::new(2));
        let release_token = Arc::new(Barrier::new(2));
        let destroying = Arc::new(Barrier::new(2));
        let finish_destroy = Arc::new(Barrier::new(2));
        let mut value = Value::new();
        let reached = destroying.clone();
        let resume = finish_destroy.clone();
        value.on_drop = Some(Box::new(move || {
            reached.wait();
            resume.wait();
        }));
        let target = create(&mut worker, 0, value);
        let other = create(&mut worker, 1, Value::new());
        let source = root(&mut worker, 0, target);
        assert_eq!(
            roots(&mut worker, RootCommand::Publish(source.key())),
            Progress::Committed
        );
        let mut reference = reference(&mut worker, 0);
        let reference_key = reference.key();
        assert_eq!(
            refs(
                &mut worker,
                ReferenceCommand::Publish {
                    reference: reference_key,
                    source: source.key(),
                    kind: ReferenceKind::Weak
                }
            ),
            Progress::Committed
        );
        worker.submit_reference_reservation(reference_key).unwrap();
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert_eq!(worker.poll_in(&7), Progress::Pending);
        assert!(matches!(
            worker.state,
            RequestState::Reference(Request::CreatingReservation { .. })
        ));
        let reached = requested.clone();
        let resume = release_token.clone();
        reference.token.after_drop_request(move || {
            reached.wait();
            resume.wait();
        });
        let thread = std::thread::spawn(move || drop(reference));
        requested.wait();
        let mut helper = catalog.register_record_helper().unwrap();
        assert_eq!(
            refs(&mut helper, ReferenceCommand::Retire(reference_key)),
            Progress::Committed
        );
        assert!(helper.release_retired_reference(reference_key).unwrap());
        let source_key = source.key();
        drop(source);
        assert_eq!(
            roots(&mut helper, RootCommand::RemoveRequested(source_key)),
            Progress::Committed
        );
        assert!(helper.release_retired_root(source_key).unwrap());
        for domain in 0..2 {
            finish_domain(&mut helper, &[target], domain);
            assert_eq!(
                run(&mut helper, GuardedCommand::Finish { domain }),
                Progress::Committed
            );
        }
        assert_eq!(
            run(
                &mut helper,
                GuardedCommand::Start {
                    domain: 0,
                    mode: DomainMode::Full
                }
            ),
            Progress::Committed
        );
        finish_domain(&mut helper, &[target], 0);
        assert_eq!(
            run(
                &mut helper,
                GuardedCommand::Retire {
                    key: target,
                    domain: 0
                }
            ),
            Progress::Committed
        );
        assert!(helper.release_retired(target).unwrap());
        // This late announcement writes the old reference object, whose token is
        // still being destroyed. The stale request relinquishes all its owners.
        assert_eq!(finish(&mut worker), Progress::Rejected(RequestError::Stale));
        assert_eq!(catalog.reserved_construction_tokens(), 0);
        drop(worker);
        release_token.wait();
        destroying.wait();
        assert_eq!(catalog.reserved_reference_tokens(), 1);
        assert_eq!(
            helper.submit_reference_creation(0),
            Err(RequestError::ReferenceLimit)
        );
        assert_eq!(
            run(&mut helper, GuardedCommand::Abandon(other)),
            Progress::Committed
        );
        assert!(helper.release_retired(other).unwrap());
        let charged = catalog.charged_bytes();
        drop(helper);
        drop(catalog);
        assert_eq!(budget.used(), charged);
        finish_destroy.wait();
        thread.join().unwrap();
        assert_eq!(budget.used(), 0);
    }

    #[test]
    fn stalled_construction_destructor_keeps_its_own_quota_after_helper_cancellation() {
        let capacities = Capacities::derive(3, 1, 1)
            .unwrap()
            .with_roots(1)
            .unwrap()
            .with_references(1)
            .unwrap();
        let budget = Budget::new(Footprint::for_records::<Value>(capacities).unwrap().total());
        let catalog = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
        let mut worker = catalog.register_record_mutator().unwrap();
        let requested = Arc::new(Barrier::new(2));
        let release_token = Arc::new(Barrier::new(2));
        let destroying = Arc::new(Barrier::new(2));
        let finish_destroy = Arc::new(Barrier::new(2));
        let mut value = Value::new();
        let reached = destroying.clone();
        let resume = finish_destroy.clone();
        value.on_drop = Some(Box::new(move || {
            reached.wait();
            resume.wait();
        }));
        let target = create(&mut worker, 0, value);
        let other = create(&mut worker, 1, Value::new());
        let source = root(&mut worker, 0, target);
        assert_eq!(
            roots(&mut worker, RootCommand::Publish(source.key())),
            Progress::Committed
        );
        let reference = reference(&mut worker, 0);
        assert_eq!(
            refs(
                &mut worker,
                ReferenceCommand::Publish {
                    reference: reference.key(),
                    source: source.key(),
                    kind: ReferenceKind::Weak
                }
            ),
            Progress::Committed
        );
        worker
            .submit_reference_reservation(reference.key())
            .unwrap();
        assert!(matches!(
            finish(&mut worker),
            Progress::ConstructionCreated(_)
        ));
        let mut construction = worker.take_construction().unwrap();
        let held_key = construction.key();
        let reached = requested.clone();
        let resume = release_token.clone();
        construction.token.after_drop_request(move || {
            reached.wait();
            resume.wait();
        });
        let thread = std::thread::spawn(move || drop(construction));
        requested.wait();
        let mut helper = catalog.register_record_helper().unwrap();
        assert_eq!(
            refs(&mut helper, ReferenceCommand::CancelOwner(reference.key())),
            Progress::Committed
        );
        assert!(helper.release_retired_root(held_key).unwrap());
        let source_key = source.key();
        drop(source);
        assert_eq!(
            roots(&mut helper, RootCommand::RemoveRequested(source_key)),
            Progress::Committed
        );
        assert!(helper.release_retired_root(source_key).unwrap());
        let reference_key = reference.key();
        drop(reference);
        assert_eq!(
            refs(&mut helper, ReferenceCommand::Retire(reference_key)),
            Progress::Committed
        );
        assert!(helper.release_retired_reference(reference_key).unwrap());
        for domain in 0..2 {
            finish_domain(&mut helper, &[target], domain);
            assert_eq!(
                run(&mut helper, GuardedCommand::Finish { domain }),
                Progress::Committed
            );
        }
        assert_eq!(
            run(
                &mut helper,
                GuardedCommand::Start {
                    domain: 0,
                    mode: DomainMode::Full
                }
            ),
            Progress::Committed
        );
        finish_domain(&mut helper, &[target], 0);
        assert_eq!(
            run(
                &mut helper,
                GuardedCommand::Retire {
                    key: target,
                    domain: 0
                }
            ),
            Progress::Committed
        );
        assert!(helper.release_retired(target).unwrap());
        drop(worker);
        release_token.wait();
        destroying.wait();
        assert_eq!(catalog.reserved_construction_tokens(), 1);
        assert_eq!(
            helper.submit_reference_reservation(reference_key),
            Err(RequestError::ConstructionLimit)
        );
        assert_eq!(
            run(&mut helper, GuardedCommand::Abandon(other)),
            Progress::Committed
        );
        assert!(helper.release_retired(other).unwrap());
        let charged = catalog.charged_bytes();
        drop(helper);
        drop(catalog);
        assert_eq!(budget.used(), charged);
        finish_destroy.wait();
        thread.join().unwrap();
        assert_eq!(budget.used(), 0);
    }
}
