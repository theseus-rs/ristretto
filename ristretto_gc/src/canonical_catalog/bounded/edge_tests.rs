use super::super::{
    Arc, AtomicUsize, Budget, Capacities, ConfigError, Footprint, GuardedCommand, RootCommand,
};
use super::*;
use crate::canonical_catalog::{DomainMode, Entered, RETIREMENT_FLOOR, RecordError};

struct Value {
    drops: Arc<AtomicUsize>,
    identity: usize,
}
impl Payload for Value {
    type Context = ();
    type Guard<'a> = ();

    fn enter<'a>(&'a self, context: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        ().enter(context)
    }
}
impl Drop for Value {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}
type Worker = Participant<Value, GuardedCommand>;

fn setup(edges: usize) -> (CatalogHandle<Value>, Arc<Budget>, Arc<AtomicUsize>) {
    let capacities = Capacities::derive(3, 2, 1)
        .unwrap()
        .with_roots(4)
        .unwrap()
        .with_edges(edges)
        .unwrap();
    let footprint = Footprint::for_records::<Value>(capacities).unwrap();
    let budget = Budget::new(footprint.total());
    let owner = BoundedCatalog::with_records_in(capacities, &budget).unwrap();
    (owner, budget, Arc::new(AtomicUsize::new(0)))
}

fn finish(worker: &mut Worker) -> Progress {
    for _ in 0..4096 {
        let result = worker.poll_in(&());
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("bounded edge request did not settle");
}

fn run(worker: &mut Worker, command: GuardedCommand) {
    worker.submit(command).unwrap();
    assert_eq!(finish(worker), Progress::Committed);
}

fn root_run(worker: &mut Worker, command: RootCommand) {
    worker.submit_root(command).unwrap();
    assert_eq!(finish(worker), Progress::Committed);
}

fn edge_run(worker: &mut Worker, command: EdgeCommand) {
    worker.submit_edge(command).unwrap();
    assert_eq!(finish(worker), Progress::Committed);
}

fn record(worker: &mut Worker, slot: usize, drops: &Arc<AtomicUsize>) -> RootHandle<Value> {
    assert!(
        worker
            .submit_rooted_record(
                slot,
                slot,
                slot + 8,
                Value {
                    drops: drops.clone(),
                    identity: slot,
                }
            )
            .is_ok()
    );
    assert!(matches!(finish(worker), Progress::RootCreated(_)));
    worker.take_root().unwrap()
}

fn edge(worker: &mut Worker, slot: usize) -> EdgeHandle<Value> {
    worker.submit_edge_creation(slot).unwrap();
    assert!(matches!(finish(worker), Progress::EdgeCreated(_)));
    worker.take_edge().unwrap()
}

fn publish(
    worker: &mut Worker,
    slot: usize,
    source: &RootHandle<Value>,
    target: &RootHandle<Value>,
) -> EdgeKey {
    let handle = edge(worker, slot);
    let key = handle.key();
    edge_run(
        worker,
        EdgeCommand::Publish {
            edge: key,
            source: source.key(),
            target: Some(target.key()),
        },
    );
    match worker.release_edge_to_source(handle, &()) {
        Ok(released) => {
            assert_eq!(released, key);
            key
        }
        Err(failure) => panic!("certified edge handoff failed: {:?}", failure.error),
    }
}

fn acknowledge(worker: &mut Worker, key: RecordKey, domain: usize) {
    let Membership::Tracing(marks) = worker.observe(key).unwrap().membership else {
        panic!()
    };
    if marks[domain].ticket != 0 {
        run(
            worker,
            GuardedCommand::Acknowledge {
                key,
                domain,
                ticket: marks[domain].ticket,
            },
        );
    }
}

fn scan(worker: &mut Worker, domain: usize) {
    while worker.phase_in(&()).unwrap().root_cursors[domain] < 4 {
        root_run(worker, RootCommand::ScanBatch { domain });
    }
}

fn end(worker: &mut Worker, domain: usize) {
    scan(worker, domain);
    for _ in 0..RETIREMENT_FLOOR {
        run(worker, GuardedCommand::Advance { domain });
    }
    run(worker, GuardedCommand::Finish { domain });
}

fn drop_root(worker: &mut Worker, root: RootHandle<Value>) {
    let key = root.key();
    drop(root);
    root_run(worker, RootCommand::RemoveRequested(key));
    assert!(worker.release_retired_root(key).unwrap());
}

#[test]
fn bounded_source_edge_observation_rejects_failed_or_foreign_authority() {
    let (owner, _, drops) = setup(2);
    let (other, _, other_drops) = setup(2);
    let mut worker = owner.register_record_mutator().unwrap();
    let mut foreign = other.register_record_mutator().unwrap();
    let a = record(&mut worker, 0, &drops);
    let b = record(&mut worker, 1, &drops);
    let x = record(&mut foreign, 0, &other_drops);
    let key = publish(&mut worker, 0, &a, &b);
    let observed = worker.observe_source_edge(a.key(), 0, &()).unwrap();
    assert_eq!(worker.observe_rooted_edge(&a, 0, &()), Ok(observed));
    assert_eq!(
        worker.observe_rooted_edge(&b, 0, &()),
        Err(RequestError::Conflict)
    );
    assert_eq!(
        worker.observe_rooted_edge(&x, 0, &()),
        Err(RequestError::Foreign)
    );
    assert_eq!(observed.key, key);
    assert!(matches!(observed.state, EdgeMembership::Live(state) if state.target.is_some()));
    assert_eq!(
        worker.observe_source_edge(b.key(), 0, &()),
        Err(RequestError::Conflict)
    );
    assert_eq!(
        worker.observe_source_edge(x.key(), 0, &()),
        Err(RequestError::Foreign)
    );
    assert_eq!(
        worker.observe_source_edge(a.key(), 2, &()),
        Err(RequestError::InvalidSlot)
    );
    for failure in 0..5 {
        crate::canonical_catalog::FAIL_LOAD.with(|count| count.set(Some(failure)));
        let result = worker.observe_source_edge(a.key(), 0, &());
        crate::canonical_catalog::FAIL_LOAD.with(|count| count.set(None));
        assert_eq!(
            result,
            Err(RequestError::Contended),
            "failed observation {failure} granted graph authority"
        );
    }
    // The closing comparison uses the already retained controller.
    crate::canonical_catalog::FAIL_LOAD.with(|count| count.set(Some(5)));
    assert_eq!(worker.observe_source_edge(a.key(), 0, &()), Ok(observed));
    assert_eq!(
        crate::canonical_catalog::FAIL_LOAD.with(std::cell::Cell::take),
        Some(0)
    );
    // Borrowing the live handle removes one more fallible registry load.
    for failure in 0..4 {
        crate::canonical_catalog::FAIL_LOAD.with(|count| count.set(Some(failure)));
        assert_eq!(
            worker.observe_rooted_edge(&a, 0, &()),
            Err(RequestError::Contended)
        );
    }
    crate::canonical_catalog::FAIL_LOAD.with(|count| count.set(Some(4)));
    assert_eq!(worker.observe_rooted_edge(&a, 0, &()), Ok(observed));
    assert_eq!(
        crate::canonical_catalog::FAIL_LOAD.with(std::cell::Cell::take),
        Some(0)
    );
    root_run(&mut worker, RootCommand::Remove(a.key()));
    assert_eq!(
        worker.observe_rooted_edge(&a, 0, &()),
        Err(RequestError::Conflict)
    );
    assert_eq!(
        worker.observe_source_edge(a.key(), 0, &()),
        Err(RequestError::Conflict)
    );
}

#[test]
fn bounded_discard_preserves_won_edge_commit_and_releases_participant_request() {
    let (owner, budget, drops) = setup(2);
    let mut worker = owner.register_record_mutator().unwrap();
    let mut helper = owner.register_record_helper().unwrap();
    let a = record(&mut worker, 0, &drops);
    let b = record(&mut worker, 1, &drops);
    let key = publish(&mut worker, 0, &a, &b);
    worker
        .submit_edge(EdgeCommand::Replace {
            edge: key,
            source: a.key(),
            target: None,
        })
        .unwrap();
    let mut installed = false;
    for _ in 0..16 {
        if let RequestState::Edge(Request::Attempt(_, attempt)) = &worker.state {
            owner.raw.effect_and_stop(attempt);
            installed = true;
            break;
        }
        assert_eq!(worker.poll_in(&()), Progress::Pending);
    }
    assert!(installed);
    assert!(!worker.cancel(), "cancellation revoked a won edge commit");
    worker.discard_request();
    assert_eq!(
        worker.poll_in(&()),
        Progress::Idle,
        "discard retained a committed request"
    );
    let observed = (0..16)
        .find_map(|_| match helper.observe_source_edge(a.key(), 0, &()) {
            Ok(observed) => Some(observed),
            Err(RequestError::Contended) => None,
            error => panic!("unexpected source observation: {error:?}"),
        })
        .expect("committed edge did not settle");
    assert!(
        matches!(observed.state, EdgeMembership::Live(state) if state.target.is_none()),
        "stable read escaped an unfinished committed edge"
    );
    edge_run(
        &mut worker,
        EdgeCommand::Replace {
            edge: key,
            source: a.key(),
            target: Some(b.key()),
        },
    );
    drop(a);
    drop(b);
    drop(worker);
    drop(helper);
    drop(owner);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(budget.used(), 0);
}

#[test]
fn bounded_source_edge_observation_retries_replacement_and_revocation_windows() {
    for (revoke, borrowed) in [(false, false), (true, false), (false, true), (true, true)] {
        let (owner, _, drops) = setup(2);
        let mut reader = owner.register_record_mutator().unwrap();
        let mut writer = owner.register_record_mutator().unwrap();
        let a = record(&mut reader, 0, &drops);
        let b = record(&mut reader, 1, &drops);
        let edge = publish(&mut reader, 0, &a, &b);
        let source = a.key();
        AFTER_SOURCE_OBSERVATION.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move || {
                if revoke {
                    root_run(&mut writer, RootCommand::Remove(source));
                } else {
                    edge_run(
                        &mut writer,
                        EdgeCommand::Replace {
                            edge,
                            source,
                            target: None,
                        },
                    );
                }
            }));
        });
        assert_eq!(
            if borrowed {
                reader.observe_rooted_edge(&a, 0, &())
            } else {
                reader.observe_source_edge(source, 0, &())
            },
            Err(RequestError::Contended),
            "source observation crossed a changed controller revision"
        );
        let next = reader.observe_source_edge(source, 0, &());
        if revoke {
            assert_eq!(next, Err(RequestError::Conflict));
        } else {
            assert!(
                matches!(next.unwrap().state, EdgeMembership::Live(state) if state.target.is_none())
            );
        }
    }
}

#[test]
fn bounded_q0_edge_capacities_are_order_independent_checked_and_reversible() {
    let base = Capacities::derive(3, 2, 1).unwrap();
    let a = base
        .with_roots(4)
        .unwrap()
        .with_references(0)
        .unwrap()
        .with_edges(2)
        .unwrap();
    let b = base
        .with_edges(2)
        .unwrap()
        .with_references(0)
        .unwrap()
        .with_roots(4)
        .unwrap();
    assert_eq!(a, b);
    assert_eq!(a.edge_slots(), 2);
    assert!(a.edge_pools().0 >= 2 * 2 + 6 * a.participants() + 4);
    assert!(a.edge_pools().1 >= 2 * 2 + 20 * a.participants() + 8);
    assert_eq!(
        a.with_edges(0).unwrap(),
        base.with_roots(4).unwrap().with_references(0).unwrap()
    );
    assert_eq!(
        base.with_edges(usize::MAX),
        Err(ConfigError::CapacityOverflow)
    );
    let footprint = Footprint::for_records::<Value>(a).unwrap();
    assert_eq!(footprint.edge_tokens(), 2 * size_of::<EdgeHandle<Value>>());
    let short = Budget::new(footprint.total() - 1);
    assert!(matches!(
        BoundedCatalog::<Value>::with_records_in(a, &short),
        Err(ConfigError::BudgetExceeded)
    ));
    assert_eq!(short.used(), 0);
}

#[test]
fn bounded_edge_creation_owner_holds_quota_and_charge_until_its_drop() {
    let (owner, budget, _) = setup(1);
    let mut worker = owner.register_record_mutator().unwrap();
    let handle = edge(&mut worker, 0);
    assert_eq!(owner.reserved_edge_tokens(), 1);
    assert_eq!(worker.submit_edge_creation(0), Err(RequestError::EdgeLimit));
    let (foreign, foreign_budget, _) = setup(1);
    let mut other = foreign.register_record_mutator().unwrap();
    let Err(failure) = other.release_edge_to_source(handle, &()) else {
        panic!("foreign handoff succeeded")
    };
    assert_eq!(failure.error, RequestError::Foreign);
    let charged = owner.charged_bytes();
    drop(worker);
    drop(owner);
    assert_eq!(budget.used(), charged);
    drop(failure.edge);
    assert_eq!(budget.used(), 0);
    drop(other);
    drop(foreign);
    assert_eq!(foreign_budget.used(), 0);
}

#[test]
fn bounded_edge_cancellation_releases_every_creation_stage_and_posts_cleanup() {
    for steps in 0..=4 {
        let (owner, budget, _) = setup(1);
        let mut worker = owner.register_record_mutator().unwrap();
        worker.submit_edge_creation(0).unwrap();
        for _ in 0..steps {
            worker.poll_in(&());
        }
        assert_eq!(owner.reserved_edge_tokens(), 1);
        assert!(worker.cancel());
        assert_eq!(owner.reserved_edge_tokens(), 0);
        if let Some(observed) = worker.discover_edge(0).unwrap() {
            assert!(observed.removal_requested);
            edge_run(&mut worker, EdgeCommand::RemoveRequested(observed.key));
            assert!(worker.release_retired_edge(observed.key).unwrap());
        }
        assert!(worker.discover_edge(0).unwrap().is_none());
        drop(worker);
        drop(owner);
        assert_eq!(budget.used(), 0);
    }
}

#[test]
fn bounded_source_owned_cycle_reclaims_real_records_and_all_credits() {
    let (owner, budget, drops) = setup(2);
    let mut worker = owner.register_record_mutator().unwrap();
    let a = record(&mut worker, 0, &drops);
    let b = record(&mut worker, 1, &drops);
    assert_eq!(a.borrow_live_record().unwrap().identity, 0);
    let keys = [a.observe().target, b.observe().target];
    let edges = [
        publish(&mut worker, 0, &a, &b),
        publish(&mut worker, 1, &b, &a),
    ];
    assert_eq!(owner.reserved_edge_tokens(), 0);
    for key in keys {
        for domain in 0..2 {
            acknowledge(&mut worker, key, domain);
        }
    }
    drop_root(&mut worker, a);
    drop_root(&mut worker, b);
    for domain in 0..2 {
        end(&mut worker, domain);
    }
    for domain in 0..2 {
        run(
            &mut worker,
            GuardedCommand::Start {
                domain,
                mode: DomainMode::Full,
            },
        );
    }
    scan(&mut worker, 0);
    for _ in 0..RETIREMENT_FLOOR {
        run(&mut worker, GuardedCommand::Advance { domain: 0 });
    }
    for key in keys {
        run(&mut worker, GuardedCommand::Retire { key, domain: 0 });
        assert!(worker.release_retired(key).unwrap());
    }
    assert_eq!(
        drops.load(Ordering::Relaxed),
        2,
        "source-owned edges retained unreachable records"
    );
    for key in edges {
        assert!(!worker.observe_edge(key).unwrap().removal_requested);
        edge_run(&mut worker, EdgeCommand::RemoveOrphan(key));
        assert!(worker.release_retired_edge(key).unwrap());
    }
    assert_eq!(owner.reserved_root_tokens(), 0);
    assert_eq!(owner.reserved_edge_tokens(), 0);
    drop(worker);
    assert_eq!(owner.registered(), (0, 0));
    drop(owner);
    assert_eq!(budget.used(), 0);
    assert_eq!(Arc::strong_count(&drops), 1);
}

#[test]
fn bounded_acknowledgement_cannot_skip_a_transitive_edge() {
    let (owner, budget, drops) = setup(1);
    let mut worker = owner.register_record_mutator().unwrap();
    let a = record(&mut worker, 0, &drops);
    let b = record(&mut worker, 1, &drops);
    let ak = a.observe().target;
    let bk = b.observe().target;
    publish(&mut worker, 0, &a, &b);
    for key in [ak, bk] {
        for domain in 0..2 {
            acknowledge(&mut worker, key, domain);
        }
    }
    drop_root(&mut worker, b);
    end(&mut worker, 0);
    assert_eq!(
        worker.submit(GuardedCommand::Start {
            domain: 0,
            mode: DomainMode::Young
        }),
        Err(RequestError::Conflict)
    );
    run(
        &mut worker,
        GuardedCommand::Start {
            domain: 0,
            mode: DomainMode::Full,
        },
    );
    scan(&mut worker, 0);
    let Membership::Tracing(before) = worker.observe(bk).unwrap().membership else {
        panic!()
    };
    assert_eq!(before[0].ticket, 0);
    acknowledge(&mut worker, ak, 0);
    let Membership::Tracing(after) = worker.observe(bk).unwrap().membership else {
        panic!()
    };
    assert_ne!(
        after[0].ticket, 0,
        "bounded acknowledgement skipped graph traversal"
    );
    assert_eq!(
        after[0].retention,
        Some(crate::canonical_catalog::TraceRetention::Strong)
    );
    assert_eq!(worker.phase_in(&()).unwrap().domains[0].pending, 1);
    drop(a);
    drop(worker);
    drop(owner);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(budget.used(), 0);
}

#[cfg(not(target_family = "wasm"))]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "retain the frozen three-actor bounded mutation/read/epoch regression in one fixture"
)]
fn bounded_edge_mutation_child_reads_and_full_epochs_share_one_controller() {
    fn retry<T>(mut operation: impl FnMut() -> Result<T, RequestError>) -> T {
        for _ in 0..1_000_000 {
            match operation() {
                Ok(value) => return value,
                Err(RequestError::Contended) => std::thread::yield_now(),
                Err(error) => panic!("unexpected observation error: {error:?}"),
            }
        }
        panic!("concurrent observation exceeded fixture watchdog");
    }

    fn settle(worker: &mut Worker) -> Progress {
        for _ in 0..1_000_000 {
            let progress = worker.poll_in(&());
            if progress != Progress::Pending {
                return progress;
            }
            std::thread::yield_now();
        }
        panic!("concurrent edge request exceeded fixture watchdog");
    }

    fn phase(worker: &mut Worker) -> Phase {
        for _ in 0..1_000_000 {
            match worker.phase_in(&()) {
                Ok(phase) => return phase,
                Err(RequestError::Contended) => std::thread::yield_now(),
                other => panic!("unexpected phase observation: {other:?}"),
            }
        }
        panic!("concurrent edge phase exceeded fixture watchdog");
    }

    fn command(worker: &mut Worker, command: GuardedCommand) {
        worker.submit(command).unwrap();
        assert_eq!(settle(worker), Progress::Committed);
    }
    let (owner, budget, drops) = setup(1);
    let mut helper = owner.register_record_helper().unwrap();
    let roots = [
        record(&mut helper, 0, &drops),
        record(&mut helper, 1, &drops),
        record(&mut helper, 2, &drops),
    ];
    let records = roots.each_ref().map(|root| root.observe().target);
    let root_keys = roots.each_ref().map(RootHandle::key);
    let edge = publish(&mut helper, 0, &roots[0], &roots[1]);
    let mut writer = owner.register_record_mutator().unwrap();
    let mut reader = owner.register_record_mutator().unwrap();
    let gate = std::sync::Barrier::new(3);
    let iterations = if cfg!(miri) { 4 } else { 64 };
    std::thread::scope(|scope| {
        let written = scope.spawn(|| {
            gate.wait();
            for iteration in 0..iterations {
                writer
                    .submit_edge(EdgeCommand::Replace {
                        edge,
                        source: root_keys[0],
                        target: Some(root_keys[1 + iteration % 2]),
                    })
                    .unwrap();
                assert_eq!(settle(&mut writer), Progress::Committed);
            }
        });
        let read = scope.spawn(|| {
            gate.wait();
            let mut successes = 0;
            for _ in 0..1_000_000 {
                let observed = match reader.observe_edge(edge) {
                    Ok(observed) => observed,
                    Err(RequestError::Contended) => {
                        std::thread::yield_now();
                        continue;
                    }
                    other => panic!("unexpected edge observation: {other:?}"),
                };
                let EdgeMembership::Live(state) = observed.state else {
                    panic!("lost live edge");
                };
                let target = state.target.expect("non-null racing target");
                let index = records
                    .iter()
                    .position(|key| key.identity() == target.identity)
                    .expect("known target");
                reader.submit_root_creation(3, records[index]).unwrap();
                assert!(matches!(settle(&mut reader), Progress::RootCreated(_)));
                let child = reader.take_root().unwrap();
                reader
                    .submit_edge(EdgeCommand::RootTarget {
                        edge,
                        source: root_keys[0],
                        child: child.key(),
                    })
                    .unwrap();
                match settle(&mut reader) {
                    Progress::Committed => {
                        assert_eq!(
                            child.borrow_live_record().unwrap().identity,
                            index,
                            "edge child read returned another record identity"
                        );
                        successes += 1;
                    }
                    Progress::Rejected(RequestError::Conflict) => {}
                    other => panic!("unexpected racing read: {other:?}"),
                }
                let key = child.key();
                drop(child);
                reader
                    .submit_root(RootCommand::RemoveRequested(key))
                    .unwrap();
                let removed = settle(&mut reader);
                assert!(matches!(
                    removed,
                    Progress::Committed | Progress::Rejected(RequestError::Conflict)
                ));
                assert_eq!(
                    retry(|| reader.observe_root(key)).status,
                    RootStatus::Retired
                );
                assert!(retry(|| reader.release_retired_root(key)));
                if successes == iterations {
                    return successes;
                }
            }
            panic!("concurrent child reads exceeded fixture watchdog");
        });
        gate.wait();
        for _ in 0..3 {
            while phase(&mut helper).root_cursors[0] < 4 {
                helper
                    .submit_root(RootCommand::ScanBatch { domain: 0 })
                    .unwrap();
                assert_eq!(settle(&mut helper), Progress::Committed);
            }
            while phase(&mut helper).domains[0].pending != 0 {
                for key in records {
                    let observed = (0..1_000_000)
                        .find_map(|_| match helper.observe(key) {
                            Ok(value) => Some(value),
                            Err(RequestError::Contended) => {
                                std::thread::yield_now();
                                None
                            }
                            other => panic!("unexpected record observation: {other:?}"),
                        })
                        .expect("record observation exceeded fixture watchdog");
                    let Membership::Tracing(marks) = observed.membership else {
                        panic!()
                    };
                    if marks[0].ticket != 0 {
                        command(
                            &mut helper,
                            GuardedCommand::Acknowledge {
                                key,
                                domain: 0,
                                ticket: marks[0].ticket,
                            },
                        );
                    }
                }
            }
            for _ in 0..RETIREMENT_FLOOR {
                command(&mut helper, GuardedCommand::Advance { domain: 0 });
            }
            command(&mut helper, GuardedCommand::Finish { domain: 0 });
            command(
                &mut helper,
                GuardedCommand::Start {
                    domain: 0,
                    mode: DomainMode::Full,
                },
            );
        }
        written.join().unwrap();
        assert_eq!(read.join().unwrap(), iterations);
    });
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    drop(roots);
    drop(writer);
    drop(reader);
    drop(helper);
    assert_eq!(owner.registered(), (0, 0));
    drop(owner);
    assert_eq!(drops.load(Ordering::Relaxed), 3);
    assert_eq!(budget.used(), 0);
}

#[test]
fn bounded_indexed_layout_rejects_invalid_geometry_before_reserving_credit() {
    let capacities = Capacities::derive(3, 2, 1)
        .unwrap()
        .with_roots(4)
        .unwrap()
        .with_edges(6)
        .unwrap();
    let budget = Budget::new(0);
    for degree in [0, 1, 3, usize::MAX] {
        assert!(matches!(
            BoundedCatalog::<Value>::with_indexed_records_and_storage_in(
                capacities, &budget, 0, degree
            ),
            Err(ConfigError::InvalidEdgeLayout)
        ));
        assert_eq!(budget.used(), 0);
    }
    assert!(matches!(
        BoundedCatalog::<Value>::with_indexed_records_and_storage_in(capacities, &budget, 0, 2),
        Err(ConfigError::BudgetExceeded)
    ));
}

#[test]
fn bounded_indexed_publication_rejects_wrong_source_and_remains_usable() {
    let capacities = Capacities::derive(3, 2, 1)
        .unwrap()
        .with_roots(4)
        .unwrap()
        .with_edges(6)
        .unwrap();
    let budget = Budget::new(Footprint::for_records::<Value>(capacities).unwrap().total());
    let owner =
        BoundedCatalog::with_indexed_records_and_storage_in(capacities, &budget, 0, 2).unwrap();
    let mut worker = owner.register_record_mutator().unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let source = record(&mut worker, 1, &drops);
    let target = record(&mut worker, 2, &drops);
    let misplaced = edge(&mut worker, 0);
    worker
        .submit_edge(EdgeCommand::Publish {
            edge: misplaced.key(),
            source: source.key(),
            target: Some(target.key()),
        })
        .unwrap();
    assert_eq!(
        finish(&mut worker),
        Progress::Rejected(RequestError::InvalidSlot),
        "bounded indexed publication did not reject wrong placement"
    );
    assert_eq!(
        worker.observe_edge(misplaced.key()).unwrap().state,
        EdgeMembership::Prepared
    );
    publish(&mut worker, 3, &source, &target);
    drop(misplaced);
    drop(source);
    drop(target);
    drop(worker);
    drop(owner);
    assert_eq!(drops.load(Ordering::Relaxed), 2);
    assert_eq!(budget.used(), 0);
}
