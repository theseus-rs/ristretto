//! Reference/construction/notification allocator audit, separate from timing.
use ristretto_catalog_tagged_prototype::{
    DomainMode, Entered, Membership, RETIREMENT_FLOOR, Record, RecordError, ReferenceMembership,
    ReferenceRootKind, audit,
    bounded::{
        BoundedCatalog, Budget, Capacities, ConfigError, ConstructionHandle, Footprint,
        GuardedCommand as Command, Participant, Progress, RecordKey, ReferenceCommand,
        ReferenceHandle, RootCommand, RootHandle,
    },
    reference::ReferenceKind,
};
#[path = "support/counting.rs"]
mod counting;
struct Inline<const N: usize>([u8; N]);
impl<const N: usize> Default for Inline<N> {
    fn default() -> Self {
        Self([0; N])
    }
}
impl<const N: usize> Record for Inline<N> {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, context: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        assert_eq!(self.0[0], 0);
        ().enter(context)
    }
}
#[repr(align(64))]
#[derive(Default)]
struct Aligned(Inline<65>);
impl Record for Aligned {
    type Context = ();
    type Guard<'a> = ();
    fn enter<'a>(&'a self, context: &'a ()) -> Result<Entered<'a, ()>, RecordError> {
        self.0.enter(context)
    }
}

type Worker<R> = Participant<R, Command>;
fn finish<R: Record<Context = ()>>(worker: &mut Worker<R>) -> Progress {
    for _ in 0..16384 {
        let result = worker.poll_in(&());
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("reference audit did not settle");
}
fn run<R: Record<Context = ()>>(worker: &mut Worker<R>, command: Command) {
    worker.submit(command).unwrap();
    assert_eq!(finish(worker), Progress::Committed);
}
fn roots<R: Record<Context = ()>>(worker: &mut Worker<R>, command: RootCommand) {
    worker.submit_root(command).unwrap();
    assert_eq!(finish(worker), Progress::Committed);
}
fn refs<R: Record<Context = ()>>(worker: &mut Worker<R>, command: ReferenceCommand) {
    worker.submit_reference(command).unwrap();
    assert_eq!(finish(worker), Progress::Committed);
}
fn create<R: Record<Context = ()> + Default>(worker: &mut Worker<R>, slot: usize) -> RecordKey {
    worker
        .submit_record(slot, 8 * (slot + 1), R::default())
        .unwrap();
    let Progress::Created(key) = finish(worker) else {
        panic!("record creation")
    };
    key
}
fn root<R: Record<Context = ()>>(
    worker: &mut Worker<R>,
    slot: usize,
    target: RecordKey,
) -> RootHandle<R> {
    worker.submit_root_creation(slot, target).unwrap();
    assert!(matches!(finish(worker), Progress::RootCreated(_)));
    worker.take_root().unwrap()
}
fn reference<R: Record<Context = ()>>(worker: &mut Worker<R>) -> ReferenceHandle<R> {
    worker.submit_reference_creation(0).unwrap();
    assert!(matches!(finish(worker), Progress::ReferenceCreated(_)));
    worker.take_reference().unwrap()
}
fn release<R: Record<Context = ()>>(worker: &mut Worker<R>, root: RootHandle<R>) {
    let key = root.key();
    drop(root);
    roots(worker, RootCommand::RemoveRequested(key));
    assert!(worker.release_retired_root(key).unwrap());
}
fn finish_domain<R: Record<Context = ()>>(
    worker: &mut Worker<R>,
    keys: &[RecordKey],
    domain: usize,
    capacities: Capacities,
) {
    if worker.phase_in(&()).unwrap().domains[domain].mode == DomainMode::Inactive {
        return;
    }
    while worker.phase_in(&()).unwrap().domains[domain].floor < RETIREMENT_FLOOR {
        if worker.phase_in(&()).unwrap().domains[domain].floor == 0 {
            while worker.phase_in(&()).unwrap().root_cursors[domain]
                < capacities.root_slots() + 2 * capacities.reference_slots()
            {
                roots(worker, RootCommand::Scan { domain });
            }
        } else {
            while worker.phase_in(&()).unwrap().reference_cursors[domain]
                < capacities.reference_slots()
            {
                refs(worker, ReferenceCommand::Scan { domain });
            }
        }
        for key in keys {
            let Membership::Tracing(marks) = worker.observe(*key).unwrap().membership else {
                panic!("live audit object")
            };
            for d in 0..2 {
                if marks[d].ticket != 0 {
                    run(
                        worker,
                        Command::Acknowledge {
                            key: *key,
                            domain: d,
                            ticket: marks[d].ticket,
                        },
                    );
                }
            }
        }
        run(worker, Command::Advance { domain });
    }
}
fn cycle<R: Record<Context = ()> + Default>(worker: &mut Worker<R>, capacities: Capacities) {
    let target = create(worker, 0);
    let source = root(worker, 0, target);
    roots(worker, RootCommand::Publish(source.key()));
    let reference = reference(worker);
    refs(
        worker,
        ReferenceCommand::Publish {
            reference: reference.key(),
            source: source.key(),
            kind: ReferenceKind::Final,
        },
    );
    worker
        .submit_reference_reservation(reference.key())
        .unwrap();
    assert!(matches!(finish(worker), Progress::ConstructionCreated(_)));
    let construction = worker.take_construction().unwrap();
    let owner = create(worker, 1);
    let owner_root = root(worker, 1, owner);
    refs(
        worker,
        ReferenceCommand::PublishOwner {
            reference: reference.key(),
            owner: owner_root.key(),
        },
    );
    assert!(worker.release_retired_root(construction.key()).unwrap());
    release(worker, source);
    let destination = root(worker, 0, owner);
    for domain in 0..2 {
        if worker.phase_in(&()).unwrap().domains[domain].mode != DomainMode::Inactive {
            finish_domain(worker, &[target, owner], domain, capacities);
            run(worker, Command::Finish { domain });
        }
    }
    run(
        worker,
        Command::Start {
            domain: 0,
            mode: DomainMode::Full,
        },
    );
    finish_domain(worker, &[target, owner], 0, capacities);
    let ReferenceMembership::Live(state) = worker.observe_reference(reference.key()).unwrap().state
    else {
        panic!("live reference")
    };
    assert!(state.activated && state.queued_root.is_some());
    let held = worker
        .discover_reference_root(reference.key(), ReferenceRootKind::Notification)
        .unwrap()
        .unwrap();
    refs(
        worker,
        ReferenceCommand::Take {
            reference: reference.key(),
            root: destination.key(),
        },
    );
    assert!(worker.release_retired_root(held.key).unwrap());
    refs(worker, ReferenceCommand::Clear(reference.key()));
    let key = reference.key();
    drop(reference);
    refs(worker, ReferenceCommand::Retire(key));
    assert!(worker.release_retired_reference(key).unwrap());
    drop(construction);
    release(worker, owner_root);
    release(worker, destination);
    run(worker, Command::Finish { domain: 0 });
    run(
        worker,
        Command::Start {
            domain: 0,
            mode: DomainMode::Full,
        },
    );
    finish_domain(worker, &[target, owner], 0, capacities);
    for key in [target, owner] {
        run(worker, Command::Retire { key, domain: 0 });
        assert!(worker.release_retired(key).unwrap());
    }
    run(worker, Command::Finish { domain: 0 });
    run(
        worker,
        Command::Start {
            domain: 0,
            mode: DomainMode::Full,
        },
    );
}
fn retained_handles<R: Record<Context = ()> + Default>(
    worker: &mut Worker<R>,
) -> (ReferenceHandle<R>, ConstructionHandle<R>) {
    let target = create(worker, 0);
    let source = root(worker, 0, target);
    roots(worker, RootCommand::Publish(source.key()));
    let reference = reference(worker);
    refs(
        worker,
        ReferenceCommand::Publish {
            reference: reference.key(),
            source: source.key(),
            kind: ReferenceKind::Weak,
        },
    );
    worker
        .submit_reference_reservation(reference.key())
        .unwrap();
    assert!(matches!(finish(worker), Progress::ConstructionCreated(_)));
    let construction = worker.take_construction().unwrap();
    release(worker, source);
    (reference, construction)
}
fn audit_record<R: Record<Context = ()> + Default>(name: &str) {
    for registry in [2, 64, 1024, 16384] {
        for mutators in [0, 1, 7] {
            for (roots, references) in [(2, 1), (2, 64), (64, 1), (64, 64)] {
                let capacities = Capacities::derive(registry, mutators, 1)
                    .unwrap()
                    .with_roots(roots)
                    .unwrap()
                    .with_references(references)
                    .unwrap();
                let footprint = Footprint::for_records::<R>(capacities).unwrap();
                let budget = Budget::new(footprint.total());
                let before = audit::snapshot();
                let catalog = BoundedCatalog::<R>::with_records_in(capacities, &budget).unwrap();
                let after = audit::snapshot();
                assert_eq!(
                    after.requested - before.requested,
                    footprint.backing() as u64
                );
                assert_eq!(after.frees, before.frees);
                let mut helper = catalog.register_record_helper().unwrap();
                cycle(&mut helper, capacities);
                cycle(&mut helper, capacities);
                let (reference, construction) = retained_handles(&mut helper);
                let held = audit::snapshot();
                assert_eq!(
                    held.requests, after.requests,
                    "reference lifecycle allocated"
                );
                assert_eq!(held.frees, after.frees, "lifecycle released backing");
                drop(catalog);
                drop(helper);
                assert_eq!(budget.used(), footprint.total());
                // Both unique handle kinds independently retain the charge.
                drop(reference);
                assert_eq!(budget.used(), footprint.total());
                assert_eq!(audit::snapshot().frees, after.frees);
                drop(construction);
                let freed = audit::snapshot();
                assert_eq!(budget.used(), 0);
                assert_eq!(freed.refunds - before.refunds, 1);
                assert_eq!(freed.refunded - before.refunded, footprint.total() as u64);
                assert_eq!(freed.released - before.released, footprint.backing() as u64);
                assert_eq!(freed.frees - before.frees, after.requests - before.requests);
                assert!(freed.last_refund > freed.last_free);
                println!(
                    "{name},{registry},{mutators},1,{roots},{references},{},{},{},{},{},{},{},0,true",
                    footprint.backing(),
                    footprint.participants(),
                    footprint.root_tokens(),
                    footprint.reference_tokens(),
                    footprint.construction_tokens(),
                    footprint.total(),
                    after.requests - before.requests
                );
                let short = Budget::new(footprint.total() - 1);
                let before = audit::snapshot();
                assert!(matches!(
                    BoundedCatalog::<R>::with_records_in(capacities, &short),
                    Err(ConfigError::BudgetExceeded)
                ));
                assert_eq!(audit::snapshot().requests, before.requests);
                assert_eq!(short.used(), 0);
            }
        }
    }
}
fn main() {
    println!(
        "record,registry,mutators,helpers,roots,references,backing,participants,root_tokens,reference_tokens,construction_tokens,total,requests,lifecycle_requests,refund_after_last_free"
    );
    audit_record::<Inline<1>>("byte");
    audit_record::<Inline<512>>("large");
    audit_record::<Aligned>("aligned");
}
