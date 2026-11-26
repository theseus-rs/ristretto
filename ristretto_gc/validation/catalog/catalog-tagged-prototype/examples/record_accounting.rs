//! Opaque-record allocator/credit audit, separate from timing.
use ristretto_catalog_tagged_prototype::{
    Entered, Membership, RETIREMENT_FLOOR, Record, RecordError, audit,
    bounded::{
        BoundedCatalog, Budget, Capacities, ConfigError, Footprint, GuardedCommand as Command,
        Participant, Progress,
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
fn finish<R: Record<Context = ()>>(p: &mut Participant<R, Command>) -> Progress {
    for _ in 0..4096 {
        let result = p.poll_in(&());
        if result != Progress::Pending {
            return result;
        }
    }
    panic!("record audit request did not settle");
}
fn run<R: Record<Context = ()>>(p: &mut Participant<R, Command>, command: Command) {
    p.submit(command).unwrap();
    assert_eq!(finish(p), Progress::Committed);
}
fn cycle<R: Record<Context = ()> + Default>(p: &mut Participant<R, Command>) {
    p.submit_record(0, 8, R::default()).unwrap();
    let Progress::Created(key) = finish(p) else {
        panic!("creation failed")
    };
    run(p, Command::Publish(key));
    let Membership::Tracing(marks) = p.observe(key).unwrap().membership else {
        panic!("publication failed")
    };
    for domain in 0..2 {
        run(
            p,
            Command::Acknowledge {
                key,
                domain,
                ticket: marks[domain].ticket,
            },
        );
    }
    for _ in 0..RETIREMENT_FLOOR {
        run(p, Command::Advance { domain: 0 });
    }
    run(p, Command::Finish { domain: 0 });
    run(
        p,
        Command::Start {
            domain: 0,
            mode: ristretto_catalog_tagged_prototype::DomainMode::Full,
        },
    );
    for _ in 0..RETIREMENT_FLOOR {
        run(p, Command::Advance { domain: 0 });
    }
    run(p, Command::Retire { key, domain: 0 });
    assert!(p.release_retired(key).unwrap());
    assert!(p.discover(0).unwrap().is_none());
}
fn audit_record<R: Record<Context = ()> + Default>(name: &str) {
    for registry in [1, 64, 1024, 16384] {
        for mutators in [0, 1, 7] {
            let capacities = Capacities::derive(registry, mutators, 1).unwrap();
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
            cycle(&mut helper);
            let held = audit::snapshot();
            assert_eq!(held.requests, after.requests, "record lifecycle allocated");
            assert_eq!(held.frees, after.frees, "record lifecycle released backing");
            drop(catalog);
            assert_eq!(budget.used(), footprint.total());
            drop(helper);
            let freed = audit::snapshot();
            assert_eq!(budget.used(), 0);
            assert_eq!(freed.refunds - before.refunds, 1);
            assert_eq!(freed.refunded - before.refunded, footprint.total() as u64);
            assert_eq!(freed.released - before.released, footprint.backing() as u64);
            assert_eq!(freed.frees - before.frees, after.requests - before.requests);
            assert!(freed.last_refund > freed.last_free);
            println!(
                "{name},{registry},{mutators},1,{},{},{},{},0,true",
                footprint.backing(),
                footprint.participants(),
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
fn main() {
    println!(
        "record,registry,mutators,helpers,backing,participants,total,requests,lifecycle_requests,refund_after_last_free"
    );
    audit_record::<Inline<1>>("byte");
    audit_record::<Inline<512>>("large");
    audit_record::<Aligned>("aligned");
}
