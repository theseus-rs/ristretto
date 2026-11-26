//! Requested-backing and lifetime accounting; deliberately separate from timing.
use ristretto_catalog_tagged_prototype::{
    DomainMode, Entered, Membership, RETIREMENT_FLOOR, Record, RecordError, audit,
    bounded::{
        BoundedCatalog, Budget, Capacities, EdgeCommand, EdgeHandle, Footprint,
        GuardedCommand as Command, Participant, Progress, RecordKey, RootCommand, RootHandle,
    },
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
fn finish<R: Record<Context = ()>>(p: &mut Worker<R>) -> Progress {
    for _ in 0..16384 {
        let result = p.poll_in(&());
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("edge accounting request did not settle");
}
fn run<R: Record<Context = ()>>(p: &mut Worker<R>, command: Command) {
    p.submit(command).unwrap();
    assert_eq!(finish(p), Progress::Committed);
}
fn roots<R: Record<Context = ()>>(p: &mut Worker<R>, command: RootCommand) {
    p.submit_root(command).unwrap();
    assert_eq!(finish(p), Progress::Committed);
}
fn edges<R: Record<Context = ()>>(p: &mut Worker<R>, command: EdgeCommand) {
    p.submit_edge(command).unwrap();
    assert_eq!(finish(p), Progress::Committed);
}
fn record<R: Record<Context = ()> + Default>(p: &mut Worker<R>, slot: usize) -> RootHandle<R> {
    assert!(
        p.submit_rooted_record(slot, slot, slot + 8, R::default())
            .is_ok()
    );
    assert!(matches!(finish(p), Progress::RootCreated(_)));
    p.take_root().unwrap()
}
fn edge<R: Record<Context = ()>>(p: &mut Worker<R>, slot: usize) -> EdgeHandle<R> {
    p.submit_edge_creation(slot).unwrap();
    assert!(matches!(finish(p), Progress::EdgeCreated(_)));
    p.take_edge().unwrap()
}
fn scan<R: Record<Context = ()>>(p: &mut Worker<R>, domain: usize, capacity: usize) {
    while p.phase_in(&()).unwrap().root_cursors[domain] < capacity {
        roots(p, RootCommand::ScanBatch { domain });
    }
}
fn acknowledge<R: Record<Context = ()>>(p: &mut Worker<R>, key: RecordKey, domain: usize) {
    let Membership::Tracing(marks) = p.observe(key).unwrap().membership else {
        panic!()
    };
    if marks[domain].ticket != 0 {
        run(
            p,
            Command::Acknowledge {
                key,
                domain,
                ticket: marks[domain].ticket,
            },
        );
    }
}
fn close_floors<R: Record<Context = ()>>(p: &mut Worker<R>, domain: usize, references: usize) {
    for _ in 0..RETIREMENT_FLOOR {
        let floor = p.phase_in(&()).unwrap().domains[domain].floor;
        if (1..RETIREMENT_FLOOR).contains(&floor) {
            while p.phase_in(&()).unwrap().reference_cursors[domain] < references {
                p.submit_reference(
                    ristretto_catalog_tagged_prototype::bounded::ReferenceCommand::Scan { domain },
                )
                .unwrap();
                assert_eq!(finish(p), Progress::Committed);
            }
        }
        run(p, Command::Advance { domain });
    }
}
fn cycle<R: Record<Context = ()> + Default>(p: &mut Worker<R>, capacity: Capacities) {
    let a = record(p, 0);
    let b = record(p, 1);
    let records = [a.observe().target, b.observe().target];
    let mut inventory = [None; 2];
    for (slot, (source, target)) in [(&a, &b), (&b, &a)]
        .into_iter()
        .enumerate()
        .take(capacity.edge_slots())
    {
        let edge = edge(p, slot);
        let key = edge.key();
        edges(
            p,
            EdgeCommand::Publish {
                edge: key,
                source: source.key(),
                target: Some(target.key()),
            },
        );
        assert!(p.release_edge_to_source(edge, &()).is_ok());
        inventory[slot] = Some(key);
    }
    for record in records {
        for domain in 0..2 {
            acknowledge(p, record, domain);
        }
    }
    for root in [a, b] {
        let key = root.key();
        drop(root);
        roots(p, RootCommand::RemoveRequested(key));
        assert!(p.release_retired_root(key).unwrap());
    }
    // References are absent, but their reserved construction/notification slots
    // are part of the complete root scan when Q is nonzero.
    let root_capacity = capacity.root_slots() + 2 * capacity.reference_slots();
    for domain in 0..2 {
        scan(p, domain, root_capacity);
        close_floors(p, domain, capacity.reference_slots());
        run(p, Command::Finish { domain });
    }
    for domain in 0..2 {
        run(
            p,
            Command::Start {
                domain,
                mode: DomainMode::Full,
            },
        );
    }
    scan(p, 0, root_capacity);
    close_floors(p, 0, capacity.reference_slots());
    for key in records {
        run(p, Command::Retire { key, domain: 0 });
        assert!(p.release_retired(key).unwrap());
    }
    for key in inventory.into_iter().flatten() {
        edges(p, EdgeCommand::RemoveOrphan(key));
        assert!(p.release_retired_edge(key).unwrap());
    }
}
fn audit_record<R: Record<Context = ()> + Default>(name: &str) {
    for registry in [2, 64, 1024] {
        for mutators in [0, 7] {
            for edge_slots in [0, 1, 64] {
                for references in [0, 1] {
                    let capacities = Capacities::derive(registry, mutators, 1)
                        .unwrap()
                        .with_roots(2)
                        .unwrap()
                        .with_references(references)
                        .unwrap()
                        .with_edges(edge_slots)
                        .unwrap();
                    let footprint = Footprint::for_records::<R>(capacities).unwrap();
                    let budget = Budget::new(footprint.total());
                    let before = audit::snapshot();
                    let catalog =
                        BoundedCatalog::<R>::with_records_in(capacities, &budget).unwrap();
                    let after = audit::snapshot();
                    assert_eq!(
                        after.requested - before.requested,
                        footprint.backing() as u64
                    );
                    assert_eq!(after.frees, before.frees);
                    let mut helper = catalog.register_record_helper().unwrap();
                    cycle(&mut helper, capacities);
                    let root = record(&mut helper, 0);
                    let tail = (edge_slots != 0).then(|| edge(&mut helper, 0));
                    let held = audit::snapshot();
                    assert_eq!(
                        held.requests, after.requests,
                        "edge lifecycle allocated backing"
                    );
                    assert_eq!(held.frees, after.frees, "edge lifecycle freed backing");
                    drop(catalog);
                    drop(helper);
                    assert_eq!(budget.used(), footprint.total());
                    drop(root);
                    if tail.is_some() {
                        assert_eq!(budget.used(), footprint.total());
                    }
                    drop(tail);
                    let freed = audit::snapshot();
                    assert_eq!(budget.used(), 0);
                    assert_eq!(freed.refunds - before.refunds, 1);
                    assert_eq!(freed.refunded - before.refunded, footprint.total() as u64);
                    assert_eq!(freed.released - before.released, footprint.backing() as u64);
                    assert_eq!(freed.frees - before.frees, after.requests - before.requests);
                    assert!(freed.last_refund > freed.last_free);
                    println!(
                        "{name},{registry},{mutators},1,2,{references},{edge_slots},{},{},{},{},{},{},{},{},0,true",
                        footprint.backing(),
                        footprint.participants(),
                        footprint.root_tokens(),
                        footprint.edge_tokens(),
                        footprint.reference_tokens(),
                        footprint.construction_tokens(),
                        footprint.total(),
                        after.requests - before.requests
                    );
                }
            }
        }
    }
}
fn main() {
    println!(
        "record,registry,mutators,helpers,roots,references,edges,backing,participants,root_tokens,edge_tokens,reference_tokens,construction_tokens,total,requests,lifecycle_requests,refund_after_last_free"
    );
    audit_record::<Inline<1>>("byte");
    audit_record::<Inline<512>>("large");
    audit_record::<Aligned>("aligned");
}
